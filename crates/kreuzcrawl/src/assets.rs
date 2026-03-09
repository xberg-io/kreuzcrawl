//! Asset discovery and downloading from HTML pages.

use std::collections::HashSet;

use scraper::{Html, Selector};
use sha2::{Digest, Sha256};
use url::Url;

use crate::types::{AssetCategory, CrawlConfig, DownloadedAsset};

/// A reference to an asset discovered in an HTML page.
pub(crate) struct AssetRef {
    url: String,
    category: AssetCategory,
    html_tag: String,
}

/// Discover downloadable assets from a parsed HTML document.
pub(crate) fn discover_assets(doc: &Html, base_url: &Url) -> Vec<AssetRef> {
    static SEL_LINK_CSS: std::sync::LazyLock<Selector> =
        std::sync::LazyLock::new(|| Selector::parse("link[rel='stylesheet'][href]").unwrap());
    static SEL_SCRIPT_SRC: std::sync::LazyLock<Selector> =
        std::sync::LazyLock::new(|| Selector::parse("script[src]").unwrap());
    static SEL_IMG_SRC: std::sync::LazyLock<Selector> =
        std::sync::LazyLock::new(|| Selector::parse("img[src]").unwrap());

    let mut assets = Vec::new();

    // CSS stylesheets
    for el in doc.select(&SEL_LINK_CSS) {
        if let Some(href) = el.value().attr("href")
            && let Ok(url) = base_url.join(href)
        {
            assets.push(AssetRef {
                url: url.to_string(),
                category: AssetCategory::Stylesheet,
                html_tag: "link".to_owned(),
            });
        }
    }

    // JavaScript files
    for el in doc.select(&SEL_SCRIPT_SRC) {
        if let Some(src) = el.value().attr("src")
            && let Ok(url) = base_url.join(src)
        {
            assets.push(AssetRef {
                url: url.to_string(),
                category: AssetCategory::Script,
                html_tag: "script".to_owned(),
            });
        }
    }

    // Images
    for el in doc.select(&SEL_IMG_SRC) {
        if let Some(src) = el.value().attr("src")
            && !src.starts_with("data:")
            && let Ok(url) = base_url.join(src)
        {
            assets.push(AssetRef {
                url: url.to_string(),
                category: AssetCategory::Image,
                html_tag: "img".to_owned(),
            });
        }
    }

    assets
}

/// Download discovered assets, applying config filters.
pub(crate) async fn download_assets(
    refs: Vec<AssetRef>,
    config: &CrawlConfig,
    client: &reqwest::Client,
) -> Vec<DownloadedAsset> {
    let mut downloaded = Vec::new();
    let mut seen_urls: HashSet<String> = HashSet::new();

    for asset_ref in refs {
        // Dedup by URL
        if !seen_urls.insert(asset_ref.url.clone()) {
            continue;
        }

        // Filter by asset type
        if let Some(ref types) = config.asset_types
            && !types.contains(&asset_ref.category)
        {
            continue;
        }

        // Download
        let resp = match client.get(&asset_ref.url).send().await {
            Ok(r) => r,
            Err(_) => continue,
        };

        if !resp.status().is_success() {
            continue;
        }

        let mime_type = resp
            .headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .map(String::from);

        let bytes = match resp.bytes().await {
            Ok(b) => b,
            Err(_) => continue,
        };

        // Check max_asset_size
        if let Some(max_size) = config.max_asset_size
            && bytes.len() > max_size
        {
            continue;
        }

        // Compute SHA-256 hash
        let mut hasher = Sha256::new();
        hasher.update(&bytes);
        let hash = format!("{:x}", hasher.finalize());

        downloaded.push(DownloadedAsset {
            url: asset_ref.url,
            content_hash: hash,
            mime_type,
            size: bytes.len(),
            asset_category: asset_ref.category,
            html_tag: Some(asset_ref.html_tag),
        });
    }

    downloaded
}

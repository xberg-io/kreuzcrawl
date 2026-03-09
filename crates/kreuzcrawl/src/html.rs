//! HTML parsing helpers for metadata extraction, link discovery, and content processing.

use std::sync::LazyLock;

use regex::Regex;
use scraper::{Html, Selector};
use url::Url;

use crate::types::{FeedInfo, FeedType, ImageInfo, JsonLdEntry, LinkInfo, PageMetadata};

/// Document file extensions used for link classification.
static DOCUMENT_EXTENSIONS: &[&str] = &[
    ".pdf", ".doc", ".docx", ".xls", ".xlsx", ".ppt", ".pptx", ".odt", ".ods", ".odp", ".rtf",
    ".csv", ".txt", ".zip", ".tar", ".gz", ".rar",
];

/// Binary file extensions used to detect non-HTML content.
static BINARY_EXTENSIONS: &[&str] = &[
    ".jpg", ".jpeg", ".png", ".gif", ".bmp", ".webp", ".svg", ".ico", ".tiff", ".mp4", ".avi",
    ".mov", ".wmv", ".flv", ".mkv", ".webm", ".mp3", ".wav", ".ogg", ".flac", ".aac", ".wma",
    ".exe", ".dll", ".so", ".bin",
];

// --- LazyLock regex patterns for metadata extraction ---

static META_RE_NAME_CONTENT: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r#"<meta\s+[^>]*name\s*=\s*["']([^"']+)["'][^>]*content\s*=\s*["']([^"']+)["'][^>]*>"#,
    )
    .unwrap()
});
static META_RE_CONTENT_NAME: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r#"<meta\s+[^>]*content\s*=\s*["']([^"']+)["'][^>]*name\s*=\s*["']([^"']+)["'][^>]*>"#,
    )
    .unwrap()
});

// --- LazyLock CSS selectors ---

static SEL_META: LazyLock<Selector> = LazyLock::new(|| Selector::parse("meta").unwrap());
static SEL_TITLE: LazyLock<Selector> = LazyLock::new(|| Selector::parse("title").unwrap());
static SEL_CANONICAL: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("link[rel='canonical']").unwrap());
static SEL_ROBOTS_META: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("meta[name='robots']").unwrap());
static SEL_META_REFRESH: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("meta[http-equiv='refresh']").unwrap());
static SEL_A_HREF: LazyLock<Selector> = LazyLock::new(|| Selector::parse("a[href]").unwrap());
static SEL_BASE_HREF: LazyLock<Selector> = LazyLock::new(|| Selector::parse("base[href]").unwrap());
static SEL_IMG_SRC: LazyLock<Selector> = LazyLock::new(|| Selector::parse("img[src]").unwrap());
static SEL_SOURCE_SRCSET: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("source[srcset]").unwrap());
static SEL_OG_IMAGE: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("meta[property='og:image']").unwrap());
static SEL_TWITTER_IMAGE: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("meta[name='twitter:image']").unwrap());
static SEL_FEED_ALTERNATE: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("link[rel='alternate']").unwrap());
static SEL_JSON_LD: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("script[type='application/ld+json']").unwrap());
static SEL_MAIN_CONTENT: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("main, article, [role='main']").unwrap());

/// Extract metadata name-value pairs from raw HTML using regex (fallback for malformed HTML).
pub(crate) fn extract_metadata_from_raw(body: &str) -> Vec<(String, String)> {
    let mut results = Vec::new();
    for cap in META_RE_NAME_CONTENT.captures_iter(body) {
        results.push((cap[1].to_lowercase(), cap[2].to_owned()));
    }
    for cap in META_RE_CONTENT_NAME.captures_iter(body) {
        results.push((cap[2].to_lowercase(), cap[1].to_owned()));
    }
    results
}

/// Extract metadata from a parsed HTML document, with regex fallback for malformed content.
pub(crate) fn extract_metadata(doc: &Html, raw_body: &str) -> PageMetadata {
    let title = doc
        .select(&SEL_TITLE)
        .next()
        .map(|el| el.text().collect::<String>());
    let canonical_url = doc
        .select(&SEL_CANONICAL)
        .next()
        .and_then(|el| el.value().attr("href").map(String::from));

    let mut md = PageMetadata {
        title,
        canonical_url,
        ..Default::default()
    };

    for meta in doc.select(&SEL_META) {
        let el = meta.value();
        let name = el
            .attr("name")
            .or_else(|| el.attr("property"))
            .unwrap_or("");
        let content = el.attr("content").unwrap_or("").to_owned();
        if content.is_empty() {
            continue;
        }

        let name_lower = name.to_lowercase();
        match name_lower.as_str() {
            "description" => md.description = Some(content),
            "og:title" => md.og_title = Some(content),
            "og:type" => md.og_type = Some(content),
            "og:image" => md.og_image = Some(content),
            "og:description" => md.og_description = Some(content),
            "og:url" => md.og_url = Some(content),
            "og:site_name" => md.og_site_name = Some(content),
            "og:locale" => md.og_locale = Some(content),
            "twitter:card" => md.twitter_card = Some(content),
            "twitter:title" => md.twitter_title = Some(content),
            "twitter:description" => md.twitter_description = Some(content),
            "twitter:image" => md.twitter_image = Some(content),
            "twitter:site" => md.twitter_site = Some(content),
            "twitter:creator" => md.twitter_creator = Some(content),
            "dc.title" => md.dc_title = Some(content),
            "dc.creator" => md.dc_creator = Some(content),
            "dc.subject" => md.dc_subject = Some(content),
            "dc.description" => md.dc_description = Some(content),
            "dc.publisher" => md.dc_publisher = Some(content),
            "dc.date" => md.dc_date = Some(content),
            "dc.type" => md.dc_type = Some(content),
            "dc.format" => md.dc_format = Some(content),
            "dc.identifier" => md.dc_identifier = Some(content),
            "dc.language" => md.dc_language = Some(content),
            "dc.rights" => md.dc_rights = Some(content),
            _ => {}
        }
    }

    // Regex-based fallback for malformed HTML where DOM parsing misses meta tags
    if !raw_body.is_empty() {
        let raw_meta = extract_metadata_from_raw(raw_body);
        for (name, content) in raw_meta {
            match name.as_str() {
                "description" if md.description.is_none() => md.description = Some(content),
                "og:title" if md.og_title.is_none() => md.og_title = Some(content),
                "og:description" if md.og_description.is_none() => {
                    md.og_description = Some(content);
                }
                "twitter:title" if md.twitter_title.is_none() => {
                    md.twitter_title = Some(content);
                }
                "twitter:description" if md.twitter_description.is_none() => {
                    md.twitter_description = Some(content);
                }
                _ => {}
            }
        }
    }

    md
}

/// Check whether a meta robots directive contains the given keyword (e.g., "noindex" or "nofollow").
fn has_robots_directive(doc: &Html, directive: &str) -> bool {
    for el in doc.select(&SEL_ROBOTS_META) {
        if let Some(content) = el.value().attr("content")
            && content.to_lowercase().contains(directive)
        {
            return true;
        }
    }
    false
}

/// Detect whether a page has a `noindex` robots directive in its meta tags.
pub(crate) fn detect_noindex(doc: &Html) -> bool {
    has_robots_directive(doc, "noindex")
}

/// Detect whether a page has a `nofollow` robots directive in its meta tags.
pub(crate) fn detect_nofollow(doc: &Html) -> bool {
    has_robots_directive(doc, "nofollow")
}

/// Detect a `<meta http-equiv="refresh">` tag and return the redirect target URL.
pub(crate) fn detect_meta_refresh(doc: &Html) -> Option<String> {
    for el in doc.select(&SEL_META_REFRESH) {
        if let Some(content) = el.value().attr("content") {
            // Format: "N;url=TARGET" or "N; url=TARGET"
            if let Some(pos) = content.to_lowercase().find("url=") {
                let target = content[pos + 4..].trim().to_owned();
                if !target.is_empty() {
                    return Some(target);
                }
            }
        }
    }
    None
}

/// Check whether content appears to be HTML based on Content-Type header or body content.
pub(crate) fn is_html_content(content_type: &str, body: &str) -> bool {
    content_type.contains("html") || body.trim_start().starts_with('<')
}

/// Classify a link as "internal", "external", "anchor", or "document".
pub(crate) fn classify_link(href: &str, base_url: &Url) -> String {
    if href.starts_with('#') {
        return "anchor".to_owned();
    }

    // Check for document extensions
    let lower = href.to_lowercase();
    for ext in DOCUMENT_EXTENSIONS {
        if lower.ends_with(ext) {
            return "document".to_owned();
        }
    }

    // Try resolving
    if let Ok(resolved) = base_url.join(href) {
        if resolved.host_str() != base_url.host_str() {
            return "external".to_owned();
        }
        "internal".to_owned()
    } else if href.starts_with("http://") || href.starts_with("https://") {
        if let Ok(u) = Url::parse(href)
            && u.host_str() != base_url.host_str()
        {
            return "external".to_owned();
        }
        "internal".to_owned()
    } else {
        "internal".to_owned()
    }
}

/// Extract all links from a parsed HTML document.
pub(crate) fn extract_links(doc: &Html, base_url: &Url) -> Vec<LinkInfo> {
    // Check for <base> tag
    let effective_base = doc
        .select(&SEL_BASE_HREF)
        .next()
        .and_then(|el| el.value().attr("href"))
        .and_then(|href| Url::parse(href).ok())
        .unwrap_or_else(|| base_url.clone());

    let mut links = Vec::new();
    for el in doc.select(&SEL_A_HREF) {
        let href = el.value().attr("href").unwrap_or("").trim();
        if href.is_empty() {
            continue;
        }

        // Skip non-HTTP schemes
        if href.starts_with("mailto:")
            || href.starts_with("javascript:")
            || href.starts_with("tel:")
            || href.starts_with("data:")
        {
            continue;
        }

        // Protocol-relative URLs: keep as-is
        let link_type = if href.starts_with("//") {
            "external".to_owned()
        } else {
            classify_link(href, &effective_base)
        };

        let resolved_url = if href.starts_with("//") {
            href.to_owned()
        } else if let Ok(u) = effective_base.join(href) {
            u.to_string()
        } else {
            href.to_owned()
        };

        let rel = el.value().attr("rel").map(String::from);
        let nofollow = rel
            .as_ref()
            .map(|r| r.contains("nofollow"))
            .unwrap_or(false);
        let text = el.text().collect::<String>().trim().to_owned();

        links.push(LinkInfo {
            url: resolved_url,
            text,
            link_type,
            rel,
            nofollow,
        });
    }
    links
}

/// Extract all images from a parsed HTML document (img, picture source, og:image, twitter:image).
pub(crate) fn extract_images(doc: &Html, _base_url: &Url) -> Vec<ImageInfo> {
    let mut images = Vec::new();

    for el in doc.select(&SEL_IMG_SRC) {
        let src = el.value().attr("src").unwrap_or("").to_owned();
        if src.is_empty() || src.starts_with("data:") {
            continue;
        }
        let alt = el.value().attr("alt").map(String::from);
        let width = el.value().attr("width").and_then(|w| w.parse::<u32>().ok());
        let height = el
            .value()
            .attr("height")
            .and_then(|h| h.parse::<u32>().ok());
        images.push(ImageInfo {
            url: src,
            alt,
            width,
            height,
            source: "img".to_owned(),
        });
    }

    for el in doc.select(&SEL_SOURCE_SRCSET) {
        let srcset = el.value().attr("srcset").unwrap_or("").to_owned();
        if !srcset.is_empty() {
            // Take the first URL from srcset
            let first_url = srcset.split(',').next().unwrap_or("").trim();
            let url = first_url.split_whitespace().next().unwrap_or("").to_owned();
            if !url.is_empty() {
                images.push(ImageInfo {
                    url,
                    alt: None,
                    width: None,
                    height: None,
                    source: "picture_source".to_owned(),
                });
            }
        }
    }

    // Extract from og:image
    for el in doc.select(&SEL_OG_IMAGE) {
        if let Some(content) = el.value().attr("content")
            && !content.is_empty()
        {
            images.push(ImageInfo {
                url: content.to_owned(),
                alt: None,
                width: None,
                height: None,
                source: "og:image".to_owned(),
            });
        }
    }

    // Extract from twitter:image
    for el in doc.select(&SEL_TWITTER_IMAGE) {
        if let Some(content) = el.value().attr("content")
            && !content.is_empty()
        {
            images.push(ImageInfo {
                url: content.to_owned(),
                alt: None,
                width: None,
                height: None,
                source: "twitter:image".to_owned(),
            });
        }
    }

    images
}

/// Extract feed links (RSS, Atom, JSON Feed) from a parsed HTML document.
pub(crate) fn extract_feeds(doc: &Html) -> Vec<FeedInfo> {
    let mut feeds = Vec::new();

    for el in doc.select(&SEL_FEED_ALTERNATE) {
        let link_type = el.value().attr("type").unwrap_or("");
        let href = el.value().attr("href").unwrap_or("").to_owned();
        let title = el.value().attr("title").map(String::from);

        let feed_type = match link_type {
            "application/rss+xml" => Some(FeedType::Rss),
            "application/atom+xml" => Some(FeedType::Atom),
            "application/json" | "application/feed+json" => Some(FeedType::JsonFeed),
            _ => None,
        };

        if let Some(ft) = feed_type {
            feeds.push(FeedInfo {
                url: href,
                title,
                feed_type: ft,
            });
        }
    }
    feeds
}

/// Extract JSON-LD structured data entries from a parsed HTML document.
pub(crate) fn extract_json_ld(doc: &Html) -> Vec<JsonLdEntry> {
    let mut entries = Vec::new();

    for el in doc.select(&SEL_JSON_LD) {
        let raw = el.text().collect::<String>();
        if let Ok(val) = serde_json::from_str::<serde_json::Value>(&raw) {
            let schema_type = val
                .get("@type")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_owned();
            let name = val.get("name").and_then(|v| v.as_str()).map(String::from);
            entries.push(JsonLdEntry {
                schema_type,
                name,
                raw,
            });
        }
    }
    entries
}

/// Detect the character encoding from the Content-Type header or HTML meta tags.
pub(crate) fn detect_charset(content_type: &str, body: &str) -> Option<String> {
    // From Content-Type header
    if let Some(pos) = content_type.to_lowercase().find("charset=") {
        let charset = content_type[pos + 8..]
            .split(';')
            .next()
            .unwrap_or("")
            .trim()
            .trim_matches('"')
            .to_lowercase();
        if !charset.is_empty() {
            return Some(charset);
        }
    }

    // From <meta charset="..."> in HTML body
    let lower_body = body.to_lowercase();
    if let Some(pos) = lower_body.find("charset=") {
        // Could be <meta charset="utf-8"> or <meta ... content="...; charset=utf-8">
        let after = &body[pos + 8..];
        let charset = after
            .trim_start_matches(['"', '\''])
            .split(|c: char| c == '"' || c == '\'' || c == '>' || c == ';' || c.is_whitespace())
            .next()
            .unwrap_or("")
            .trim()
            .to_lowercase();
        if !charset.is_empty() {
            return Some(charset);
        }
    }

    // Check for UTF-8 BOM
    if body.starts_with('\u{FEFF}') {
        return Some("utf-8".to_owned());
    }

    None
}

/// Check whether a Content-Type header indicates binary content.
pub(crate) fn is_binary_content_type(ct: &str) -> bool {
    let lower = ct.to_lowercase();
    lower.starts_with("image/")
        || lower.starts_with("video/")
        || lower.starts_with("audio/")
        || lower.starts_with("application/octet-stream")
        || lower.starts_with("application/zip")
        || lower.starts_with("application/pdf")
}

/// Check whether a URL has a binary file extension.
pub(crate) fn is_binary_url(url: &str) -> bool {
    let lower = url.to_lowercase();
    // Strip query and fragment
    let path = lower.split('?').next().unwrap_or(&lower);
    let path = path.split('#').next().unwrap_or(path);
    BINARY_EXTENSIONS.iter().any(|ext| path.ends_with(ext))
}

/// Check whether content is a PDF based on Content-Type or body magic bytes.
pub(crate) fn is_pdf_content(ct: &str, body: &str) -> bool {
    ct.to_lowercase().contains("application/pdf") || body.starts_with("%PDF")
}

/// Check whether a URL has a `.pdf` extension.
pub(crate) fn is_pdf_url(url: &str) -> bool {
    let lower = url.to_lowercase();
    let path = lower.split('?').next().unwrap_or(&lower);
    let path = path.split('#').next().unwrap_or(path);
    path.ends_with(".pdf")
}

/// Remove elements matching the given CSS selectors from the HTML string.
pub(crate) fn apply_remove_tags(html: &str, tags: &[String]) -> String {
    let doc = Html::parse_document(html);
    let mut output = html.to_owned();
    for tag in tags {
        if let Ok(sel) = Selector::parse(tag) {
            for el in doc.select(&sel) {
                let outer = el.html();
                output = output.replace(&outer, "");
            }
        }
    }
    output
}

/// Extract the main content from an HTML page (looks for `<main>`, `<article>`, or `[role='main']`).
pub(crate) fn extract_main_content(html: &str) -> String {
    let doc = Html::parse_document(html);
    doc.select(&SEL_MAIN_CONTENT)
        .next()
        .map(|el| el.html())
        .unwrap_or_else(|| html.to_owned())
}

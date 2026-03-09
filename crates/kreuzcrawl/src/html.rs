//! HTML parsing helpers for metadata extraction, link discovery, and content processing.

use std::sync::LazyLock;

use regex::Regex;
use scraper::{Html, Selector};
use url::Url;

use crate::types::{
    ArticleMetadata, FaviconInfo, FeedInfo, FeedType, HeadingInfo, HreflangEntry, ImageInfo,
    ImageSource, JsonLdEntry, LinkInfo, LinkType, PageMetadata,
};

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
    .expect("valid regex: META_RE_NAME_CONTENT")
});
static META_RE_CONTENT_NAME: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r#"<meta\s+[^>]*content\s*=\s*["']([^"']+)["'][^>]*name\s*=\s*["']([^"']+)["'][^>]*>"#,
    )
    .expect("valid regex: META_RE_CONTENT_NAME")
});

// --- LazyLock CSS selectors ---

static SEL_META: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("meta").expect("valid selector: meta"));
static SEL_TITLE: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("title").expect("valid selector: title"));
static SEL_CANONICAL: LazyLock<Selector> = LazyLock::new(|| {
    Selector::parse("link[rel='canonical']").expect("valid selector: link[rel='canonical']")
});
static SEL_ROBOTS_META: LazyLock<Selector> = LazyLock::new(|| {
    Selector::parse("meta[name='robots']").expect("valid selector: meta[name='robots']")
});
static SEL_META_REFRESH: LazyLock<Selector> = LazyLock::new(|| {
    Selector::parse("meta[http-equiv='refresh']")
        .expect("valid selector: meta[http-equiv='refresh']")
});
static SEL_A_HREF: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("a[href]").expect("valid selector: a[href]"));
static SEL_BASE_HREF: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("base[href]").expect("valid selector: base[href]"));
static SEL_IMG_SRC: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("img[src]").expect("valid selector: img[src]"));
static SEL_SOURCE_SRCSET: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("source[srcset]").expect("valid selector: source[srcset]"));
static SEL_OG_IMAGE: LazyLock<Selector> = LazyLock::new(|| {
    Selector::parse("meta[property='og:image']").expect("valid selector: meta[property='og:image']")
});
static SEL_TWITTER_IMAGE: LazyLock<Selector> = LazyLock::new(|| {
    Selector::parse("meta[name='twitter:image']")
        .expect("valid selector: meta[name='twitter:image']")
});
static SEL_FEED_ALTERNATE: LazyLock<Selector> = LazyLock::new(|| {
    Selector::parse("link[rel='alternate']").expect("valid selector: link[rel='alternate']")
});
static SEL_JSON_LD: LazyLock<Selector> = LazyLock::new(|| {
    Selector::parse("script[type='application/ld+json']")
        .expect("valid selector: script[type='application/ld+json']")
});
static SEL_MAIN_CONTENT: LazyLock<Selector> = LazyLock::new(|| {
    Selector::parse("main, article, [role='main']")
        .expect("valid selector: main, article, [role='main']")
});
static SEL_HTML: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("html").expect("valid selector: html"));
static SEL_HREFLANG: LazyLock<Selector> = LazyLock::new(|| {
    Selector::parse("link[rel='alternate'][hreflang]")
        .expect("valid selector: link[rel='alternate'][hreflang]")
});
static SEL_FAVICON: LazyLock<Selector> = LazyLock::new(|| {
    Selector::parse("link[rel='icon'], link[rel='shortcut icon'], link[rel='apple-touch-icon']")
        .expect("valid selector: link[rel='icon'], link[rel='shortcut icon'], link[rel='apple-touch-icon']")
});
static SEL_HEADINGS: LazyLock<Selector> = LazyLock::new(|| {
    Selector::parse("h1, h2, h3, h4, h5, h6").expect("valid selector: h1, h2, h3, h4, h5, h6")
});

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

    // Extract html lang and dir attributes
    if let Some(html_el) = doc.select(&SEL_HTML).next() {
        md.html_lang = html_el.value().attr("lang").map(String::from);
        md.html_dir = html_el.value().attr("dir").map(String::from);
    }

    let mut article = ArticleMetadata::default();
    let mut has_article = false;
    let mut og_locale_alternates: Vec<String> = Vec::new();

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
            "keywords" => md.keywords = Some(content),
            "author" => md.author = Some(content),
            "viewport" => md.viewport = Some(content),
            "theme-color" => md.theme_color = Some(content),
            "generator" => md.generator = Some(content),
            "robots" => md.robots = Some(content),
            "og:title" => md.og_title = Some(content),
            "og:type" => md.og_type = Some(content),
            "og:image" => md.og_image = Some(content),
            "og:description" => md.og_description = Some(content),
            "og:url" => md.og_url = Some(content),
            "og:site_name" => md.og_site_name = Some(content),
            "og:locale" => md.og_locale = Some(content),
            "og:video" => md.og_video = Some(content),
            "og:audio" => md.og_audio = Some(content),
            "og:locale:alternate" => og_locale_alternates.push(content),
            "article:published_time" => {
                article.published_time = Some(content);
                has_article = true;
            }
            "article:modified_time" => {
                article.modified_time = Some(content);
                has_article = true;
            }
            "article:author" => {
                article.author = Some(content);
                has_article = true;
            }
            "article:section" => {
                article.section = Some(content);
                has_article = true;
            }
            "article:tag" => {
                article.tags.push(content);
                has_article = true;
            }
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

    if has_article {
        md.article = Some(article);
    }
    if !og_locale_alternates.is_empty() {
        md.og_locale_alternates = Some(og_locale_alternates);
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
    if content_type.contains("html") {
        return true;
    }
    let trimmed = body.trim_start();
    if !trimmed.starts_with('<') {
        return false;
    }
    let lower = trimmed.to_lowercase();
    // Reject XML/SVG that isn't HTML
    if lower.starts_with("<?xml") && !lower.contains("<html") {
        return false;
    }
    // Accept common HTML markers
    lower.starts_with("<!doctype")
        || lower.starts_with("<html")
        || lower.starts_with("<head")
        || lower.starts_with("<body")
        || lower.starts_with("<div")
        || lower.starts_with("<p")
        || lower.starts_with("<h1")
        || lower.starts_with("<script")
        || lower.starts_with("<meta")
        || lower.starts_with("<link")
        || lower.starts_with("<!")
}

/// Classify a link as internal, external, anchor, or document.
pub(crate) fn classify_link(href: &str, base_url: &Url) -> LinkType {
    if href.starts_with('#') {
        return LinkType::Anchor;
    }

    // Check for document extensions
    let lower = href.to_lowercase();
    for ext in DOCUMENT_EXTENSIONS {
        if lower.ends_with(ext) {
            return LinkType::Document;
        }
    }

    // Try resolving
    if let Ok(resolved) = base_url.join(href) {
        if resolved.host_str() != base_url.host_str() {
            return LinkType::External;
        }
        LinkType::Internal
    } else if href.starts_with("http://") || href.starts_with("https://") {
        if let Ok(u) = Url::parse(href)
            && u.host_str() != base_url.host_str()
        {
            return LinkType::External;
        }
        LinkType::Internal
    } else {
        LinkType::Internal
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

        // Protocol-relative URLs: resolve against the base URL's scheme for classification
        let link_type = if href.starts_with("//") {
            let resolved = format!("{}:{}", effective_base.scheme(), href);
            if let Ok(u) = Url::parse(&resolved) {
                if u.host_str() != effective_base.host_str() {
                    LinkType::External
                } else {
                    LinkType::Internal
                }
            } else {
                LinkType::External
            }
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

/// Resolve a URL against a base URL, returning the resolved string or the original if resolution fails.
fn resolve_url(src: &str, base_url: &Url) -> String {
    base_url
        .join(src)
        .map(|u| u.to_string())
        .unwrap_or_else(|_| src.to_owned())
}

/// Extract all images from a parsed HTML document (img, picture source, og:image, twitter:image).
pub(crate) fn extract_images(doc: &Html, base_url: &Url) -> Vec<ImageInfo> {
    let mut images = Vec::new();

    for el in doc.select(&SEL_IMG_SRC) {
        let src = el.value().attr("src").unwrap_or("");
        if src.is_empty() || src.starts_with("data:") {
            continue;
        }
        let resolved = resolve_url(src, base_url);
        let alt = el.value().attr("alt").map(String::from);
        let width = el.value().attr("width").and_then(|w| w.parse::<u32>().ok());
        let height = el
            .value()
            .attr("height")
            .and_then(|h| h.parse::<u32>().ok());
        images.push(ImageInfo {
            url: resolved,
            alt,
            width,
            height,
            source: ImageSource::Img,
        });
    }

    for el in doc.select(&SEL_SOURCE_SRCSET) {
        let srcset = el.value().attr("srcset").unwrap_or("");
        if !srcset.is_empty() {
            // Take the first URL from srcset
            let first_url = srcset.split(',').next().unwrap_or("").trim();
            let raw_url = first_url.split_whitespace().next().unwrap_or("");
            if !raw_url.is_empty() {
                let resolved = resolve_url(raw_url, base_url);
                images.push(ImageInfo {
                    url: resolved,
                    alt: None,
                    width: None,
                    height: None,
                    source: ImageSource::PictureSource,
                });
            }
        }
    }

    // Extract from og:image
    for el in doc.select(&SEL_OG_IMAGE) {
        if let Some(content) = el.value().attr("content")
            && !content.is_empty()
        {
            let resolved = resolve_url(content, base_url);
            images.push(ImageInfo {
                url: resolved,
                alt: None,
                width: None,
                height: None,
                source: ImageSource::OgImage,
            });
        }
    }

    // Extract from twitter:image
    for el in doc.select(&SEL_TWITTER_IMAGE) {
        if let Some(content) = el.value().attr("content")
            && !content.is_empty()
        {
            let resolved = resolve_url(content, base_url);
            images.push(ImageInfo {
                url: resolved,
                alt: None,
                width: None,
                height: None,
                source: ImageSource::TwitterImage,
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
    // From Content-Type header (ASCII search, case-insensitive)
    if let Some(pos) = ascii_find_case_insensitive(content_type.as_bytes(), b"charset=") {
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

    // From <meta charset="..."> in HTML body — search only first 2048 bytes for performance
    let search_len = body.len().min(2048);
    // Find a char boundary at or before the limit
    let search_end = (0..=search_len)
        .rev()
        .find(|&i| body.is_char_boundary(i))
        .unwrap_or(0);
    let head = &body[..search_end];
    if let Some(pos) = ascii_find_case_insensitive(head.as_bytes(), b"charset=") {
        // pos is a byte offset that is valid for the original string since "charset=" is ASCII
        let after = &head[pos + 8..];
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

/// Case-insensitive search for an ASCII needle within a byte slice.
/// Returns the byte offset of the first match, or `None`.
fn ascii_find_case_insensitive(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    if needle.is_empty() || haystack.len() < needle.len() {
        return None;
    }
    let needle_lower: Vec<u8> = needle.iter().map(|b| b.to_ascii_lowercase()).collect();
    'outer: for i in 0..=(haystack.len() - needle.len()) {
        for (j, &nb) in needle_lower.iter().enumerate() {
            if haystack[i + j].to_ascii_lowercase() != nb {
                continue 'outer;
            }
        }
        return Some(i);
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
///
/// Parses the HTML and re-serializes it, removing matched elements.
/// The output is based on the re-serialized DOM to ensure consistent string representations.
pub(crate) fn apply_remove_tags(html: &str, tags: &[String]) -> String {
    let doc = Html::parse_document(html);
    // Collect the serialized HTML of all elements to remove
    let mut to_remove = Vec::new();
    for tag in tags {
        if let Ok(sel) = Selector::parse(tag) {
            for el in doc.select(&sel) {
                to_remove.push(el.html());
            }
        }
    }
    if to_remove.is_empty() {
        return html.to_owned();
    }
    // Work on the re-serialized document so string representations match
    let mut output = doc.root_element().inner_html();
    for fragment in &to_remove {
        if let Some(pos) = output.find(fragment.as_str()) {
            output.replace_range(pos..pos + fragment.len(), "");
        }
    }
    output
}

/// Extract hreflang alternate links from a parsed HTML document.
pub(crate) fn extract_hreflangs(doc: &Html) -> Vec<HreflangEntry> {
    let mut entries = Vec::new();
    for el in doc.select(&SEL_HREFLANG) {
        let lang = el.value().attr("hreflang").unwrap_or("").to_owned();
        let url = el.value().attr("href").unwrap_or("").to_owned();
        if !lang.is_empty() && !url.is_empty() {
            entries.push(HreflangEntry { lang, url });
        }
    }
    entries
}

/// Extract favicon and icon links from a parsed HTML document.
pub(crate) fn extract_favicons(doc: &Html) -> Vec<FaviconInfo> {
    let mut favicons = Vec::new();
    for el in doc.select(&SEL_FAVICON) {
        let url = el.value().attr("href").unwrap_or("").to_owned();
        if url.is_empty() {
            continue;
        }
        let rel = el.value().attr("rel").unwrap_or("").to_owned();
        let sizes = el.value().attr("sizes").map(String::from);
        let mime_type = el.value().attr("type").map(String::from);
        favicons.push(FaviconInfo {
            url,
            rel,
            sizes,
            mime_type,
        });
    }
    favicons
}

/// Extract heading elements (h1-h6) from a parsed HTML document.
pub(crate) fn extract_headings(doc: &Html) -> Vec<HeadingInfo> {
    let mut headings = Vec::new();
    for el in doc.select(&SEL_HEADINGS) {
        let tag_name = el.value().name();
        let level = match tag_name {
            "h1" => 1,
            "h2" => 2,
            "h3" => 3,
            "h4" => 4,
            "h5" => 5,
            "h6" => 6,
            _ => continue,
        };
        let text = el.text().collect::<String>().trim().to_owned();
        headings.push(HeadingInfo { level, text });
    }
    headings
}

/// Compute the word count of visible text in the HTML body.
pub(crate) fn compute_word_count(doc: &Html) -> usize {
    static SEL_BODY: LazyLock<Selector> =
        LazyLock::new(|| Selector::parse("body").expect("valid selector: body"));
    let body_text = doc
        .select(&SEL_BODY)
        .next()
        .map(|el| el.text().collect::<String>())
        .unwrap_or_default();
    body_text.split_whitespace().count()
}

/// Extract the main content from an HTML page (looks for `<main>`, `<article>`, or `[role='main']`).
pub(crate) fn extract_main_content(html: &str) -> String {
    let doc = Html::parse_document(html);
    doc.select(&SEL_MAIN_CONTENT)
        .next()
        .map(|el| el.html())
        .unwrap_or_else(|| html.to_owned())
}

//! Link extraction and classification from HTML documents.

use scraper::Html;
use url::Url;

use crate::types::{LinkInfo, LinkType};

use super::selectors::{SEL_A_HREF, SEL_BASE_HREF};

/// Document file extensions used for link classification.
static DOCUMENT_EXTENSIONS: &[&str] = &[
    ".pdf", ".doc", ".docx", ".xls", ".xlsx", ".ppt", ".pptx", ".odt", ".ods", ".odp", ".rtf",
    ".csv", ".txt", ".zip", ".tar", ".gz", ".rar",
];

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

        // Protocol-relative URLs
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

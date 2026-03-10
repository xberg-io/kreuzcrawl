//! Metadata extraction from HTML documents.

use scraper::Html;

use crate::types::{ArticleMetadata, PageMetadata};

use super::selectors::{
    META_RE_CONTENT_NAME, META_RE_NAME_CONTENT, SEL_CANONICAL, SEL_HTML, SEL_META,
    SEL_META_REFRESH, SEL_ROBOTS_META, SEL_TITLE,
};

/// Extract metadata name-value pairs from raw HTML using regex (fallback for malformed HTML).
fn extract_metadata_from_raw(body: &str) -> Vec<(String, String)> {
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

/// Check whether a meta robots directive contains the given keyword.
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

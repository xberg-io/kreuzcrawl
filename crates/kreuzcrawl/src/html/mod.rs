//! HTML parsing helpers for metadata extraction, link discovery, and content processing.

mod charset;
mod content;
mod detection;
mod extract;
mod feeds;
mod images;
mod json_ld;
mod links;
mod metadata;
pub(crate) mod selectors;

use url::Url;

pub(crate) fn resolve_url(src: &str, base_url: &Url) -> String {
    base_url
        .join(src)
        .map(|u| u.to_string())
        .unwrap_or_else(|_| src.to_owned())
}

pub(crate) use charset::detect_charset;
pub(crate) use content::{apply_remove_tags, extract_main_content};
pub(crate) use detection::{
    is_binary_content_type, is_binary_url, is_html_content, is_pdf_content, is_pdf_url,
};
pub(crate) use extract::{HtmlExtraction, extract_page_data};
pub(crate) use links::extract_links;
pub(crate) use metadata::{detect_meta_refresh, detect_nofollow, detect_noindex};

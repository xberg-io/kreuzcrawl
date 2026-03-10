//! HTML parsing helpers for metadata extraction, link discovery, and content processing.

mod charset;
mod content;
mod detection;
mod feeds;
mod images;
mod json_ld;
mod links;
mod metadata;
mod selectors;

pub(crate) use charset::detect_charset;
pub(crate) use content::{apply_remove_tags, compute_word_count, extract_main_content};
pub(crate) use detection::{
    is_binary_content_type, is_binary_url, is_html_content, is_pdf_content, is_pdf_url,
};
pub(crate) use feeds::{extract_favicons, extract_feeds, extract_headings, extract_hreflangs};
pub(crate) use images::extract_images;
pub(crate) use json_ld::extract_json_ld;
pub(crate) use links::extract_links;
pub(crate) use metadata::{detect_meta_refresh, detect_nofollow, detect_noindex, extract_metadata};

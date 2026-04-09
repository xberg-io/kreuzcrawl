//! Image extraction from HTML documents.

use scraper::Html;
use url::Url;

use crate::types::{ImageInfo, ImageSource};

use super::resolve_url;
use super::selectors::{SEL_IMG_SRC, SEL_OG_IMAGE, SEL_SOURCE_SRCSET, SEL_TWITTER_IMAGE};

/// Extract all images from a parsed HTML document.
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

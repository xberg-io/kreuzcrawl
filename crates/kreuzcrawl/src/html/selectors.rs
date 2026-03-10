//! Lazily-initialized CSS selectors and regex patterns used across html submodules.

use std::sync::LazyLock;

use regex::Regex;
use scraper::Selector;

// --- LazyLock regex patterns for metadata extraction ---

pub(super) static META_RE_NAME_CONTENT: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r#"<meta\s+[^>]*name\s*=\s*["']([^"']+)["'][^>]*content\s*=\s*["']([^"']+)["'][^>]*>"#,
    )
    .expect("valid regex: META_RE_NAME_CONTENT")
});
pub(super) static META_RE_CONTENT_NAME: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r#"<meta\s+[^>]*content\s*=\s*["']([^"']+)["'][^>]*name\s*=\s*["']([^"']+)["'][^>]*>"#,
    )
    .expect("valid regex: META_RE_CONTENT_NAME")
});

// --- LazyLock CSS selectors ---

pub(super) static SEL_META: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("meta").expect("valid selector: meta"));
pub(super) static SEL_TITLE: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("title").expect("valid selector: title"));
pub(super) static SEL_CANONICAL: LazyLock<Selector> = LazyLock::new(|| {
    Selector::parse("link[rel='canonical']").expect("valid selector: link[rel='canonical']")
});
pub(super) static SEL_ROBOTS_META: LazyLock<Selector> = LazyLock::new(|| {
    Selector::parse("meta[name='robots']").expect("valid selector: meta[name='robots']")
});
pub(super) static SEL_META_REFRESH: LazyLock<Selector> = LazyLock::new(|| {
    Selector::parse("meta[http-equiv='refresh']")
        .expect("valid selector: meta[http-equiv='refresh']")
});
pub(super) static SEL_A_HREF: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("a[href]").expect("valid selector: a[href]"));
pub(super) static SEL_BASE_HREF: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("base[href]").expect("valid selector: base[href]"));
pub(super) static SEL_IMG_SRC: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("img[src]").expect("valid selector: img[src]"));
pub(super) static SEL_SOURCE_SRCSET: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("source[srcset]").expect("valid selector: source[srcset]"));
pub(super) static SEL_OG_IMAGE: LazyLock<Selector> = LazyLock::new(|| {
    Selector::parse("meta[property='og:image']").expect("valid selector: meta[property='og:image']")
});
pub(super) static SEL_TWITTER_IMAGE: LazyLock<Selector> = LazyLock::new(|| {
    Selector::parse("meta[name='twitter:image']")
        .expect("valid selector: meta[name='twitter:image']")
});
pub(super) static SEL_FEED_ALTERNATE: LazyLock<Selector> = LazyLock::new(|| {
    Selector::parse("link[rel='alternate']").expect("valid selector: link[rel='alternate']")
});
pub(super) static SEL_JSON_LD: LazyLock<Selector> = LazyLock::new(|| {
    Selector::parse("script[type='application/ld+json']")
        .expect("valid selector: script[type='application/ld+json']")
});
pub(super) static SEL_MAIN_CONTENT: LazyLock<Selector> = LazyLock::new(|| {
    Selector::parse("main, article, [role='main']")
        .expect("valid selector: main, article, [role='main']")
});
pub(super) static SEL_HTML: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("html").expect("valid selector: html"));
pub(super) static SEL_HREFLANG: LazyLock<Selector> = LazyLock::new(|| {
    Selector::parse("link[rel='alternate'][hreflang]")
        .expect("valid selector: link[rel='alternate'][hreflang]")
});
pub(super) static SEL_FAVICON: LazyLock<Selector> = LazyLock::new(|| {
    Selector::parse("link[rel='icon'], link[rel='shortcut icon'], link[rel='apple-touch-icon']")
        .expect(
            "valid selector: link[rel='icon'], \
             link[rel='shortcut icon'], link[rel='apple-touch-icon']",
        )
});
pub(super) static SEL_HEADINGS: LazyLock<Selector> = LazyLock::new(|| {
    Selector::parse("h1, h2, h3, h4, h5, h6").expect("valid selector: h1, h2, h3, h4, h5, h6")
});

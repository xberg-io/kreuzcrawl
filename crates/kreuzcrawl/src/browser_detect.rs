//! Detection heuristics for identifying pages that need JavaScript rendering.
//!
//! These are pure functions operating on HTML strings — no browser or network
//! access required. Always compiled regardless of feature flags.

use std::sync::LazyLock;

use regex::Regex;
use scraper::{Html, Selector};

/// Minimum word count to consider a page as having substantial content.
const MIN_CONTENT_WORD_COUNT: usize = 50;

/// Word count below which script tags suggest client-side rendering.
const SPARSE_CONTENT_WORD_COUNT: usize = 20;

/// Well-known SPA mount-point IDs with typically empty content.
static SPA_MOUNT_IDS: &[&str] = &["root", "app", "__next", "__nuxt"];

/// Selector for script elements.
static SCRIPT_SELECTOR: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("script[src]").expect("valid selector: script[src]"));

/// Selector for noscript elements.
static NOSCRIPT_SELECTOR: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("noscript").expect("valid selector: noscript"));

/// Pattern matching common noscript warning text.
static NOSCRIPT_WARNING: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)(enable|need|require)\s+(javascript|js)")
        .expect("noscript warning regex should compile")
});

/// Detect whether a page's HTML content suggests it needs JavaScript rendering
/// to produce meaningful content.
///
/// Returns `true` when the page appears to be a client-side rendered SPA shell
/// with no substantial server-rendered content.
pub(crate) fn detect_js_render_needed(body: &str, word_count: usize) -> bool {
    // Pages with substantial text content are not empty shells.
    if word_count >= MIN_CONTENT_WORD_COUNT {
        return false;
    }

    let doc = Html::parse_document(body);

    // Check for SPA mount points with empty or near-empty content.
    if has_empty_spa_mount(&doc) {
        return true;
    }

    // Check for noscript warnings about JS being required.
    if has_noscript_js_warning(&doc) {
        return true;
    }

    // Low text content combined with script tags suggests client-side rendering.
    if word_count < SPARSE_CONTENT_WORD_COUNT && has_script_tags(&doc) {
        return true;
    }

    false
}

/// Check if the document has a well-known SPA mount-point div that is empty
/// or contains only whitespace.
fn has_empty_spa_mount(doc: &Html) -> bool {
    for id in SPA_MOUNT_IDS {
        let selector_str = format!("#{id}");
        if let Ok(selector) = Selector::parse(&selector_str) {
            for el in doc.select(&selector) {
                let inner_text: String = el.text().collect::<String>();
                let trimmed = inner_text.trim();
                if trimmed.is_empty() {
                    return true;
                }
            }
        }
    }
    false
}

/// Check if the document has a `<noscript>` element with text suggesting
/// JavaScript is required to use the page.
fn has_noscript_js_warning(doc: &Html) -> bool {
    for el in doc.select(&NOSCRIPT_SELECTOR) {
        let text: String = el.text().collect();
        if NOSCRIPT_WARNING.is_match(&text) {
            return true;
        }
    }
    false
}

/// Check if the document has external script tags.
fn has_script_tags(doc: &Html) -> bool {
    doc.select(&SCRIPT_SELECTOR).next().is_some()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn react_shell_detected() {
        let html = r#"<html><body><div id="root"></div>
            <noscript>You need to enable JavaScript to run this app.</noscript>
            <script src="/main.js"></script></body></html>"#;
        assert!(detect_js_render_needed(html, 0));
    }

    #[test]
    fn vue_shell_detected() {
        let html = r#"<html><body><div id="app"></div>
            <script src="/app.js"></script></body></html>"#;
        assert!(detect_js_render_needed(html, 0));
    }

    #[test]
    fn next_empty_detected() {
        let html = r#"<html><body><div id="__next"></div>
            <script id="__NEXT_DATA__" type="application/json">{}</script>
            <script src="/_next/static/main.js"></script></body></html>"#;
        assert!(detect_js_render_needed(html, 0));
    }

    #[test]
    fn nuxt_shell_detected() {
        let html = r#"<html><body><div id="__nuxt"></div>
            <script src="/_nuxt/entry.js"></script></body></html>"#;
        assert!(detect_js_render_needed(html, 0));
    }

    #[test]
    fn next_ssr_not_detected() {
        let html = r#"<html><body><div id="__next"><main><h1>Hello World</h1>
            <p>This is a server-rendered page with plenty of content that should not
            trigger the JS render hint because it has substantial text content already
            present in the HTML response from the server.</p></main></div>
            <script src="/_next/static/main.js"></script></body></html>"#;
        assert!(!detect_js_render_needed(html, 60));
    }

    #[test]
    fn normal_page_not_detected() {
        let html = r#"<html><body><h1>Example</h1>
            <p>This is a normal page with content.</p></body></html>"#;
        assert!(!detect_js_render_needed(html, 10));
    }

    #[test]
    fn minimal_page_no_scripts_not_detected() {
        let html = r#"<html><body><h1>Contact</h1>
            <p>Email us at hello@example.com</p></body></html>"#;
        assert!(!detect_js_render_needed(html, 8));
    }

    #[test]
    fn noscript_warning_triggers() {
        let html = r#"<html><body><div id="app"></div>
            <noscript><strong>Please enable JavaScript to continue.</strong></noscript>
            <script src="/app.js"></script></body></html>"#;
        assert!(detect_js_render_needed(html, 0));
    }
}

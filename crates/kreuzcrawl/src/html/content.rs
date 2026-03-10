//! Content processing: tag removal, word count, main content extraction.

use std::sync::LazyLock;

use scraper::{Html, Selector};

use super::selectors::SEL_MAIN_CONTENT;

/// Remove elements matching the given CSS selectors from the HTML string.
pub(crate) fn apply_remove_tags(html: &str, tags: &[String]) -> String {
    let doc = Html::parse_document(html);
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
    let mut output = doc.root_element().inner_html();
    for fragment in &to_remove {
        if let Some(pos) = output.find(fragment.as_str()) {
            output.replace_range(pos..pos + fragment.len(), "");
        }
    }
    output
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

/// Extract the main content from an HTML page.
pub(crate) fn extract_main_content(html: &str) -> String {
    let doc = Html::parse_document(html);
    doc.select(&SEL_MAIN_CONTENT)
        .next()
        .map(|el| el.html())
        .unwrap_or_else(|| html.to_owned())
}

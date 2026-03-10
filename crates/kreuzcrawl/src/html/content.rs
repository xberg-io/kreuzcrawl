//! Content processing: tag removal, word count, main content extraction.

use std::sync::LazyLock;

use scraper::{Html, Selector};

use super::selectors::SEL_MAIN_CONTENT;

/// Remove elements matching the given CSS selectors from the HTML string.
pub(crate) fn apply_remove_tags(html: &str, tags: &[String]) -> String {
    let doc = Html::parse_document(html);
    // Collect (start_offset, fragment) pairs from the serialized DOM
    let serialized = doc.root_element().inner_html();
    let mut ranges: Vec<(usize, usize)> = Vec::new();
    for tag in tags {
        if let Ok(sel) = Selector::parse(tag) {
            for el in doc.select(&sel) {
                let fragment = el.html();
                // Find the exact position in the serialized output
                if let Some(pos) = serialized.find(&fragment) {
                    ranges.push((pos, pos + fragment.len()));
                }
            }
        }
    }
    if ranges.is_empty() {
        return html.to_owned();
    }
    // Sort by start position descending so we remove from end first,
    // preserving earlier offsets.
    ranges.sort_by(|a, b| b.0.cmp(&a.0));
    // Deduplicate overlapping ranges
    ranges.dedup_by(|a, b| a.0 >= b.0 && a.0 < b.1);
    let mut output = serialized;
    for (start, end) in &ranges {
        output.replace_range(*start..*end, "");
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

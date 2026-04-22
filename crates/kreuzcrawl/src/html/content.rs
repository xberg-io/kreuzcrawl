//! Content processing: word count.

use tl::VDom;

/// Compute the word count of visible text in the HTML body.
pub(crate) fn compute_word_count(dom: &VDom<'_>) -> usize {
    let parser = dom.parser();
    let body_text = dom
        .query_selector("body")
        .and_then(|mut iter| {
            iter.next()
                .and_then(|h| h.get(parser))
                .map(|node| node.inner_text(parser).to_string())
        })
        .unwrap_or_default();
    body_text.split_whitespace().count()
}

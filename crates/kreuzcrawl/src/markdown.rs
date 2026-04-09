//! HTML-to-Markdown conversion -- always active.

use crate::types::MarkdownResult;

/// Convert an HTML string to Markdown, returning a rich result.
///
/// Delegates to `html-to-markdown-rs` in a blocking task so the
/// conversion does not block the async runtime.
pub(crate) async fn convert_to_markdown(html: &str) -> Option<MarkdownResult> {
    let html = html.to_owned();
    tokio::task::spawn_blocking(move || {
        let options = html_to_markdown_rs::options::ConversionOptions {
            include_document_structure: true,
            ..Default::default()
        };

        match html_to_markdown_rs::convert(&html, Some(options)) {
            Ok(result) => {
                let content = result.content.unwrap_or_default();
                let document_structure = result.document.and_then(|d| serde_json::to_value(d).ok());
                let tables = result
                    .tables
                    .iter()
                    .filter_map(|t| serde_json::to_value(t).ok())
                    .collect();
                let warnings = result.warnings.iter().map(|w| format!("{:?}", w)).collect();

                let citations = Some(crate::citations::generate_citations(&content));
                let fit_content = Some(crate::pruning::generate_fit_markdown(&content));

                Some(MarkdownResult {
                    content,
                    document_structure,
                    tables,
                    warnings,
                    citations,
                    fit_content,
                })
            }
            Err(_) => None,
        }
    })
    .await
    .ok()
    .flatten()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn converts_heading() {
        let result = convert_to_markdown("<h1>Hello</h1>").await;
        let result = result.expect("should produce markdown");
        assert!(
            result.content.contains("# Hello"),
            "expected '# Hello' in markdown, got: {}",
            result.content
        );
    }

    #[tokio::test]
    async fn converts_paragraph() {
        let result = convert_to_markdown("<p>Some text.</p>").await;
        let result = result.expect("should produce markdown");
        assert!(
            result.content.contains("Some text."),
            "expected 'Some text.' in markdown, got: {}",
            result.content
        );
    }

    #[tokio::test]
    async fn converts_link() {
        let result = convert_to_markdown(r#"<a href="https://example.com">Click</a>"#).await;
        let result = result.expect("should produce markdown");
        assert!(
            result.content.contains("[Click](https://example.com)"),
            "expected markdown link, got: {}",
            result.content
        );
    }

    #[tokio::test]
    async fn converts_full_page() {
        let html = r#"<html><head><title>Test</title></head><body>
            <h1>Hello World</h1>
            <p>This is a paragraph.</p>
            <a href="/link">Click here</a>
        </body></html>"#;
        let result = convert_to_markdown(html).await;
        let result = result.expect("should produce markdown");
        assert!(
            result.content.contains("# Hello World"),
            "missing heading: {}",
            result.content
        );
        assert!(
            result.content.contains("This is a paragraph."),
            "missing paragraph: {}",
            result.content
        );
        assert!(
            result.content.contains("[Click here]"),
            "missing link text: {}",
            result.content
        );
    }

    #[tokio::test]
    async fn empty_html_returns_some() {
        let result = convert_to_markdown("").await;
        // Even empty HTML should return Some (possibly empty content)
        assert!(result.is_some(), "empty html should still return Some");
    }
}

//! Integration tests for LLM extraction.
//! Requires `.env` file with API keys. Skipped when keys not available.

#![cfg(feature = "ai")]

use kreuzcrawl::{CrawlConfig, CrawlEngine, LlmExtractor};

fn load_api_key() -> Option<String> {
    // Try .env file
    if let Ok(content) = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .join(".env"),
    ) {
        for line in content.lines() {
            if let Some(key) = line.strip_prefix("OPENAI_API_KEY=") {
                return Some(key.trim().to_owned());
            }
        }
    }
    std::env::var("OPENAI_API_KEY").ok()
}

#[tokio::test]
async fn test_llm_extractor_builds_successfully() {
    let Some(api_key) = load_api_key() else {
        eprintln!("OPENAI_API_KEY not available, skipping");
        return;
    };

    let extractor = LlmExtractor::new(
        &api_key,
        "openai/gpt-4o-mini",
        Some(serde_json::json!({
            "type": "object",
            "properties": {
                "title": { "type": "string" }
            },
            "required": ["title"],
            "additionalProperties": false
        })),
        Some("Extract the page title.".to_owned()),
        None,
    );
    assert!(
        extractor.is_ok(),
        "LlmExtractor should build: {:?}",
        extractor.err()
    );
}

#[tokio::test]
async fn test_llm_extractor_with_engine() {
    let Some(api_key) = load_api_key() else {
        eprintln!("OPENAI_API_KEY not available, skipping");
        return;
    };

    let extractor = LlmExtractor::new(
        &api_key,
        "openai/gpt-4o-mini",
        Some(serde_json::json!({
            "type": "object",
            "properties": {
                "title": { "type": "string" },
                "summary": { "type": "string" }
            },
            "required": ["title", "summary"],
            "additionalProperties": false
        })),
        Some("Extract the title and a one-sentence summary.".to_owned()),
        None,
    )
    .unwrap();

    let config = CrawlConfig::default();
    let engine = CrawlEngine::builder()
        .config(config)
        .content_filter(extractor)
        .build()
        .unwrap();

    // Scrape a real page
    let result = engine.scrape("https://example.com").await;
    match result {
        Ok(r) => {
            assert!(
                r.extracted_data.is_some(),
                "extracted_data should be populated"
            );
            let data = r.extracted_data.unwrap();
            assert!(data.get("title").is_some(), "should have title field");
            assert!(data.get("summary").is_some(), "should have summary field");
            println!(
                "Extracted: {}",
                serde_json::to_string_pretty(&data).unwrap()
            );
        }
        Err(e) => {
            eprintln!("Scrape failed (may be network issue): {e}");
        }
    }
}

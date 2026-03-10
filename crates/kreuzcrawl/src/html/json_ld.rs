//! JSON-LD structured data extraction from HTML documents.

use scraper::Html;

use crate::types::JsonLdEntry;

use super::selectors::SEL_JSON_LD;

/// Extract JSON-LD structured data entries from a parsed HTML document.
pub(crate) fn extract_json_ld(doc: &Html) -> Vec<JsonLdEntry> {
    let mut entries = Vec::new();

    for el in doc.select(&SEL_JSON_LD) {
        let raw = el.text().collect::<String>();
        if let Ok(val) = serde_json::from_str::<serde_json::Value>(&raw) {
            let schema_type = val
                .get("@type")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_owned();
            let name = val.get("name").and_then(|v| v.as_str()).map(String::from);
            entries.push(JsonLdEntry {
                schema_type,
                name,
                raw,
            });
        }
    }
    entries
}

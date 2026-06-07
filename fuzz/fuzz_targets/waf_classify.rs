//! libFuzzer target: WAF classifier robustness against random HTTP responses.
#![no_main]

use libfuzzer_sys::fuzz_target;
use kreuzcrawl::{TomlClassifier, WafClassifier, http::HttpResponse};

fuzz_target!(|data: &[u8]| {
    // First byte determines the HTTP status code offset (200..599).
    let Some((&status_byte, rest)) = data.split_first() else { return };
    let status = 200u16 + (status_byte as u16 % 400);

    // Remaining bytes are treated as an arbitrary response body.
    let body = String::from_utf8_lossy(rest).into_owned();
    let body_bytes = rest.to_vec();

    // HttpResponse has public fields -- construct directly, no helper needed.
    let response = HttpResponse {
        status,
        content_type: String::new(),
        body,
        body_bytes,
        headers: std::collections::HashMap::new(),
        browser_extras: None,
        final_url: String::new(),
    };

    let classifier = TomlClassifier::builtin();
    let _ = classifier.classify(&response);
});

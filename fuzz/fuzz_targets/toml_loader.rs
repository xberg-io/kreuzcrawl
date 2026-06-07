//! libFuzzer target: WAF-rules TOML loader robustness against random byte input.
#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let Ok(s) = std::str::from_utf8(data) else { return };
    let _ = kreuzcrawl::waf_rules_from_str(s);
});

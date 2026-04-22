//! Quality metric computation.
//!
//! Tokenises scrape output and fixture ground-truth text, then computes
//! token-level truth coverage, noise rejection, and a combined quality score.
//!
//! ## Scoring model
//!
//! The scrape-evals dataset provides asymmetric ground truth:
//! - `truth_text`: core content that **should** appear in the extraction.
//! - `lie_text`: noise that should **not** appear in the extraction.
//!
//! These metrics use dataset-specific definitions — they are NOT standard IR
//! precision/recall:
//!
//! * **Truth coverage** = `truth_tokens_found / truth_tokens_total`
//! * **Noise rejection** = `1.0 - (lie_tokens_found / lie_tokens_total)`
//! * **Quality score**:
//!   - Both provided: harmonic mean of truth_coverage and noise_rejection
//!     (0.0 when denominator is 0)
//!   - Only truth: truth_coverage
//!   - Only lie: noise_rejection
//!   - Neither: `None`
//!
//! Extracted content is markdown-stripped before tokenization so that syntax
//! tokens (`#`, `**`, `[]()`, etc.) do not inflate or corrupt scores.
//!
//! ## Limitations
//!
//! - Matching is set-based (not frequency-aware): a term counts once regardless
//!   of how often it appears.
//! - Markdown stripping is best-effort via regex; unusual or deeply nested
//!   syntax may survive stripping and affect scores.
//! - Very short truth or lie texts (1–2 tokens) may produce unstable scores
//!   because a single token match swings the metric by 50–100 %.

use ahash::AHashSet;
use regex::Regex;
use std::sync::OnceLock;

use crate::types::ScrapeQualityMetrics;

/// Strip common markdown syntax from `text` before tokenization.
///
/// This prevents syntax tokens (`#`, `**`, `` ` ``, `|`, etc.) from
/// inflating or corrupting token-overlap scores when extracted content is
/// markdown but ground-truth texts are plain text.
///
/// Transformations applied (in order):
/// 1. Images `![alt](url)` → `alt`
/// 2. Links `[text](url)` → `text`
/// 3. Fenced code blocks ` ``` … ``` ` → (removed)
/// 4. Inline code `` `…` `` → (removed)
/// 5. Bold/italic markers `**`, `__`, `*`, `_` → (removed)
/// 6. Heading markers `#+ ` at line start → (removed)
/// 7. Horizontal rules `---`, `***`, `___` on their own line → (removed)
/// 8. Blockquote markers `>` at line start → (removed)
/// 9. Table pipe characters `|` → space
pub(crate) fn strip_markdown(text: &str) -> String {
    // Compile regexes once per process.
    static IMAGE_RE: OnceLock<Regex> = OnceLock::new();
    static LINK_RE: OnceLock<Regex> = OnceLock::new();
    static FENCED_CODE_RE: OnceLock<Regex> = OnceLock::new();
    static INLINE_CODE_RE: OnceLock<Regex> = OnceLock::new();
    static BOLD_ITALIC_RE: OnceLock<Regex> = OnceLock::new();
    static HEADING_RE: OnceLock<Regex> = OnceLock::new();
    static HLINE_RE: OnceLock<Regex> = OnceLock::new();
    static BLOCKQUOTE_RE: OnceLock<Regex> = OnceLock::new();

    let image_re = IMAGE_RE.get_or_init(|| Regex::new(r"!\[([^\]]*)\]\([^)]*\)").unwrap());
    let link_re = LINK_RE.get_or_init(|| Regex::new(r"\[([^\]]*)\]\([^)]*\)").unwrap());
    let fenced_code_re =
        FENCED_CODE_RE.get_or_init(|| Regex::new(r"(?s)```[^`]*```").unwrap());
    let inline_code_re = INLINE_CODE_RE.get_or_init(|| Regex::new(r"`[^`]*`").unwrap());
    // Strip bold/italic markers: `**`, `__`, lone `*`, lone `_`.
    // The regex crate does not support lookaheads, so we match the two-char
    // forms first, then remove any remaining lone `*` or `_` characters.
    let bold_italic_re =
        BOLD_ITALIC_RE.get_or_init(|| Regex::new(r"\*\*|__|[*_]").unwrap());
    let heading_re = HEADING_RE.get_or_init(|| Regex::new(r"(?m)^#{1,6}\s+").unwrap());
    let hline_re = HLINE_RE
        .get_or_init(|| Regex::new(r"(?m)^\s*(?:---|\*\*\*|___)\s*$").unwrap());
    let blockquote_re =
        BLOCKQUOTE_RE.get_or_init(|| Regex::new(r"(?m)^>\s?").unwrap());

    let s = image_re.replace_all(text, "$1");
    let s = link_re.replace_all(&s, "$1");
    let s = fenced_code_re.replace_all(&s, "");
    let s = inline_code_re.replace_all(&s, "");
    let s = bold_italic_re.replace_all(&s, "");
    let s = heading_re.replace_all(&s, "");
    let s = hline_re.replace_all(&s, "");
    let s = blockquote_re.replace_all(&s, "");
    s.replace('|', " ")
}

/// Truth coverage threshold above which [`ScrapeQualityMetrics::truth_found`] is `true`.
const TRUTH_FOUND_THRESHOLD: f64 = 0.5;

/// Noise rejection threshold above which [`ScrapeQualityMetrics::lie_rejected`] is `true`.
const LIE_REJECTED_THRESHOLD: f64 = 0.5;

/// Compute quality metrics for extracted content against ground truth.
///
/// Returns `None` if neither `truth_text` nor `lie_text` is provided (no
/// ground truth to score against), or if `truth_text` is `Some("")` (empty
/// after tokenization — a perfect score would be meaningless).
///
/// # Examples
///
/// ```
/// use benchmark_harness::quality::compute_scrape_quality;
///
/// let metrics = compute_scrape_quality(
///     "The quick brown fox jumps over the lazy dog",
///     Some("quick brown fox"),
///     Some("completely unrelated noise"),
/// )
/// .expect("ground truth provided");
///
/// assert!(metrics.truth_coverage > 0.0);
/// assert!(metrics.lie_rejected);
/// ```
pub fn compute_scrape_quality(
    extracted_content: &str,
    truth_text: Option<&str>,
    lie_text: Option<&str>,
) -> Option<ScrapeQualityMetrics> {
    if truth_text.is_none() && lie_text.is_none() {
        return None;
    }

    // Strip markdown syntax before tokenizing so that structural tokens
    // (`#`, `**`, `|`, etc.) do not inflate or corrupt overlap scores.
    let stripped = strip_markdown(extracted_content);
    let extracted_tokens = tokenize(&stripped);
    let extracted_set = token_set(&extracted_tokens);

    // --- truth_coverage: fraction of truth tokens found in extraction ---
    //
    // When truth_text is None we do NOT default to 1.0 — that would
    // manufacture a phantom perfect truth score. Instead, the quality_score
    // computation below uses only the available signal.
    let (truth_coverage, truth_tokens_found, truth_tokens_total) = match truth_text {
        None => (None, 0_usize, 0_usize),
        Some(text) => {
            let truth_tokens = tokenize(text);
            let truth_set = token_set(&truth_tokens);
            let total = truth_set.len();
            // Empty truth_text after tokenization yields no scoreable signal —
            // return None rather than a misleading perfect score.
            if total == 0 {
                return None;
            }
            let found = truth_set
                .iter()
                .filter(|t| extracted_set.contains(*t))
                .count();
            let coverage = found as f64 / total as f64;
            (Some(coverage), found, total)
        }
    };

    // --- noise_rejection: 1.0 - fraction of lie tokens found in extraction ---
    //
    // When lie_text is None we do NOT default to 1.0 — that would manufacture
    // a phantom perfect noise-rejection score. The quality_score computation
    // below uses only the available signal.
    let (noise_rejection, lie_tokens_found, lie_tokens_total) = match lie_text {
        None => (None, 0_usize, 0_usize),
        Some(text) => {
            let lie_tokens = tokenize(text);
            let lie_set = token_set(&lie_tokens);
            let total = lie_set.len();
            if total == 0 {
                tracing::warn!(
                    "lie_text is Some but tokenizes to zero tokens — noise_rejection \
                     defaults to 1.0, which may be misleading"
                );
                (Some(1.0_f64), 0, 0)
            } else {
                let found = lie_set
                    .iter()
                    .filter(|t| extracted_set.contains(*t))
                    .count();
                let rejection = 1.0 - (found as f64 / total as f64);
                (Some(rejection), found, total)
            }
        }
    };

    // Compute quality_score using only the signals that are actually present.
    // Mixing a real score with a phantom 1.0 default would bias the result.
    let quality_score = match (truth_coverage, noise_rejection) {
        (Some(tc), Some(nr)) => {
            let denom = tc + nr;
            if denom == 0.0 { 0.0 } else { 2.0 * tc * nr / denom }
        }
        (Some(tc), None) => tc,
        (None, Some(nr)) => nr,
        // Both None is ruled out by the early return at the top.
        (None, None) => return None,
    };

    // Unwrap with safe fallbacks: None means "not provided" → use 0.0/1.0 for
    // the individual metric fields so downstream consumers still get meaningful
    // numbers for the one-sided case.
    let truth_coverage_val = truth_coverage.unwrap_or(0.0);
    let noise_rejection_val = noise_rejection.unwrap_or(1.0);

    Some(ScrapeQualityMetrics {
        truth_coverage: truth_coverage_val,
        noise_rejection: noise_rejection_val,
        quality_score,
        truth_found: truth_coverage_val >= TRUTH_FOUND_THRESHOLD,
        lie_rejected: noise_rejection_val >= LIE_REJECTED_THRESHOLD,
        truth_tokens_found,
        truth_tokens_total,
        lie_tokens_found,
        lie_tokens_total,
    })
}

/// Normalise and tokenise `text` for token-level comparison.
///
/// Steps applied to each whitespace-delimited token:
/// 1. Lowercase the entire input.
/// 2. Strip leading and trailing non-alphanumeric characters.
/// 3. Discard empty tokens and tokens that are purely non-alphanumeric.
///
/// Embedded punctuation such as `"3.14"` is preserved because its period is
/// surrounded by alphanumeric characters. Trailing punctuation is stripped by
/// `trim_matches`, so `"hello,"` becomes `"hello"`, and `",hello,"` also
/// becomes `"hello"`. Only punctuation flanked by alphanumeric characters on
/// both sides (e.g. `"say,hello"`) survives stripping.
pub(crate) fn tokenize(text: &str) -> Vec<String> {
    text.to_lowercase()
        .split_whitespace()
        .filter_map(|raw| {
            let stripped = raw
                .trim_matches(|c: char| !c.is_alphanumeric())
                .to_owned();
            if stripped.is_empty() || stripped.chars().all(|c| !c.is_alphanumeric()) {
                None
            } else {
                Some(stripped)
            }
        })
        .collect()
}

/// Build a deduplicated lookup set from a token slice.
///
/// Returns references into the original slice, so the slice must outlive the
/// returned set.
pub(crate) fn token_set(tokens: &[String]) -> AHashSet<&str> {
    tokens.iter().map(String::as_str).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    // -----------------------------------------------------------------------
    // tokenize
    // -----------------------------------------------------------------------

    #[test]
    fn test_tokenize_empty_string() {
        assert!(tokenize("").is_empty());
    }

    #[test]
    fn test_tokenize_whitespace_only() {
        assert!(tokenize("   \t\n  ").is_empty());
    }

    #[test]
    fn test_tokenize_lowercases() {
        assert_eq!(tokenize("Hello WORLD"), vec!["hello", "world"]);
    }

    #[test]
    fn test_tokenize_strips_leading_trailing_punctuation() {
        // Leading/trailing non-alphanumeric stripped, inner kept.
        assert_eq!(tokenize("...hello..."), vec!["hello"]);
        assert_eq!(tokenize(",hello,"), vec!["hello"]);
    }

    #[test]
    fn test_tokenize_keeps_embedded_period() {
        // "3.14" — the period is embedded, so it survives.
        assert_eq!(tokenize("3.14"), vec!["3.14"]);
    }

    #[test]
    fn test_tokenize_keeps_embedded_comma() {
        // Outer comma stripped, but inner comma would stay.
        // "hello," → strip trailing "," → "hello"
        assert_eq!(tokenize("hello,"), vec!["hello"]);
        // "say,hello" has no outer non-alphanum — stays as-is.
        assert_eq!(tokenize("say,hello"), vec!["say,hello"]);
    }

    #[test]
    fn test_tokenize_filters_pure_punctuation_tokens() {
        assert_eq!(tokenize("--- !!! ???"), Vec::<String>::new());
    }

    #[test]
    fn test_tokenize_mixed() {
        let result = tokenize("The quick 3.14 fox!");
        assert_eq!(result, vec!["the", "quick", "3.14", "fox"]);
    }

    // -----------------------------------------------------------------------
    // token_set
    // -----------------------------------------------------------------------

    #[test]
    fn test_token_set_deduplicates() {
        let tokens = vec!["apple".to_owned(), "banana".to_owned(), "apple".to_owned()];
        let set = token_set(&tokens);
        assert_eq!(set.len(), 2);
        assert!(set.contains("apple"));
        assert!(set.contains("banana"));
    }

    #[test]
    fn test_token_set_empty() {
        let tokens: Vec<String> = vec![];
        assert!(token_set(&tokens).is_empty());
    }

    // -----------------------------------------------------------------------
    // compute_scrape_quality — None cases
    // -----------------------------------------------------------------------

    #[test]
    fn test_returns_none_when_no_ground_truth() {
        assert!(compute_scrape_quality("some content", None, None).is_none());
    }

    // -----------------------------------------------------------------------
    // compute_scrape_quality — truth only
    // -----------------------------------------------------------------------

    #[test]
    fn test_full_truth_match() {
        let metrics =
            compute_scrape_quality("the quick brown fox", Some("quick brown fox"), None).unwrap();

        assert_eq!(metrics.truth_coverage, 1.0);
        // No lie text provided — noise_rejection field shows 1.0 (display fallback),
        // and quality_score equals truth_coverage alone (no phantom harmonic mean).
        assert_eq!(metrics.noise_rejection, 1.0);
        assert_eq!(metrics.quality_score, 1.0);
        assert_eq!(metrics.truth_tokens_total, 3);
        assert_eq!(metrics.truth_tokens_found, 3);
        assert!(metrics.truth_found);
        assert!(metrics.lie_rejected);
    }

    #[test]
    fn test_no_truth_match() {
        let metrics =
            compute_scrape_quality("completely different text", Some("quick brown fox"), None)
                .unwrap();

        assert_eq!(metrics.truth_coverage, 0.0);
        assert_eq!(metrics.truth_tokens_found, 0);
        assert!(!metrics.truth_found);
    }

    #[test]
    fn test_partial_truth_match() {
        // extracted has 2 of 4 truth tokens
        let metrics = compute_scrape_quality(
            "quick fox runs fast",
            Some("quick brown fox jumps"),
            None,
        )
        .unwrap();

        assert_eq!(metrics.truth_tokens_total, 4);
        assert_eq!(metrics.truth_tokens_found, 2);
        assert!((metrics.truth_coverage - 0.5).abs() < f64::EPSILON);
        // exactly at threshold → truth_found is true
        assert!(metrics.truth_found);
    }

    #[test]
    fn test_truth_case_insensitive() {
        let metrics =
            compute_scrape_quality("QUICK BROWN FOX", Some("quick brown fox"), None).unwrap();
        assert_eq!(metrics.truth_coverage, 1.0);
    }

    #[test]
    fn test_empty_truth_text_returns_none() {
        // Empty truth_text after tokenization has no scoreable signal → None.
        let metrics = compute_scrape_quality("some extracted content", Some(""), None);
        assert!(metrics.is_none());
    }

    // -----------------------------------------------------------------------
    // compute_scrape_quality — lie only
    // -----------------------------------------------------------------------

    #[test]
    fn test_lie_fully_rejected() {
        let metrics =
            compute_scrape_quality("completely different content", None, Some("noise garbage spam"))
                .unwrap();

        assert_eq!(metrics.lie_tokens_found, 0);
        assert_eq!(metrics.noise_rejection, 1.0);
        assert!(metrics.lie_rejected);
    }

    #[test]
    fn test_lie_fully_present() {
        let metrics =
            compute_scrape_quality("noise garbage spam here", None, Some("noise garbage spam"))
                .unwrap();

        assert_eq!(metrics.lie_tokens_total, 3);
        assert_eq!(metrics.lie_tokens_found, 3);
        assert_eq!(metrics.noise_rejection, 0.0);
        assert!(!metrics.lie_rejected);
    }

    #[test]
    fn test_lie_partially_present() {
        // 1 of 2 lie tokens found → noise_rejection = 1 - 0.5 = 0.5
        let metrics =
            compute_scrape_quality("noise clean content", None, Some("noise garbage")).unwrap();

        assert_eq!(metrics.lie_tokens_total, 2);
        assert_eq!(metrics.lie_tokens_found, 1);
        assert!((metrics.noise_rejection - 0.5).abs() < f64::EPSILON);
        // exactly at threshold → lie_rejected is true
        assert!(metrics.lie_rejected);
    }

    #[test]
    fn test_empty_lie_text_gives_full_noise_rejection() {
        let metrics = compute_scrape_quality("some content", None, Some("")).unwrap();
        assert_eq!(metrics.noise_rejection, 1.0);
        assert_eq!(metrics.lie_tokens_total, 0);
    }

    // -----------------------------------------------------------------------
    // compute_scrape_quality — combined truth + lie
    // -----------------------------------------------------------------------

    #[test]
    fn test_combined_full_match_no_lie() {
        let metrics = compute_scrape_quality(
            "the quick brown fox jumps",
            Some("quick brown fox"),
            Some("spam noise garbage"),
        )
        .unwrap();

        assert_eq!(metrics.truth_coverage, 1.0);
        assert_eq!(metrics.noise_rejection, 1.0);
        assert!(metrics.quality_score > 0.99);
    }

    #[test]
    fn test_combined_partial_match_with_some_lie() {
        // truth: "alpha beta gamma delta" (4 tokens); extracted has 2 → truth_coverage = 0.5
        // lie: "spam junk" (2 tokens); extracted has 1 → noise_rejection = 0.5
        // quality_score = 2 * 0.5 * 0.5 / (0.5 + 0.5) = 0.5
        let metrics = compute_scrape_quality(
            "alpha beta spam extra words",
            Some("alpha beta gamma delta"),
            Some("spam junk"),
        )
        .unwrap();

        assert_eq!(metrics.truth_tokens_total, 4);
        assert_eq!(metrics.truth_tokens_found, 2);
        assert_eq!(metrics.lie_tokens_total, 2);
        assert_eq!(metrics.lie_tokens_found, 1);
        assert!((metrics.truth_coverage - 0.5).abs() < f64::EPSILON);
        assert!((metrics.noise_rejection - 0.5).abs() < f64::EPSILON);
        assert!((metrics.quality_score - 0.5).abs() < f64::EPSILON);
    }

    #[test]
    fn test_quality_score_is_zero_when_both_zero() {
        // truth_coverage = 0 and noise_rejection = 0 → denominator is 0 → quality_score = 0
        let metrics = compute_scrape_quality(
            "completely irrelevant",
            Some("truth tokens here"),
            Some("completely irrelevant"), // all lie tokens present → noise_rejection = 0
        )
        .unwrap();

        assert_eq!(metrics.truth_coverage, 0.0);
        assert_eq!(metrics.noise_rejection, 0.0);
        assert_eq!(metrics.quality_score, 0.0);
    }

    #[test]
    fn test_noise_rejection_mixed_case() {
        // lie tokens should match case-insensitively
        let metrics =
            compute_scrape_quality("SPAM HERE", None, Some("spam noise")).unwrap();

        assert_eq!(metrics.lie_tokens_found, 1); // "spam" found (via lowercasing)
        assert_eq!(metrics.lie_tokens_total, 2);
        assert!((metrics.noise_rejection - 0.5).abs() < f64::EPSILON);
    }

    #[test]
    fn test_truth_deduplication() {
        // Duplicate truth tokens count only once in the set.
        let metrics =
            compute_scrape_quality("fox", Some("fox fox fox"), None).unwrap();

        assert_eq!(metrics.truth_tokens_total, 1); // deduplicated to 1 unique token
        assert_eq!(metrics.truth_tokens_found, 1);
        assert_eq!(metrics.truth_coverage, 1.0);
    }

    #[test]
    fn test_extracted_content_empty() {
        let metrics = compute_scrape_quality(
            "",
            Some("important content here"),
            Some("spam noise"),
        )
        .unwrap();

        assert_eq!(metrics.truth_coverage, 0.0);
        assert_eq!(metrics.noise_rejection, 1.0); // no lie tokens found in empty content
        assert_eq!(metrics.truth_tokens_found, 0);
        assert_eq!(metrics.lie_tokens_found, 0);
    }

    #[test]
    fn test_punctuation_stripped_in_truth_matching() {
        // "fox!" in extracted should match "fox" in truth after stripping.
        let metrics =
            compute_scrape_quality("quick fox! jumps", Some("quick fox jumps"), None).unwrap();

        assert_eq!(metrics.truth_coverage, 1.0);
    }

    // -----------------------------------------------------------------------
    // strip_markdown
    // -----------------------------------------------------------------------

    #[test]
    fn test_strip_markdown_headings() {
        assert_eq!(strip_markdown("# Title\n## Subtitle"), "Title\nSubtitle");
    }

    #[test]
    fn test_strip_markdown_bold_italic() {
        assert_eq!(strip_markdown("**bold** and __also bold__"), "bold and also bold");
        assert_eq!(strip_markdown("*italic*"), "italic");
    }

    #[test]
    fn test_strip_markdown_links() {
        assert_eq!(strip_markdown("[click here](https://example.com)"), "click here");
    }

    #[test]
    fn test_strip_markdown_images() {
        assert_eq!(strip_markdown("![alt text](https://example.com/img.png)"), "alt text");
    }

    #[test]
    fn test_strip_markdown_inline_code() {
        assert_eq!(strip_markdown("use `foo()` here"), "use  here");
    }

    #[test]
    fn test_strip_markdown_fenced_code() {
        let input = "intro\n```\ncode block\n```\noutro";
        let result = strip_markdown(input);
        assert!(!result.contains("code block"));
        assert!(result.contains("intro"));
        assert!(result.contains("outro"));
    }

    #[test]
    fn test_strip_markdown_horizontal_rule() {
        let result = strip_markdown("before\n---\nafter");
        assert!(!result.contains("---"));
        assert!(result.contains("before"));
        assert!(result.contains("after"));
    }

    #[test]
    fn test_strip_markdown_blockquote() {
        assert_eq!(strip_markdown("> quoted text"), "quoted text");
    }

    #[test]
    fn test_strip_markdown_table_pipes() {
        let result = strip_markdown("| col1 | col2 |");
        assert!(!result.contains('|'));
    }

    #[test]
    fn test_strip_markdown_plain_text_unchanged() {
        let plain = "The quick brown fox jumps over the lazy dog";
        assert_eq!(strip_markdown(plain), plain);
    }

    // -----------------------------------------------------------------------
    // quality_score single-signal semantics
    // -----------------------------------------------------------------------

    #[test]
    fn test_quality_score_truth_only_equals_truth_coverage() {
        // When only truth is provided, quality_score should equal truth_coverage
        // (no phantom noise_rejection=1.0 inflating the harmonic mean).
        let metrics =
            compute_scrape_quality("alpha beta", Some("alpha beta gamma"), None).unwrap();

        // truth_coverage = 2/3 ≈ 0.667
        let expected = 2.0_f64 / 3.0;
        assert!((metrics.truth_coverage - expected).abs() < 1e-9);
        assert!((metrics.quality_score - expected).abs() < 1e-9,
            "quality_score ({}) should equal truth_coverage ({}) when lie is absent",
            metrics.quality_score, expected);
    }

    #[test]
    fn test_quality_score_lie_only_equals_noise_rejection() {
        // When only lie is provided, quality_score should equal noise_rejection.
        let metrics =
            compute_scrape_quality("clean content here", None, Some("spam garbage junk")).unwrap();

        assert_eq!(metrics.noise_rejection, 1.0); // none of the lie tokens appear
        assert_eq!(metrics.quality_score, 1.0,
            "quality_score should equal noise_rejection when truth is absent");
    }

    #[test]
    fn test_quality_score_lie_only_partial_rejection() {
        // 1 of 2 lie tokens found → noise_rejection = 0.5 → quality_score = 0.5
        let metrics =
            compute_scrape_quality("spam clean", None, Some("spam garbage")).unwrap();

        assert!((metrics.noise_rejection - 0.5).abs() < f64::EPSILON);
        assert!((metrics.quality_score - 0.5).abs() < f64::EPSILON);
    }

    // -----------------------------------------------------------------------
    // strip_markdown integration with scoring
    // -----------------------------------------------------------------------

    #[test]
    fn test_markdown_syntax_not_scored_as_tokens() {
        // Markdown heading marker "#" and bold "**" should not count as tokens.
        // Without stripping, "##" would be a token in extracted and could match
        // nothing in truth — but more importantly, stripping prevents phantom
        // matches on punctuation tokens.
        let extracted = "## Title\n**bold content** here";
        let truth = "Title bold content here";
        let metrics = compute_scrape_quality(extracted, Some(truth), None).unwrap();
        // After stripping markdown, all truth tokens appear in extracted.
        assert_eq!(metrics.truth_coverage, 1.0);
    }
}

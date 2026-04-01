use std::collections::HashMap;

use anyhow::{Context, Result, bail};
use camino::Utf8Path;
use serde::Deserialize;

/// A single E2E test fixture describing a crawl scenario.
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Fixture {
    /// Unique identifier for this fixture (e.g. "scrape_basic_html_page").
    pub id: String,
    /// Human-readable description of what this test validates.
    pub description: String,
    /// Tags for filtering and grouping (e.g. ["scrape", "metadata", "smoke"]).
    #[serde(default)]
    pub tags: Vec<String>,
    /// Category determines the test approach (scrape, crawl, map, robots, sitemap, etc.).
    #[serde(default)]
    pub category: Option<String>,
    /// HTTP request specification (optional — defaults to GET on mock server root).
    #[serde(default)]
    pub request: Option<RequestSpec>,
    /// Mock HTTP response to serve (single-route scenarios).
    #[serde(default)]
    pub mock_response: Option<MockResponseSpec>,
    /// Multiple mock routes for crawl/map scenarios.
    #[serde(default)]
    pub mock_responses: Option<Vec<MockRouteSpec>>,
    /// Crawl configuration overrides.
    #[serde(default)]
    pub config: Option<CrawlConfigSpec>,
    /// Assertions to validate against the crawl result.
    #[serde(default)]
    pub assertions: Option<Assertions>,
    /// Skip directives for conditional test execution.
    #[serde(default)]
    pub skip: Option<SkipDirective>,
}

/// A single mock route for multi-route crawl scenarios.
#[derive(Debug, Deserialize)]
pub struct MockRouteSpec {
    pub path: String,
    #[serde(default = "default_method")]
    pub method: String,
    #[serde(default = "default_status")]
    pub status_code: u16,
    #[serde(default)]
    pub headers: HashMap<String, String>,
    pub body_file: Option<String>,
    pub body_inline: Option<String>,
}

impl Fixture {
    /// Returns the category (resolved during load).
    ///
    /// # Panics
    ///
    /// Panics if called before `load_fixtures` has resolved the category.
    /// This is a programming error, not a user error.
    pub fn category(&self) -> &str {
        self.category
            .as_deref()
            .expect("category should be resolved during load")
    }
}

/// HTTP request specification for the fixture.
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct RequestSpec {
    pub url: String,
    #[serde(default = "default_method")]
    pub method: String,
    #[serde(default)]
    pub headers: HashMap<String, String>,
}

fn default_method() -> String {
    "GET".to_owned()
}

/// Mock HTTP response specification.
#[derive(Debug, Deserialize)]
pub struct MockResponseSpec {
    #[serde(default = "default_status")]
    pub status_code: u16,
    #[serde(default)]
    pub headers: HashMap<String, String>,
    /// Path to a response body file relative to fixtures/responses/.
    pub body_file: Option<String>,
    /// Inline response body string.
    pub body_inline: Option<String>,
}

fn default_status() -> u16 {
    200
}

/// Crawl configuration overrides for the test.
#[derive(Debug, Deserialize)]
pub struct CrawlConfigSpec {
    pub max_depth: Option<u32>,
    pub max_pages: Option<u32>,
    pub respect_robots_txt: Option<bool>,
    pub request_timeout_ms: Option<u64>,
    pub user_agent: Option<String>,
    pub stay_on_domain: Option<bool>,
    pub include_paths: Option<Vec<String>>,
    pub exclude_paths: Option<Vec<String>>,
    pub allow_subdomains: Option<bool>,
    pub max_redirects: Option<u32>,
    pub max_body_size: Option<u64>,
    pub max_concurrent: Option<u32>,
    pub custom_headers: Option<HashMap<String, String>>,
    pub cookies_enabled: Option<bool>,
    pub auth_basic: Option<AuthBasicSpec>,
    pub auth_bearer: Option<String>,
    pub auth_header: Option<AuthHeaderSpec>,
    pub retry_count: Option<u32>,
    pub retry_codes: Option<Vec<u16>>,
    pub remove_tags: Option<Vec<String>>,
    pub main_content_only: Option<bool>,
    pub map_search: Option<String>,
    pub map_limit: Option<u32>,
    pub download_assets: Option<bool>,
    pub asset_types: Option<Vec<String>>,
    pub max_asset_size: Option<u64>,
    pub batch_urls: Option<Vec<String>>,
    pub browser_mode: Option<String>,
    pub browser_endpoint: Option<String>,
    pub browser_timeout_ms: Option<u64>,
    pub browser_wait: Option<String>,
    pub browser_wait_selector: Option<String>,
    pub browser_extra_wait_ms: Option<u64>,
    /// When true, use CrawlEngine::builder() instead of free functions.
    pub engine_mode: Option<bool>,
    /// Crawl strategy: "bfs" (default), "dfs", or "best_first"
    pub crawl_strategy: Option<String>,
    /// Per-domain rate limit delay in milliseconds for PerDomainThrottle.
    pub rate_limit_delay_ms: Option<u64>,
    /// Content filter: "none" (default) or "bm25"
    pub content_filter: Option<String>,
    /// Query string for BM25 content filter.
    pub bm25_query: Option<String>,
    /// Minimum BM25 score threshold (0.0-1.0). Pages below this are filtered out.
    pub bm25_threshold: Option<f64>,
}

/// Basic auth credentials.
#[derive(Debug, Deserialize)]
pub struct AuthBasicSpec {
    pub username: String,
    pub password: String,
}

/// Custom auth header (name, value).
#[derive(Debug, Deserialize)]
pub struct AuthHeaderSpec {
    pub name: String,
    pub value: String,
}

/// Structured assertions validated against crawl results.
#[derive(Debug, Default, Deserialize)]
pub struct Assertions {
    pub status_code: Option<u16>,
    pub content_type: Option<String>,
    pub html_not_empty: Option<bool>,
    pub metadata: Option<MetadataAssertions>,
    pub links: Option<LinkAssertions>,
    pub images: Option<ImageAssertions>,
    pub og: Option<OgAssertions>,
    pub twitter: Option<TwitterAssertions>,
    pub dublin_core: Option<DublinCoreAssertions>,
    pub json_ld: Option<JsonLdAssertions>,
    pub feeds: Option<FeedAssertions>,
    pub robots: Option<RobotsAssertions>,
    pub sitemap: Option<SitemapAssertions>,
    pub crawl: Option<CrawlAssertions>,
    pub error: Option<ErrorAssertions>,
    pub redirect: Option<RedirectAssertions>,
    pub content: Option<ContentAssertions>,
    pub cookies: Option<CookieAssertions>,
    pub auth: Option<AuthAssertions>,
    pub map: Option<MapAssertions>,
    pub extended_metadata: Option<ExtendedMetadataAssertions>,
    pub article: Option<ArticleAssertions>,
    pub extended_og: Option<ExtendedOgAssertions>,
    pub hreflang: Option<HreflangAssertions>,
    pub favicons: Option<FaviconAssertions>,
    pub headings: Option<HeadingAssertions>,
    pub computed: Option<ComputedAssertions>,
    pub response_meta: Option<ResponseMetaAssertions>,
    pub assets: Option<AssetAssertions>,
    pub stream: Option<StreamAssertions>,
    pub batch: Option<BatchAssertions>,
    pub browser: Option<BrowserAssertions>,
    pub strategy: Option<StrategyAssertions>,
    pub rate_limit: Option<RateLimitAssertions>,
    pub filter: Option<FilterAssertions>,
    pub validation: Option<ValidationAssertions>,
}

#[derive(Debug, Deserialize)]
pub struct MetadataAssertions {
    pub title: Option<String>,
    pub description_contains: Option<String>,
    pub has_canonical_url: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct LinkAssertions {
    pub min_count: Option<usize>,
    pub max_count: Option<usize>,
    #[serde(default)]
    pub has_type: Vec<String>,
    pub contains_url: Option<String>,
    pub excludes_url: Option<String>,
    pub has_protocol_relative: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct ImageAssertions {
    pub count: Option<usize>,
    pub min_count: Option<usize>,
}

#[derive(Debug, Deserialize)]
pub struct OgAssertions {
    pub has_og_title: Option<bool>,
    pub og_title: Option<String>,
    pub og_type: Option<String>,
    pub og_image: Option<String>,
    pub has_og_description: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct TwitterAssertions {
    pub has_twitter_card: Option<bool>,
    pub twitter_card_type: Option<String>,
    pub twitter_title: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct DublinCoreAssertions {
    pub has_dc_title: Option<bool>,
    pub dc_title: Option<String>,
    pub dc_creator: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct JsonLdAssertions {
    pub has_json_ld: Option<bool>,
    pub json_ld_type: Option<String>,
    pub json_ld_name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct FeedAssertions {
    pub rss_count: Option<usize>,
    pub atom_count: Option<usize>,
    pub json_feed_count: Option<usize>,
}

#[derive(Debug, Deserialize)]
pub struct RobotsAssertions {
    pub is_allowed: Option<bool>,
    pub crawl_delay: Option<u64>,
    pub noindex_detected: Option<bool>,
    pub nofollow_detected: Option<bool>,
    pub x_robots_tag: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SitemapAssertions {
    pub url_count: Option<usize>,
    pub has_lastmod: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct CrawlAssertions {
    pub pages_crawled: Option<usize>,
    pub min_pages: Option<usize>,
    pub max_pages: Option<usize>,
    pub stayed_on_domain: Option<bool>,
    pub unique_normalized_urls: Option<usize>,
}

#[derive(Debug, Deserialize)]
pub struct ErrorAssertions {
    pub is_error: Option<bool>,
    pub error_type: Option<String>,
    pub is_waf_blocked: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct RedirectAssertions {
    pub final_url_contains: Option<String>,
    pub redirect_count: Option<usize>,
    pub is_error: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct ContentAssertions {
    pub was_skipped: Option<bool>,
    pub detected_charset: Option<String>,
    pub body_size_within: Option<u64>,
    pub is_pdf: Option<bool>,
    pub main_content_only: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct CookieAssertions {
    pub cookie_count: Option<usize>,
    pub has_cookie: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AuthAssertions {
    pub auth_header_sent: Option<bool>,
    pub status_code: Option<u16>,
}

#[derive(Debug, Deserialize)]
pub struct MapAssertions {
    pub url_count: Option<usize>,
    pub min_urls: Option<usize>,
    pub has_url_containing: Option<String>,
    pub max_urls: Option<usize>,
}

#[derive(Debug, Deserialize)]
pub struct ExtendedMetadataAssertions {
    pub has_keywords: Option<bool>,
    pub keywords_contains: Option<String>,
    pub author: Option<String>,
    pub has_viewport: Option<bool>,
    pub generator: Option<String>,
    pub theme_color: Option<String>,
    pub robots_content: Option<String>,
    pub html_lang: Option<String>,
    pub html_dir: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ArticleAssertions {
    pub published_time: Option<String>,
    pub modified_time: Option<String>,
    pub author: Option<String>,
    pub section: Option<String>,
    pub tag_count: Option<usize>,
}

#[derive(Debug, Deserialize)]
pub struct ExtendedOgAssertions {
    pub og_video: Option<String>,
    pub og_audio: Option<String>,
    pub og_locale_alternate_count: Option<usize>,
}

#[derive(Debug, Deserialize)]
pub struct HreflangAssertions {
    pub count: Option<usize>,
    pub has_lang: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct FaviconAssertions {
    pub count: Option<usize>,
    pub has_apple_touch: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct HeadingAssertions {
    pub h1_count: Option<usize>,
    pub h1_text: Option<String>,
    pub total_count: Option<usize>,
}

#[derive(Debug, Deserialize)]
pub struct ComputedAssertions {
    pub word_count_min: Option<usize>,
    pub word_count_max: Option<usize>,
}

#[derive(Debug, Deserialize)]
pub struct ResponseMetaAssertions {
    pub has_etag: Option<bool>,
    pub has_last_modified: Option<bool>,
    pub server_contains: Option<String>,
    pub content_language: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AssetAssertions {
    pub total_count: Option<usize>,
    pub min_count: Option<usize>,
    pub has_category: Option<String>,
    pub unique_hashes: Option<usize>,
}

#[derive(Debug, Deserialize)]
pub struct StreamAssertions {
    pub event_count_min: Option<usize>,
    pub has_page_event: Option<bool>,
    pub has_complete_event: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct BatchAssertions {
    pub completed_count: Option<usize>,
    pub failed_count: Option<usize>,
    pub total_count: Option<usize>,
    pub has_url_result: Option<String>,
}

/// Content filter assertions.
#[derive(Debug, Deserialize)]
pub struct FilterAssertions {
    /// Number of pages remaining after filtering.
    pub pages_after_filter: Option<usize>,
    /// All remaining pages contain this keyword.
    pub remaining_contain_keyword: Option<String>,
}

/// Validation error assertions.
#[derive(Debug, Deserialize)]
pub struct ValidationAssertions {
    /// Expected error message substring.
    pub error_contains: Option<String>,
}

/// Rate-limiting assertions.
#[derive(Debug, Deserialize)]
pub struct RateLimitAssertions {
    /// Crawl took at least this many milliseconds (proves rate limiting is working).
    pub min_duration_ms: Option<u64>,
}

/// Strategy-related assertions for crawl ordering.
#[derive(Debug, Deserialize)]
pub struct StrategyAssertions {
    /// Expected page visit order (URL path suffixes in order).
    pub crawl_order: Option<Vec<String>>,
    /// First crawled page URL contains this string.
    pub first_page_url_contains: Option<String>,
    /// Last crawled page URL contains this string.
    pub last_page_url_contains: Option<String>,
}

/// Browser-related assertions.
#[derive(Debug, Deserialize)]
pub struct BrowserAssertions {
    pub js_render_hint: Option<bool>,
    pub browser_used: Option<bool>,
}

/// Skip directives for conditional test execution.
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct SkipDirective {
    #[serde(default)]
    pub requires_network: bool,
    #[serde(default)]
    pub requires_browser: bool,
    pub notes: Option<String>,
}

/// Load all JSON fixture files from the given directory.
///
/// Walks the directory recursively, skipping files named `schema.json`
/// and files/directories prefixed with `_`. Results are sorted by
/// category then id, and duplicate ids are rejected.
pub fn load_fixtures(dir: &Utf8Path) -> Result<Vec<Fixture>> {
    let std_dir: &std::path::Path = dir.as_ref();
    if !std_dir.is_dir() {
        bail!("fixtures directory does not exist: {dir}");
    }

    let mut fixtures = Vec::new();
    let walker = walkdir::WalkDir::new(std_dir).sort_by_file_name();

    for entry in walker {
        let entry = entry.with_context(|| format!("walking {dir}"))?;
        let path = entry.path();

        // Skip directories
        if entry.file_type().is_dir() {
            continue;
        }

        // Only process .json files
        let Some(ext) = path.extension() else {
            continue;
        };
        if ext != "json" {
            continue;
        }

        // Skip schema.json
        let file_name = entry.file_name().to_string_lossy();
        if file_name == "schema.json" {
            continue;
        }

        // Skip _ prefixed files and directories
        if file_name.starts_with('_') {
            continue;
        }
        let skip_underscore = entry
            .path()
            .components()
            .any(|c| c.as_os_str().to_string_lossy().starts_with('_'));
        if skip_underscore {
            continue;
        }

        let content = std::fs::read_to_string(path)
            .with_context(|| format!("reading fixture {}", path.display()))?;
        let mut fixture: Fixture = serde_json::from_str(&content)
            .with_context(|| format!("parsing fixture {}", path.display()))?;

        // Infer category from parent directory name if not set
        if fixture.category.is_none() {
            let category = path
                .parent()
                .and_then(|p| p.file_name())
                .map(|n| n.to_string_lossy().to_string());
            fixture.category = category;
        }
        if fixture.category.is_none() {
            bail!("fixture {} missing category", path.display());
        }

        // Validate fixture id matches filename
        let expected_id = file_name.trim_end_matches(".json");
        if fixture.id != expected_id {
            bail!(
                "fixture {} has id '{}' but filename suggests '{}'",
                path.display(),
                fixture.id,
                expected_id
            );
        }

        // Validate config constraints
        if let Some(ref cfg) = fixture.config {
            let auth_count = [
                cfg.auth_basic.is_some(),
                cfg.auth_bearer.is_some(),
                cfg.auth_header.is_some(),
            ]
            .iter()
            .filter(|&&x| x)
            .count();
            if auth_count > 1 {
                bail!(
                    "fixture {} specifies multiple auth methods (only one allowed)",
                    fixture.id
                );
            }
        }

        // Validate that fixtures have at least one response source (except error/network fixtures)
        if fixture.mock_response.is_none()
            && fixture.mock_responses.is_none()
            && fixture.category.as_deref() != Some("error")
            && fixture.category.as_deref() != Some("validation")
        {
            bail!(
                "fixture {} has no mock_response or mock_responses (required for non-error fixtures)",
                fixture.id
            );
        }

        // Validate that mock_response and mock_responses are not both set
        if fixture.mock_response.is_some() && fixture.mock_responses.is_some() {
            bail!(
                "fixture {} has both mock_response and mock_responses (use one or the other)",
                fixture.id
            );
        }

        fixtures.push(fixture);
    }

    // Sort by category, then by id
    fixtures.sort_by(|a, b| a.category().cmp(b.category()).then_with(|| a.id.cmp(&b.id)));

    // Detect duplicate ids
    let mut seen = HashMap::new();
    for fixture in &fixtures {
        if let Some(prev) = seen.insert(&fixture.id, &fixture.description) {
            bail!(
                "duplicate fixture id '{}': '{}' and '{}'",
                fixture.id,
                prev,
                fixture.description
            );
        }
    }

    Ok(fixtures)
}

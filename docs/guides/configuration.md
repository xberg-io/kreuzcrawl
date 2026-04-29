# Configuration

All kreuzcrawl operations are controlled through `CrawlConfig`. This guide covers every field, its default value, validation rules, and the builder pattern for constructing engines.

## Builder pattern

The engine's trait-based builder (`CrawlEngine::builder()`) is an internal API — it is not re-exported from the crate root. For most use cases, pass a `CrawlConfig` directly to `create_engine`:

```rust
use kreuzcrawl::{CrawlConfig, CrawlEngineHandle, create_engine};

let engine: CrawlEngineHandle = create_engine(Some(CrawlConfig {
    max_depth: Some(3),
    max_pages: Some(100),
    max_concurrent: Some(5),
    stay_on_domain: true,
    respect_robots_txt: true,
    ..Default::default()
}))?;
```

!!! note "Custom strategy and filter types"
    Alternative crawl strategies (`BestFirstStrategy`, `DfsStrategy`, `AdaptiveStrategy`) and content filters (`Bm25Filter`) exist in the crate's `defaults` module, but that module is not part of the public API. Custom strategies and filters are not currently supported for external use until the relevant traits are re-exported.

### Builder methods (internal reference)

These methods exist on `CrawlEngineBuilder` and are used by `create_engine` internally. They are listed here so you know which defaults are in effect:

| Method | Accepts | Default |
|---|---|---|
| `.config(config)` | `CrawlConfig` | `CrawlConfig::default()` |
| `.frontier(impl Frontier)` | URL queue and deduplication | `InMemoryFrontier` |
| `.rate_limiter(impl RateLimiter)` | Per-domain throttling | `PerDomainThrottle` (200ms) |
| `.store(impl CrawlStore)` | Result persistence | `NoopStore` |
| `.event_emitter(impl EventEmitter)` | Lifecycle event handler | `NoopEmitter` |
| `.strategy(impl CrawlStrategy)` | URL selection and scoring | `BfsStrategy` |
| `.content_filter(impl ContentFilter)` | Post-extraction page filter | `NoopFilter` |
| `.cache(impl CrawlCache)` | HTTP response cache | `NoopCache` |

Unset fields use their defaults. `create_engine` calls `.build()` which validates the config and returns `Result<CrawlEngineHandle, CrawlError>`.

## Convenience constructors

For most use cases, `create_engine` plus one of the top-level async functions is all you need:

```rust
use kreuzcrawl::{CrawlConfig, create_engine, scrape, crawl, map_urls};

let engine = create_engine(Some(CrawlConfig {
    max_depth: Some(2),
    ..Default::default()
}))?;

let scrape_result = scrape(&engine, "https://example.com").await?;
let crawl_result = crawl(&engine, "https://example.com").await?;
let map_result = map_urls(&engine, "https://example.com").await?;
```

Passing `None` to `create_engine` uses `CrawlConfig::default()`.

For scraping or crawling multiple URLs in one go, use the batch variants. Each URL runs concurrently; failures are captured per-URL rather than bubbling up as an error:

```rust
use kreuzcrawl::{create_engine, batch_scrape, batch_crawl};

let engine = create_engine(None)?;

let scrape_results = batch_scrape(&engine, vec![
    "https://example.com".into(),
    "https://example.org".into(),
]).await;

let crawl_results = batch_crawl(&engine, vec![
    "https://example.com".into(),
    "https://example.org".into(),
]).await;

// Each entry in the Vec has .url, .result (Option<_>), and .error (Option<String>).
for r in &scrape_results {
    if let Some(err) = &r.error {
        eprintln!("{}: {}", r.url, err);
    }
}
```

## Config validation

`CrawlConfig::validate()` is called automatically during `CrawlEngineBuilder::build()`. It checks:

| Rule | Error |
|---|---|
| `max_concurrent` must not be 0 | `"max_concurrent must be > 0"` |
| `max_pages` must not be 0 | `"max_pages must be > 0"` |
| `max_redirects` must be <= 100 | `"max_redirects must be <= 100"` |
| `request_timeout` must not be zero | `"request_timeout must be > 0"` |
| `browser.wait_selector` required when `browser.wait` is `Selector` | `"browser.wait_selector required when browser.wait is Selector"` |
| All `include_paths` must be valid regex | `"invalid include_path regex '...': ..."` |
| All `exclude_paths` must be valid regex | `"invalid exclude_path regex '...': ..."` |
| All `retry_codes` must be 100-599 | `"invalid retry code: ..."` |

You can also call `validate()` manually:

```rust
let config = CrawlConfig {
    max_concurrent: Some(0),
    ..Default::default()
};

match config.validate() {
    Ok(()) => println!("Valid"),
    Err(e) => eprintln!("Invalid: {}", e),
}
```

## Full field reference

### Crawl scope

| Field | Type | Default | Description |
|---|---|---|---|
| `max_depth` | `Option<usize>` | `None` | Maximum crawl depth (link hops from seed). `None` means 0 (seed only). |
| `max_pages` | `Option<usize>` | `None` | Maximum pages to crawl. `None` means unlimited. Must be > 0 if set. |
| `stay_on_domain` | `bool` | `false` | Restrict crawling to the seed URL's domain. |
| `allow_subdomains` | `bool` | `false` | Allow subdomains when `stay_on_domain` is true. |
| `include_paths` | `Vec<String>` | `[]` | Regex patterns -- only matching URL paths are crawled. Seed URL (depth 0) is always included. |
| `exclude_paths` | `Vec<String>` | `[]` | Regex patterns -- matching URL paths are skipped. |

### HTTP client

| Field | Type | Default | Description |
|---|---|---|---|
| `max_concurrent` | `Option<usize>` | `None` (10) | Maximum concurrent requests. |
| `request_timeout` | `Duration` | 30 seconds | Timeout for individual HTTP requests. Serialized as milliseconds in JSON. |
| `max_redirects` | `usize` | `10` | Maximum redirects to follow. Must be <= 100. |
| `retry_count` | `usize` | `0` | Number of retry attempts for failed requests. |
| `retry_codes` | `Vec<u16>` | `[]` | HTTP status codes that trigger a retry. Each must be 100-599. |
| `max_body_size` | `Option<usize>` | `None` | Maximum response body size in bytes. Responses are truncated. |
| `user_agent` | `Option<String>` | `None` | Custom User-Agent string. |
| `user_agents` | `Vec<String>` | `[]` | User-Agent strings for rotation. When non-empty, overrides `user_agent`. |
| `custom_headers` | `HashMap<String, String>` | `{}` | Extra HTTP headers sent with every request. |
| `cookies_enabled` | `bool` | `false` | Whether to collect and track cookies across requests. |

### Authentication

| Field | Type | Default | Description |
|---|---|---|---|
| `auth` | `Option<AuthConfig>` | `None` | Authentication configuration. |

`AuthConfig` variants:

```rust
// HTTP Basic authentication
AuthConfig::Basic { username: "user".into(), password: "pass".into() }

// Bearer token
AuthConfig::Bearer { token: "your-token".into() }

// Custom header
AuthConfig::Header { name: "X-API-Key".into(), value: "key-value".into() }
```

### Proxy

| Field | Type | Default | Description |
|---|---|---|---|
| `proxy` | `Option<ProxyConfig>` | `None` | Proxy configuration. |

```rust
ProxyConfig {
    url: "http://proxy:8080".to_string(), // or "socks5://proxy:1080"
    username: Some("user".to_string()),
    password: Some("pass".to_string()),
}
```

### Robots and compliance

| Field | Type | Default | Description |
|---|---|---|---|
| `respect_robots_txt` | `bool` | `false` | Fetch and honor robots.txt directives (allow/disallow, crawl-delay, sitemap). |

### Content processing

| Field | Type | Default | Description |
|---|---|---|---|
| `main_content_only` | `bool` | `false` | Extract only the primary content, stripping navigation and boilerplate. |
| `remove_tags` | `Vec<String>` | `[]` | CSS selectors for elements to remove before processing (e.g., `"nav"`, `".sidebar"`). |

### URL discovery (map)

| Field | Type | Default | Description |
|---|---|---|---|
| `map_limit` | `Option<usize>` | `None` | Maximum URLs returned by the map operation. |
| `map_search` | `Option<String>` | `None` | Case-insensitive substring filter for map results. |

### Asset downloading

| Field | Type | Default | Description |
|---|---|---|---|
| `download_assets` | `bool` | `false` | Download page assets (CSS, JS, images, etc.). |
| `asset_types` | `Vec<AssetCategory>` | `[]` | Filter for which asset categories to download. Empty means all. |
| `max_asset_size` | `Option<usize>` | `None` | Maximum size per asset download in bytes. |

`AssetCategory` options: `Document`, `Image`, `Audio`, `Video`, `Font`, `Stylesheet`, `Script`, `Archive`, `Data`, `Other`.

### Document downloading

| Field | Type | Default | Description |
|---|---|---|---|
| `download_documents` | `bool` | `true` | Download non-HTML resources (PDF, DOCX, images, code files) instead of skipping. |
| `document_max_size` | `Option<usize>` | 50 MB | Maximum document download size in bytes. |
| `document_mime_types` | `Vec<String>` | `[]` | MIME type allowlist. Empty uses built-in defaults. |

### Browser configuration

| Field | Type | Default | Description |
|---|---|---|---|
| `browser` | `BrowserConfig` | See below | Headless browser fallback settings. |
| `capture_screenshot` | `bool` | `false` | Capture PNG screenshot when browser is used. |
| `browser_profile` | `Option<String>` | `None` | Named browser profile for persistent sessions. |
| `save_browser_profile` | `bool` | `false` | Save browser profile changes on exit. |

#### BrowserConfig fields

| Field | Type | Default | Description |
|---|---|---|---|
| `mode` | `BrowserMode` | `Auto` | When to use the browser: `Auto`, `Always`, or `Never`. |
| `endpoint` | `Option<String>` | `None` | CDP WebSocket endpoint for an external browser instance. |
| `timeout` | `Duration` | 30 seconds | Browser page load timeout. |
| `wait` | `BrowserWait` | `NetworkIdle` | Wait strategy after navigation: `NetworkIdle`, `Selector`, or `Fixed`. |
| `wait_selector` | `Option<String>` | `None` | CSS selector to wait for (required when `wait` is `Selector`). |
| `extra_wait` | `Option<Duration>` | `None` | Additional wait time after the wait condition is met. |

### WARC output

| Field | Type | Default | Description |
|---|---|---|---|
| `warc_output` | `Option<PathBuf>` | `None` | Path to write WARC output. `None` disables WARC output. |

## Serialization

`CrawlConfig` implements `Serialize` and `Deserialize` with `#[serde(deny_unknown_fields)]`. Duration fields are serialized as milliseconds:

```json
{
  "max_depth": 3,
  "max_pages": 100,
  "max_concurrent": 5,
  "stay_on_domain": true,
  "respect_robots_txt": true,
  "request_timeout": 30000,
  "max_redirects": 10,
  "retry_count": 2,
  "retry_codes": [429, 503],
  "include_paths": ["^/docs/"],
  "exclude_paths": ["/admin/"],
  "main_content_only": false,
  "cookies_enabled": false,
  "download_assets": false,
  "download_documents": true,
  "document_max_size": 52428800,
  "capture_screenshot": false,
  "save_browser_profile": false,
  "browser": {
    "mode": "auto",
    "timeout": 30000,
    "wait": "network_idle"
  }
}
```

!!! tip "Loading config from a file"
    Since `CrawlConfig` implements `Deserialize`, you can load it from JSON, TOML, or YAML using the appropriate serde crate.

## Default values summary

| Field | Default |
|---|---|
| `max_depth` | `None` (0 -- seed only) |
| `max_pages` | `None` (unlimited) |
| `max_concurrent` | `None` (10) |
| `respect_robots_txt` | `false` |
| `user_agent` | `None` |
| `stay_on_domain` | `false` |
| `allow_subdomains` | `false` |
| `request_timeout` | 30 seconds |
| `max_redirects` | `10` |
| `retry_count` | `0` |
| `cookies_enabled` | `false` |
| `main_content_only` | `false` |
| `download_assets` | `false` |
| `download_documents` | `true` |
| `document_max_size` | 50 MB |
| `capture_screenshot` | `false` |
| `save_browser_profile` | `false` |
| `browser.mode` | `Auto` |
| `browser.timeout` | 30 seconds |
| `browser.wait` | `NetworkIdle` |

## Default trait implementations

When not overridden via the builder, the engine uses these defaults:

| Trait | Default implementation | Behavior |
|---|---|---|
| `Frontier` | `InMemoryFrontier` | In-memory HashSet for URL deduplication. |
| `RateLimiter` | `PerDomainThrottle` | 200ms minimum interval between requests to the same domain. Respects robots.txt crawl-delay. |
| `CrawlStore` | `NoopStore` | Discards all storage calls. |
| `EventEmitter` | `NoopEmitter` | Discards all events. |
| `CrawlStrategy` | `BfsStrategy` | Breadth-first URL selection. |
| `ContentFilter` | `NoopFilter` | Passes all pages through. |
| `CrawlCache` | `NoopCache` | No caching. |

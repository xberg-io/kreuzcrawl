"""Public API for conversion."""

from __future__ import annotations

from typing import TYPE_CHECKING

import kreuzcrawl._kreuzcrawl as _rust

if TYPE_CHECKING:
    from .options import BatchCrawlResult, BatchScrapeResult, BrowserConfig, CrawlConfig, CrawlEngineHandle, CrawlResult, MapResult, ProxyConfig, ScrapeResult


def _to_rust_browser_config(value: BrowserConfig | None) -> Any:
    """Convert Python BrowserConfig to Rust binding type."""
    if value is None:
        return None
    return _rust.BrowserConfig(
        mode=value.mode,
        endpoint=value.endpoint,
        timeout=value.timeout,
        wait=value.wait,
        wait_selector=value.wait_selector,
        extra_wait=value.extra_wait,
    )


def _to_rust_proxy_config(value: ProxyConfig | None) -> Any:
    """Convert Python ProxyConfig to Rust binding type."""
    if value is None:
        return None
    return _rust.ProxyConfig(
        url=value.url,
        username=value.username,
        password=value.password,
    )


def _to_rust_crawl_config(value: CrawlConfig | None) -> Any:
    """Convert Python CrawlConfig to Rust binding type."""
    if value is None:
        return None
    return _rust.CrawlConfig(
        max_depth=value.max_depth,
        max_pages=value.max_pages,
        max_concurrent=value.max_concurrent,
        respect_robots_txt=value.respect_robots_txt,
        user_agent=value.user_agent,
        stay_on_domain=value.stay_on_domain,
        allow_subdomains=value.allow_subdomains,
        include_paths=value.include_paths,
        exclude_paths=value.exclude_paths,
        custom_headers=value.custom_headers,
        request_timeout=value.request_timeout,
        max_redirects=value.max_redirects,
        retry_count=value.retry_count,
        retry_codes=value.retry_codes,
        cookies_enabled=value.cookies_enabled,
        auth=value.auth,
        max_body_size=value.max_body_size,
        main_content_only=value.main_content_only,
        remove_tags=value.remove_tags,
        map_limit=value.map_limit,
        map_search=value.map_search,
        download_assets=value.download_assets,
        asset_types=value.asset_types,
        max_asset_size=value.max_asset_size,
        browser=_to_rust_browser_config(value.browser),
        proxy=_to_rust_proxy_config(value.proxy),
        user_agents=value.user_agents,
        capture_screenshot=value.capture_screenshot,
        download_documents=value.download_documents,
        document_max_size=value.document_max_size,
        document_mime_types=value.document_mime_types,
        warc_output=value.warc_output,
        browser_profile=value.browser_profile,
        save_browser_profile=value.save_browser_profile,
    )


def create_engine(config: CrawlConfig | None = None) -> CrawlEngineHandle:
    """Create a new crawl engine with the given configuration."""
    _rust_config = _to_rust_crawl_config(config)
    return _rust.create_engine(_rust_config)


def scrape(engine: CrawlEngineHandle, url: str) -> ScrapeResult:
    """Scrape a single URL, returning extracted page data."""
    return _rust.scrape(engine, url)


def crawl(engine: CrawlEngineHandle, url: str) -> CrawlResult:
    """Crawl a website starting from `url`, following links up to the configured depth."""
    return _rust.crawl(engine, url)


def map_urls(engine: CrawlEngineHandle, url: str) -> MapResult:
    """Discover all pages on a website by following links and sitemaps."""
    return _rust.map_urls(engine, url)


def batch_scrape(engine: CrawlEngineHandle, urls: list[str]) -> list[BatchScrapeResult]:
    """Scrape multiple URLs concurrently."""
    return _rust.batch_scrape(engine, urls)


def batch_crawl(engine: CrawlEngineHandle, urls: list[str]) -> list[BatchCrawlResult]:
    """Crawl multiple seed URLs concurrently, each following links to configured depth."""
    return _rust.batch_crawl(engine, urls)



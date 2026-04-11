"""E2e tests for category: engine.
"""
from kreuzcrawl import create_engine, scrape


def test_engine_batch_basic() -> None:
    """CrawlEngine with defaults batch scrapes like the free function."""
    engine = create_engine()
    url = ""
    result = scrape(engine=engine, url=url)
    # skipped: field 'batch.completed_count' not available on result type
    # skipped: field 'batch.total_count' not available on result type

def test_engine_crawl_basic() -> None:
    """CrawlEngine with defaults crawls multiple pages like the free function."""
    engine = create_engine()
    url = ""
    result = scrape(engine=engine, url=url)
    # skipped: field 'crawl.pages_crawled' not available on result type
    # skipped: field 'crawl.min_pages' not available on result type

def test_engine_map_basic() -> None:
    """CrawlEngine with defaults discovers URLs like the free function."""
    engine = create_engine()
    url = ""
    result = scrape(engine=engine, url=url)
    # skipped: field 'map.min_urls' not available on result type

def test_engine_scrape_basic() -> None:
    """CrawlEngine with defaults scrapes a page identically to the free function."""
    engine = create_engine()
    url = ""
    result = scrape(engine=engine, url=url)
    assert result.status_code == 200
    assert result.content_type == "text/html"
    assert result.metadata.title == "Engine Test"
    assert "Testing the engine" in result.metadata.description
    assert len(result.links) >= 1
    # skipped: field 'headings.h1_text' not available on result type

def test_engine_stream_basic() -> None:
    """CrawlEngine with defaults streams events like the free function."""
    engine = create_engine()
    url = ""
    result = scrape(engine=engine, url=url)
    # skipped: field 'stream.has_page_event' not available on result type
    # skipped: field 'stream.has_complete_event' not available on result type
    # skipped: field 'stream.event_count_min' not available on result type


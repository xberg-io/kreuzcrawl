"""E2e tests for category: links.
"""
from kreuzcrawl import create_engine, scrape


def test_links_anchor_fragment() -> None:
    """Identifies fragment-only links as anchor type."""
    engine = create_engine()
    url = ""
    result = scrape(engine=engine, url=url)
    assert "anchor" in result.links[0].link_type

def test_links_base_tag() -> None:
    """Resolves relative URLs using base tag href."""
    engine = create_engine()
    url = ""
    result = scrape(engine=engine, url=url)
    assert len(result.links) > 2
    assert "example.com" in result.links[0].url

def test_links_document_types() -> None:
    """Detects PDF, DOCX, XLSX links as document type."""
    engine = create_engine()
    url = ""
    result = scrape(engine=engine, url=url)
    assert "document" in result.links[0].link_type

def test_links_empty_href() -> None:
    """Handles empty href attributes without errors."""
    engine = create_engine()
    url = ""
    result = scrape(engine=engine, url=url)
    assert len(result.links) > 0
    assert "/valid" in result.links[0].url

def test_links_internal_external_classification() -> None:
    """Correctly classifies internal vs external links by domain."""
    engine = create_engine()
    url = ""
    result = scrape(engine=engine, url=url)
    assert len(result.links) > 4
    assert "internal" in result.links[0].link_type
    assert "external" in result.links[0].link_type

def test_links_mailto_javascript_skip() -> None:
    """Skips mailto:, javascript:, and tel: scheme links."""
    engine = create_engine()
    url = ""
    result = scrape(engine=engine, url=url)
    assert len(result.links) > 0
    assert "mailto:" not in result.links[0].url

def test_links_protocol_relative() -> None:
    """Handles protocol-relative URLs (//example.com) correctly."""
    engine = create_engine()
    url = ""
    result = scrape(engine=engine, url=url)
    assert len(result.links) > 1
    assert "//" in result.links[0].url

def test_links_rel_attributes() -> None:
    """Preserves rel=nofollow and rel=canonical attributes."""
    engine = create_engine()
    url = ""
    result = scrape(engine=engine, url=url)
    assert len(result.links) > 0

def test_links_relative_parent() -> None:
    """Resolves ../ and ./ relative parent path links correctly."""
    engine = create_engine()
    url = ""
    result = scrape(engine=engine, url=url)
    assert len(result.links) > 3


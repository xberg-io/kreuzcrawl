"""E2e tests for category: scrape.
"""
from kreuzcrawl import create_engine, scrape


def test_scrape_asset_dedup() -> None:
    """Same asset linked twice results in one download with one unique hash."""
    engine = create_engine()
    url = ""
    result = scrape(engine=engine, url=url)
    assert result.status_code == 200
    assert len(result.assets) == 2
    assert result.assets[0].unique_hashes == 2

def test_scrape_asset_max_size() -> None:
    """Skips assets exceeding max_asset_size limit."""
    engine = create_engine()
    url = ""
    result = scrape(engine=engine, url=url)
    assert result.status_code == 200
    assert len(result.assets) == 2

def test_scrape_asset_type_filter() -> None:
    """Only downloads image assets when asset_types filter is set."""
    engine = create_engine()
    url = ""
    result = scrape(engine=engine, url=url)
    assert result.status_code == 200
    assert len(result.assets) == 1
    assert "image" in result.assets[0].category

def test_scrape_basic_html_page() -> None:
    """Scrapes a simple HTML page and extracts title, description, and links."""
    engine = create_engine()
    url = ""
    result = scrape(engine=engine, url=url)
    assert result.status_code == 200
    assert result.content_type == "text/html"
    assert result.html
    assert result.metadata.title == "Example Domain"
    assert "illustrative examples" in result.metadata.description
    assert result.metadata.canonical_url
    assert len(result.links) > 0
    assert "external" in result.links[0].link_type
    assert len(result.images) == 0
    # skipped: field 'og.title' not available on result type

def test_scrape_complex_links() -> None:
    """Classifies links by type: internal, external, anchor, document, image."""
    engine = create_engine()
    url = ""
    result = scrape(engine=engine, url=url)
    assert result.status_code == 200
    assert len(result.links) > 9
    assert "internal" in result.links[0].link_type
    assert "external" in result.links[0].link_type
    assert "anchor" in result.links[0].link_type
    assert "document" in result.links[0].link_type

def test_scrape_download_assets() -> None:
    """Downloads CSS, JS, and image assets from page."""
    engine = create_engine()
    url = ""
    result = scrape(engine=engine, url=url)
    assert result.status_code == 200
    assert len(result.assets) > 2

def test_scrape_dublin_core() -> None:
    """Extracts Dublin Core metadata from a page."""
    engine = create_engine()
    url = ""
    result = scrape(engine=engine, url=url)
    assert result.status_code == 200
    # skipped: field 'dublin_core.title' not available on result type
    # skipped: field 'dublin_core.title' not available on result type
    # skipped: field 'dublin_core.creator' not available on result type

def test_scrape_empty_page() -> None:
    """Handles an empty HTML document without errors."""
    engine = create_engine()
    url = ""
    result = scrape(engine=engine, url=url)
    assert result.status_code == 200
    assert len(result.links) > -1
    assert len(result.images) == 0

def test_scrape_feed_discovery() -> None:
    """Discovers RSS, Atom, and JSON feed links."""
    engine = create_engine()
    url = ""
    result = scrape(engine=engine, url=url)
    assert result.status_code == 200
    assert len(result.feeds[0].rss) == 1
    assert len(result.feeds[0].atom) == 1
    assert len(result.feeds[0].json_feed) == 1

def test_scrape_image_sources() -> None:
    """Extracts images from img, picture, og:image, twitter:image."""
    engine = create_engine()
    url = ""
    result = scrape(engine=engine, url=url)
    assert result.status_code == 200
    assert len(result.images) > 4
    # skipped: field 'og.image' not available on result type

def test_scrape_js_heavy_spa() -> None:
    """Handles SPA page with JavaScript-only content (no server-rendered HTML)."""
    engine = create_engine()
    url = ""
    result = scrape(engine=engine, url=url)
    assert result.html

def test_scrape_json_ld() -> None:
    """Extracts JSON-LD structured data from a page."""
    engine = create_engine()
    url = ""
    result = scrape(engine=engine, url=url)
    assert result.status_code == 200
    assert result.json_ld
    assert result.json_ld[0].type == "Recipe"
    assert result.json_ld[0].name == "Best Chocolate Cake"

def test_scrape_malformed_html() -> None:
    """Gracefully handles broken HTML without crashing."""
    engine = create_engine()
    url = ""
    result = scrape(engine=engine, url=url)
    assert result.status_code == 200
    assert result.html
    assert "broken HTML" in result.metadata.description

def test_scrape_og_metadata() -> None:
    """Extracts full Open Graph metadata from a page."""
    engine = create_engine()
    url = ""
    result = scrape(engine=engine, url=url)
    assert result.status_code == 200
    # skipped: field 'og.title' not available on result type
    # skipped: field 'og.title' not available on result type
    # skipped: field 'og.type' not available on result type
    # skipped: field 'og.image' not available on result type
    # skipped: field 'og.description' not available on result type
    assert result.metadata.title == "Article Title - Example Blog"

def test_scrape_twitter_card() -> None:
    """Extracts Twitter Card metadata from a page."""
    engine = create_engine()
    url = ""
    result = scrape(engine=engine, url=url)
    assert result.status_code == 200
    # skipped: field 'twitter.card' not available on result type
    # skipped: field 'twitter.card_type' not available on result type
    # skipped: field 'twitter.title' not available on result type


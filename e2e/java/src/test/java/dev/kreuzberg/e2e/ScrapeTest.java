package dev.kreuzberg.e2e;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;

/** E2e tests for category: scrape. */
class ScrapeTest {
    @Test
    void testScrapeAssetDedup() throws Exception {
        // Same asset linked twice results in one download with one unique hash
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertEquals(200, result.statusCode());
        assertEquals(2, result.assets().size());
        assertEquals(2, result.assets().getFirst().uniqueHashes());
    }

    @Test
    void testScrapeAssetMaxSize() throws Exception {
        // Skips assets exceeding max_asset_size limit
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertEquals(200, result.statusCode());
        assertEquals(2, result.assets().size());
    }

    @Test
    void testScrapeAssetTypeFilter() throws Exception {
        // Only downloads image assets when asset_types filter is set
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertEquals(200, result.statusCode());
        assertEquals(1, result.assets().size());
        assertTrue(result.assets().getFirst().category().contains("image"), "expected to contain: " + "image");
    }

    @Test
    void testScrapeBasicHtmlPage() throws Exception {
        // Scrapes a simple HTML page and extracts title, description, and links
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertEquals(200, result.statusCode());
        assertEquals("text/html", result.contentType());
        assertFalse(result.html().isEmpty(), "expected non-empty value");
        assertEquals("Example Domain", result.metadata().orElseThrow().title().orElse(""));
        assertTrue(result.metadata().orElseThrow().description().orElse("").contains("illustrative examples"), "expected to contain: " + "illustrative examples");
        assertFalse(result.metadata().orElseThrow().canonicalUrl().orElse("").isEmpty(), "expected non-empty value");
        assertTrue(result.links().size() > 0, "expected > 0");
        assertTrue(result.links().getFirst().linkType().contains("external"), "expected to contain: " + "external");
        assertEquals(0, result.images().size());
        // skipped: field 'og.title' not available on result type
    }

    @Test
    void testScrapeComplexLinks() throws Exception {
        // Classifies links by type: internal, external, anchor, document, image
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertEquals(200, result.statusCode());
        assertTrue(result.links().size() > 9, "expected > 9");
        assertTrue(result.links().getFirst().linkType().contains("internal"), "expected to contain: " + "internal");
        assertTrue(result.links().getFirst().linkType().contains("external"), "expected to contain: " + "external");
        assertTrue(result.links().getFirst().linkType().contains("anchor"), "expected to contain: " + "anchor");
        assertTrue(result.links().getFirst().linkType().contains("document"), "expected to contain: " + "document");
    }

    @Test
    void testScrapeDownloadAssets() throws Exception {
        // Downloads CSS, JS, and image assets from page
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertEquals(200, result.statusCode());
        assertTrue(result.assets().size() > 2, "expected > 2");
    }

    @Test
    void testScrapeDublinCore() throws Exception {
        // Extracts Dublin Core metadata from a page
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertEquals(200, result.statusCode());
        // skipped: field 'dublin_core.title' not available on result type
        // skipped: field 'dublin_core.title' not available on result type
        // skipped: field 'dublin_core.creator' not available on result type
    }

    @Test
    void testScrapeEmptyPage() throws Exception {
        // Handles an empty HTML document without errors
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertEquals(200, result.statusCode());
        assertTrue(result.links().size() > -1, "expected > -1");
        assertEquals(0, result.images().size());
    }

    @Test
    void testScrapeFeedDiscovery() throws Exception {
        // Discovers RSS, Atom, and JSON feed links
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertEquals(200, result.statusCode());
        assertEquals(1, result.feeds().getFirst().rss().size());
        assertEquals(1, result.feeds().getFirst().atom().size());
        assertEquals(1, result.feeds().getFirst().jsonFeed().size());
    }

    @Test
    void testScrapeImageSources() throws Exception {
        // Extracts images from img, picture, og:image, twitter:image
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertEquals(200, result.statusCode());
        assertTrue(result.images().size() > 4, "expected > 4");
        // skipped: field 'og.image' not available on result type
    }

    @Test
    void testScrapeJsHeavySpa() throws Exception {
        // Handles SPA page with JavaScript-only content (no server-rendered HTML)
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertFalse(result.html().isEmpty(), "expected non-empty value");
    }

    @Test
    void testScrapeJsonLd() throws Exception {
        // Extracts JSON-LD structured data from a page
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertEquals(200, result.statusCode());
        assertFalse(result.jsonLd().isEmpty(), "expected non-empty value");
        assertEquals("Recipe", result.jsonLd().getFirst().type());
        assertEquals("Best Chocolate Cake", result.jsonLd().getFirst().name());
    }

    @Test
    void testScrapeMalformedHtml() throws Exception {
        // Gracefully handles broken HTML without crashing
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertEquals(200, result.statusCode());
        assertFalse(result.html().isEmpty(), "expected non-empty value");
        assertTrue(result.metadata().orElseThrow().description().orElse("").contains("broken HTML"), "expected to contain: " + "broken HTML");
    }

    @Test
    void testScrapeOgMetadata() throws Exception {
        // Extracts full Open Graph metadata from a page
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertEquals(200, result.statusCode());
        // skipped: field 'og.title' not available on result type
        // skipped: field 'og.title' not available on result type
        // skipped: field 'og.type' not available on result type
        // skipped: field 'og.image' not available on result type
        // skipped: field 'og.description' not available on result type
        assertEquals("Article Title - Example Blog", result.metadata().orElseThrow().title().orElse(""));
    }

    @Test
    void testScrapeTwitterCard() throws Exception {
        // Extracts Twitter Card metadata from a page
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertEquals(200, result.statusCode());
        // skipped: field 'twitter.card' not available on result type
        // skipped: field 'twitter.card_type' not available on result type
        // skipped: field 'twitter.title' not available on result type
    }

}

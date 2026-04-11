package dev.kreuzberg.e2e;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;
import dev.kreuzberg.kreuzcrawl.Kreuzcrawl;

/** E2e tests for category: scrape. */
class ScrapeTest {
    @Test
    void testScrapeAssetDedup() throws Exception {
        // Same asset linked twice results in one download with one unique hash
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertEquals(200, result.statusCode());
        assertEquals(2, result.assets().size());
        assertFalse(result.assets().getFirst().contentHash().isEmpty(), "expected non-empty value");
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
        assertTrue(result.assets().getFirst().assetCategory().contains("image"), "expected to contain: " + "image");
    }

    @Test
    void testScrapeBasicHtmlPage() throws Exception {
        // Scrapes a simple HTML page and extracts title, description, and links
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertEquals(200, result.statusCode());
        assertEquals("text/html", result.contentType());
        assertFalse(result.html().isEmpty(), "expected non-empty value");
        assertEquals("Example Domain", result.metadata().title().orElse(""));
        assertTrue(result.metadata().description().orElse("").contains("illustrative examples"), "expected to contain: " + "illustrative examples");
        assertFalse(result.metadata().canonicalUrl().orElse("").isEmpty(), "expected non-empty value");
        assertTrue(result.links().size() > 0, "expected > 0");
        assertTrue(result.links().getFirst().linkType().contains("external"), "expected to contain: " + "external");
        assertEquals(0, result.images().size());
        assertTrue(result.metadata().ogTitle().orElse("").isEmpty(), "expected empty value");
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
        assertFalse(result.metadata().dcTitle().orElse("").isEmpty(), "expected non-empty value");
        assertEquals("Effects of Climate Change on Marine Biodiversity", result.metadata().dcTitle().orElse(""));
        assertEquals("Dr. Jane Smith", result.metadata().dcCreator().orElse(""));
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
        assertTrue(result.feeds().size() >= 3, "expected >= 3");
    }

    @Test
    void testScrapeImageSources() throws Exception {
        // Extracts images from img, picture, og:image, twitter:image
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertEquals(200, result.statusCode());
        assertTrue(result.images().size() > 4, "expected > 4");
        assertEquals("https://example.com/images/og-hero.jpg", result.metadata().ogImage().orElse(""));
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
        assertEquals("Recipe", result.jsonLd().getFirst().schemaType());
        assertEquals("Best Chocolate Cake", result.jsonLd().getFirst().name().orElse(""));
    }

    @Test
    void testScrapeMalformedHtml() throws Exception {
        // Gracefully handles broken HTML without crashing
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertEquals(200, result.statusCode());
        assertFalse(result.html().isEmpty(), "expected non-empty value");
        assertTrue(result.metadata().description().orElse("").contains("broken HTML"), "expected to contain: " + "broken HTML");
    }

    @Test
    void testScrapeOgMetadata() throws Exception {
        // Extracts full Open Graph metadata from a page
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertEquals(200, result.statusCode());
        assertFalse(result.metadata().ogTitle().orElse("").isEmpty(), "expected non-empty value");
        assertEquals("Article Title", result.metadata().ogTitle().orElse(""));
        assertEquals("article", result.metadata().ogType().orElse(""));
        assertEquals("https://example.com/images/article-hero.jpg", result.metadata().ogImage().orElse(""));
        assertFalse(result.metadata().ogDescription().orElse("").isEmpty(), "expected non-empty value");
        assertEquals("Article Title - Example Blog", result.metadata().title().orElse(""));
    }

    @Test
    void testScrapeTwitterCard() throws Exception {
        // Extracts Twitter Card metadata from a page
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertEquals(200, result.statusCode());
        assertFalse(result.metadata().twitterCard().orElse("").isEmpty(), "expected non-empty value");
        assertEquals("summary_large_image", result.metadata().twitterCard().orElse(""));
        assertEquals("New Product Launch", result.metadata().twitterTitle().orElse(""));
    }

}

package dev.kreuzberg.e2e;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;

/** E2e tests for category: middleware. */
class MiddlewareTest {
    @Test
    void testMiddlewareEngineCrawlWithDefaults() throws Exception {
        // Engine crawl with default middleware chain produces correct multi-page results
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        // skipped: field 'crawl.pages_crawled' not available on result type
        // skipped: field 'crawl.min_pages' not available on result type
    }

    @Test
    void testMiddlewareNoopNoEffect() throws Exception {
        // Default middleware chain does not affect normal scraping
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertEquals(200, result.statusCode());
        assertEquals("Middleware Test", result.metadata().orElseThrow().title().orElse(""));
    }

}

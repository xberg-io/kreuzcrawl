package dev.kreuzberg.e2e;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;

/** E2e tests for category: middleware. */
class MiddlewareTest {
    @Test
    void testMiddlewareEngineCrawlWithDefaults() throws Exception {
        // Engine crawl with default middleware chain produces correct multi-page results
        var result = Kreuzcrawl.scrape();
        assertEquals(3, result.crawl().pages_crawled());
        assertTrue(result.crawl().min_pages() >= 3, "expected >= 3");
    }

    @Test
    void testMiddlewareNoopNoEffect() throws Exception {
        // Default middleware chain does not affect normal scraping
        var result = Kreuzcrawl.scrape();
        assertEquals(200, result.status_code());
        assertEquals("Middleware Test", result.metadata().title().orElse(""));
    }

}

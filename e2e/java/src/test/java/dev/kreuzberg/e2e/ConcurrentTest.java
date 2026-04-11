package dev.kreuzberg.e2e;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;
import dev.kreuzberg.kreuzcrawl.Kreuzcrawl;

/** E2e tests for category: concurrent. */
class ConcurrentTest {
    @Test
    void testConcurrentBasic() throws Exception {
        // Concurrent crawling fetches all pages with max_concurrent workers
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        // skipped: field 'pages.length' not available on result type
        // skipped: field 'pages.length' not available on result type
    }

    @Test
    void testConcurrentDepthTwoFanOut() throws Exception {
        // Concurrent depth=2 crawl correctly fans out and deduplicates across levels
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        // skipped: field 'pages.length' not available on result type
    }

    @Test
    void testConcurrentMaxPagesExact() throws Exception {
        // Concurrent crawling does not exceed max_pages limit even with high concurrency
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        // skipped: field 'pages.length' not available on result type
    }

    @Test
    void testConcurrentPartialErrors() throws Exception {
        // Concurrent crawl handles partial failures gracefully
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        // skipped: field 'pages.length' not available on result type
    }

    @Test
    void testConcurrentRespectsMaxPages() throws Exception {
        // Concurrent crawling respects max_pages limit
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        // skipped: field 'pages.length' not available on result type
    }

}

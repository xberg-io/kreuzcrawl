package dev.kreuzberg.e2e;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;

/** E2e tests for category: rate_limit. */
class RateLimitTest {
    @Test
    void testRateLimitBasicDelay() throws Exception {
        // Rate limiter adds delay between requests to the same domain
        var result = Kreuzcrawl.scrape();
        assertEquals(3, result.crawl().pages_crawled());
        assertTrue(result.rate_limit().min_duration_ms() >= 150, "expected >= 150");
    }

    @Test
    void testRateLimitZeroNoDelay() throws Exception {
        // Rate limiter with zero delay does not slow crawling
        var result = Kreuzcrawl.scrape();
        assertEquals(2, result.crawl().pages_crawled());
    }

}

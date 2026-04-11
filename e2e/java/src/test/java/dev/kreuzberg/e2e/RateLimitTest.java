package dev.kreuzberg.e2e;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;
import dev.kreuzberg.kreuzcrawl.Kreuzcrawl;

/** E2e tests for category: rate_limit. */
class RateLimitTest {
    @Test
    void testRateLimitBasicDelay() throws Exception {
        // Rate limiter adds delay between requests to the same domain
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        // skipped: field 'crawl.pages_crawled' not available on result type
        // skipped: field 'rate_limit.min_duration_ms' not available on result type
    }

    @Test
    void testRateLimitZeroNoDelay() throws Exception {
        // Rate limiter with zero delay does not slow crawling
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        // skipped: field 'crawl.pages_crawled' not available on result type
    }

}

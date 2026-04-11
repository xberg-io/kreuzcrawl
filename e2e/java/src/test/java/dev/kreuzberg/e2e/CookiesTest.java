package dev.kreuzberg.e2e;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;
import dev.kreuzberg.kreuzcrawl.Kreuzcrawl;

/** E2e tests for category: cookies. */
class CookiesTest {
    @Test
    void testCookiesPerDomain() throws Exception {
        // Isolates cookies per domain during crawl
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        // skipped: field 'cookies.length' not available on result type
        // skipped: field 'cookies' not available on result type
    }

    @Test
    void testCookiesPersistence() throws Exception {
        // Maintains cookies across multiple crawl requests
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        // skipped: field 'cookies' not available on result type
    }

    @Test
    void testCookiesSetCookieResponse() throws Exception {
        // Respects Set-Cookie header from server responses
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        // skipped: field 'cookies' not available on result type
    }

}

package dev.kreuzberg.e2e;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;
import dev.kreuzberg.kreuzcrawl.Kreuzcrawl;

/** E2e tests for category: auth. */
class AuthTest {
    @Test
    void testAuthBasicHttp() throws Exception {
        // Sends HTTP Basic authentication header
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertEquals(true, result.authHeaderSent());
        assertEquals(200, result.statusCode());
    }

    @Test
    void testAuthBearerToken() throws Exception {
        // Sends Bearer token in Authorization header
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertEquals(true, result.authHeaderSent());
        assertEquals(200, result.statusCode());
    }

    @Test
    void testAuthCustomHeader() throws Exception {
        // Sends authentication via custom header (X-API-Key)
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertEquals(true, result.authHeaderSent());
        assertEquals(200, result.statusCode());
    }

}

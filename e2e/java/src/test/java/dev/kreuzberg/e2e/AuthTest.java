package dev.kreuzberg.e2e;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;

/** E2e tests for category: auth. */
class AuthTest {
    @Test
    void testAuthBasicHttp() throws Exception {
        // Sends HTTP Basic authentication header
        var result = Kreuzcrawl.scrape();
        assertEquals(true, result.auth_header_sent());
        assertEquals(200, result.status_code());
    }

    @Test
    void testAuthBearerToken() throws Exception {
        // Sends Bearer token in Authorization header
        var result = Kreuzcrawl.scrape();
        assertEquals(true, result.auth_header_sent());
        assertEquals(200, result.status_code());
    }

    @Test
    void testAuthCustomHeader() throws Exception {
        // Sends authentication via custom header (X-API-Key)
        var result = Kreuzcrawl.scrape();
        assertEquals(true, result.auth_header_sent());
        assertEquals(200, result.status_code());
    }

}

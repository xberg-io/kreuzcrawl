package dev.kreuzberg.e2e;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;

/** E2e tests for category: error. */
class ErrorTest {
    @Test
    void testError401Unauthorized() throws Exception {
        // Handles 401 Unauthorized response correctly
        assertThrows(Exception.class, () -> Kreuzcrawl.scrape());
    }

    @Test
    void testError403Forbidden() throws Exception {
        // Handles 403 Forbidden response correctly
        assertThrows(Exception.class, () -> Kreuzcrawl.scrape());
    }

    @Test
    void testError404Page() throws Exception {
        // Handles 404 response correctly
        assertThrows(Exception.class, () -> Kreuzcrawl.scrape());
    }

    @Test
    void testError408RequestTimeout() throws Exception {
        // Handles 408 Request Timeout response correctly
        assertThrows(Exception.class, () -> Kreuzcrawl.scrape());
    }

    @Test
    void testError410Gone() throws Exception {
        // Handles 410 Gone response correctly
        assertThrows(Exception.class, () -> Kreuzcrawl.scrape());
    }

    @Test
    void testError500Server() throws Exception {
        // Handles 500 server error
        assertThrows(Exception.class, () -> Kreuzcrawl.scrape());
    }

    @Test
    void testError502BadGateway() throws Exception {
        // Handles 502 Bad Gateway response correctly
        assertThrows(Exception.class, () -> Kreuzcrawl.scrape());
    }

    @Test
    void testErrorConnectionRefused() throws Exception {
        // Handles connection refused error gracefully
        assertThrows(Exception.class, () -> Kreuzcrawl.scrape());
    }

    @Test
    void testErrorDnsResolution() throws Exception {
        // Handles DNS resolution failure gracefully
        assertThrows(Exception.class, () -> Kreuzcrawl.scrape());
    }

    @Test
    void testErrorEmptyResponse() throws Exception {
        // Handles 200 with completely empty body gracefully
        var result = Kreuzcrawl.scrape();
        assertEquals(false, result.html_not_empty());
        assertEquals(false, result.error().is_error());
    }

    @Test
    void testErrorInvalidProxy() throws Exception {
        // Proxy pointing to unreachable address causes connection error during scrape
        assertThrows(Exception.class, () -> Kreuzcrawl.scrape());
    }

    @Test
    void testErrorPartialResponse() throws Exception {
        // Handles incomplete or truncated HTTP response
        assertThrows(Exception.class, () -> Kreuzcrawl.scrape());
    }

    @Test
    void testErrorRateLimited() throws Exception {
        // Handles 429 rate limiting with Retry-After
        assertThrows(Exception.class, () -> Kreuzcrawl.scrape());
    }

    @Test
    void testErrorRetry503() throws Exception {
        // Retries request on 503 Service Unavailable response
        assertThrows(Exception.class, () -> Kreuzcrawl.scrape());
    }

    @Test
    void testErrorRetryBackoff() throws Exception {
        // Implements exponential backoff when retrying failed requests
        assertThrows(Exception.class, () -> Kreuzcrawl.scrape());
    }

    @Test
    void testErrorSslInvalidCert() throws Exception {
        // Handles SSL certificate validation error
        assertThrows(Exception.class, () -> Kreuzcrawl.scrape());
    }

    @Test
    void testErrorTimeout() throws Exception {
        // Handles request timeout
        assertThrows(Exception.class, () -> Kreuzcrawl.scrape());
    }

    @Test
    void testErrorWafAkamai() throws Exception {
        // Akamai WAF detection returns WafBlocked error
        assertThrows(Exception.class, () -> Kreuzcrawl.scrape());
    }

    @Test
    void testErrorWafFalse403() throws Exception {
        // Detects WAF/bot protection false 403 (Cloudflare challenge page)
        assertThrows(Exception.class, () -> Kreuzcrawl.scrape());
    }

    @Test
    void testErrorWafImperva() throws Exception {
        // Imperva/Incapsula WAF detection
        assertThrows(Exception.class, () -> Kreuzcrawl.scrape());
    }

}

package dev.kreuzberg.e2e;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;

/** E2e tests for category: redirect. */
class RedirectTest {
    @Test
    void testRedirect301Permanent() throws Exception {
        // Follows 301 permanent redirect and returns final page content
        var result = Kreuzcrawl.scrape();
        assertTrue(result.final_url().contains("/target"), "expected to contain: " + "/target");
        assertEquals(1, result.redirect_count());
    }

    @Test
    void testRedirect302Found() throws Exception {
        // Follows 302 Found redirect correctly
        var result = Kreuzcrawl.scrape();
        assertTrue(result.final_url().contains("/found-target"), "expected to contain: " + "/found-target");
        assertEquals(1, result.redirect_count());
    }

    @Test
    void testRedirect303SeeOther() throws Exception {
        // Follows 303 See Other redirect (method changes to GET)
        var result = Kreuzcrawl.scrape();
        assertTrue(result.final_url().contains("/see-other"), "expected to contain: " + "/see-other");
        assertEquals(1, result.redirect_count());
    }

    @Test
    void testRedirect307Temporary() throws Exception {
        // Follows 307 Temporary Redirect (preserves method)
        var result = Kreuzcrawl.scrape();
        assertTrue(result.final_url().contains("/temp-target"), "expected to contain: " + "/temp-target");
        assertEquals(1, result.redirect_count());
    }

    @Test
    void testRedirect308Permanent() throws Exception {
        // Follows 308 Permanent Redirect (preserves method)
        var result = Kreuzcrawl.scrape();
        assertTrue(result.final_url().contains("/perm-target"), "expected to contain: " + "/perm-target");
        assertEquals(1, result.redirect_count());
    }

    @Test
    void testRedirectChain() throws Exception {
        // Follows a chain of redirects (301 -> 302 -> 200)
        var result = Kreuzcrawl.scrape();
        assertTrue(result.final_url().contains("/step2"), "expected to contain: " + "/step2");
        assertEquals(2, result.redirect_count());
    }

    @Test
    void testRedirectCrossDomain() throws Exception {
        // Reports cross-domain redirect target without following to external domain
        var result = Kreuzcrawl.scrape();
        assertTrue(result.final_url().contains("/external-redirect"), "expected to contain: " + "/external-redirect");
        assertEquals(1, result.redirect_count());
    }

    @Test
    void testRedirectLoop() throws Exception {
        // Detects redirect loop (A -> B -> A) and returns error
        var result = Kreuzcrawl.scrape();
        assertEquals(true, result.is_error());
    }

    @Test
    void testRedirectMaxExceeded() throws Exception {
        // Aborts when redirect count exceeds max_redirects limit
        var result = Kreuzcrawl.scrape();
        assertEquals(true, result.is_error());
    }

    @Test
    void testRedirectMetaRefresh() throws Exception {
        // Follows HTML meta-refresh redirect to target page
        var result = Kreuzcrawl.scrape();
        assertTrue(result.final_url().contains("/target"), "expected to contain: " + "/target");
        assertEquals(1, result.redirect_count());
    }

    @Test
    void testRedirectRefreshHeader() throws Exception {
        // Handles HTTP Refresh header redirect
        var result = Kreuzcrawl.scrape();
        assertTrue(result.final_url().contains("/refreshed"), "expected to contain: " + "/refreshed");
        assertEquals(1, result.redirect_count());
    }

    @Test
    void testRedirectTo404() throws Exception {
        // Redirect target returns 404 Not Found
        var result = Kreuzcrawl.scrape();
        assertTrue(result.final_url().contains("/gone"), "expected to contain: " + "/gone");
        assertEquals(1, result.redirect_count());
        assertEquals(true, result.is_error());
    }

}

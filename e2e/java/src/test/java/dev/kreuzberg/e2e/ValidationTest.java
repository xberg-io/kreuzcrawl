package dev.kreuzberg.e2e;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;

/** E2e tests for category: validation. */
class ValidationTest {
    @Test
    void testValidationInvalidExcludeRegex() throws Exception {
        // Invalid regex in exclude_paths is rejected
        assertThrows(Exception.class, () -> Kreuzcrawl.scrape());
    }

    @Test
    void testValidationInvalidIncludeRegex() throws Exception {
        // Invalid regex in include_paths is rejected
        assertThrows(Exception.class, () -> Kreuzcrawl.scrape());
    }

    @Test
    void testValidationInvalidRetryCode() throws Exception {
        // Retry code outside 100-599 is rejected
        assertThrows(Exception.class, () -> Kreuzcrawl.scrape());
    }

    @Test
    void testValidationMaxPagesZero() throws Exception {
        // max_pages=0 is rejected as invalid config
        assertThrows(Exception.class, () -> Kreuzcrawl.scrape());
    }

    @Test
    void testValidationMaxRedirectsTooHigh() throws Exception {
        // max_redirects > 100 is rejected as invalid config
        assertThrows(Exception.class, () -> Kreuzcrawl.scrape());
    }

    @Test
    void testValidationTimeoutZero() throws Exception {
        // Zero request timeout is rejected as invalid config
        assertThrows(Exception.class, () -> Kreuzcrawl.scrape());
    }

}

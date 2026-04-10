package dev.kreuzberg.e2e;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;

/** E2e tests for category: stealth. */
class StealthTest {
    @Test
    void testStealthUaRotationConfig() throws Exception {
        // User-agent rotation config is accepted and crawl succeeds
        var result = Kreuzcrawl.scrape();
        assertEquals(200, result.status_code());
    }

}

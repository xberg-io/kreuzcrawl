package dev.kreuzberg.e2e;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;

/** E2e tests for category: cache. */
class CacheTest {
    @Test
    void testCacheBasic() throws Exception {
        // Crawling with disk cache enabled succeeds without errors
        var result = Kreuzcrawl.scrape();
        assertEquals(200, result.status_code());
    }

}

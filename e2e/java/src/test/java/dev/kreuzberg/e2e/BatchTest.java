package dev.kreuzberg.e2e;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;

/** E2e tests for category: batch. */
class BatchTest {
    @Test
    void testScrapeBatchBasic() throws Exception {
        // Batch scrape of multiple URLs all succeeding
        var result = Kreuzcrawl.scrape();
        assertEquals(3, result.batch().completed_count());
        assertEquals(0, result.batch().failed_count());
        assertEquals(3, result.batch().total_count());
    }

    @Test
    void testScrapeBatchPartialFailure() throws Exception {
        // Batch scrape with one URL failing returns partial results
        var result = Kreuzcrawl.scrape();
        assertEquals(2, result.batch().completed_count());
        assertEquals(1, result.batch().failed_count());
        assertEquals(3, result.batch().total_count());
    }

    @Test
    void testScrapeBatchProgress() throws Exception {
        // Batch scrape results include specific URL
        var result = Kreuzcrawl.scrape();
        assertEquals(2, result.batch().total_count());
        assertTrue(result.batch().results().contains("/target"), "expected to contain: " + "/target");
    }

}

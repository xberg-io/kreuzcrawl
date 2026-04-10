package dev.kreuzberg.e2e;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;

/** E2e tests for category: stream. */
class StreamTest {
    @Test
    void testCrawlStreamEvents() throws Exception {
        // Crawl stream produces page and complete events
        var result = Kreuzcrawl.scrape();
        assertTrue(result.stream().event_count_min() >= 4, "expected >= 4");
        assertEquals(true, result.stream().has_page_event());
        assertEquals(true, result.stream().has_complete_event());
    }

    @Test
    void testStreamDepthCrawl() throws Exception {
        // Stream produces events for multi-depth crawl with link following
        var result = Kreuzcrawl.scrape();
        assertTrue(result.stream().event_count_min() >= 5, "expected >= 5");
        assertEquals(true, result.stream().has_page_event());
        assertEquals(true, result.stream().has_complete_event());
    }

    @Test
    void testStreamWithErrorEvent() throws Exception {
        // Stream emits page and complete events even when some pages fail
        var result = Kreuzcrawl.scrape();
        assertEquals(true, result.stream().has_page_event());
        assertEquals(true, result.stream().has_complete_event());
        assertTrue(result.stream().event_count_min() >= 2, "expected >= 2");
    }

}

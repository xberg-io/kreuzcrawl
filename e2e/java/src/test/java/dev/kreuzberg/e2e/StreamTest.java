package dev.kreuzberg.e2e;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;
import dev.kreuzberg.kreuzcrawl.Kreuzcrawl;

/** E2e tests for category: stream. */
class StreamTest {
    @Test
    void testCrawlStreamEvents() throws Exception {
        // Crawl stream produces page and complete events
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        // skipped: field 'stream.event_count_min' not available on result type
        // skipped: field 'stream.has_page_event' not available on result type
        // skipped: field 'stream.has_complete_event' not available on result type
    }

    @Test
    void testStreamDepthCrawl() throws Exception {
        // Stream produces events for multi-depth crawl with link following
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        // skipped: field 'stream.event_count_min' not available on result type
        // skipped: field 'stream.has_page_event' not available on result type
        // skipped: field 'stream.has_complete_event' not available on result type
    }

    @Test
    void testStreamWithErrorEvent() throws Exception {
        // Stream emits page and complete events even when some pages fail
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        // skipped: field 'stream.has_page_event' not available on result type
        // skipped: field 'stream.has_complete_event' not available on result type
        // skipped: field 'stream.event_count_min' not available on result type
    }

}

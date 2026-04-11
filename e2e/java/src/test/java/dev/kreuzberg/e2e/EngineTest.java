package dev.kreuzberg.e2e;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;
import dev.kreuzberg.kreuzcrawl.Kreuzcrawl;

/** E2e tests for category: engine. */
class EngineTest {
    @Test
    void testEngineBatchBasic() throws Exception {
        // CrawlEngine with defaults batch scrapes like the free function
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        // skipped: field 'batch.completed_count' not available on result type
        // skipped: field 'batch.total_count' not available on result type
    }

    @Test
    void testEngineCrawlBasic() throws Exception {
        // CrawlEngine with defaults crawls multiple pages like the free function
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        // skipped: field 'crawl.pages_crawled' not available on result type
        // skipped: field 'crawl.min_pages' not available on result type
    }

    @Test
    void testEngineMapBasic() throws Exception {
        // CrawlEngine with defaults discovers URLs like the free function
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        // skipped: field 'map.min_urls' not available on result type
    }

    @Test
    void testEngineScrapeBasic() throws Exception {
        // CrawlEngine with defaults scrapes a page identically to the free function
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertEquals(200, result.statusCode());
        assertEquals("text/html", result.contentType());
        assertEquals("Engine Test", result.metadata().title().orElse(""));
        assertTrue(result.metadata().description().orElse("").contains("Testing the engine"), "expected to contain: " + "Testing the engine");
        assertTrue(result.links().size() >= 1, "expected >= 1");
        assertTrue(result.metadata().headings().orElseThrow().size() >= 1, "expected >= 1");
    }

    @Test
    void testEngineStreamBasic() throws Exception {
        // CrawlEngine with defaults streams events like the free function
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        // skipped: field 'stream.has_page_event' not available on result type
        // skipped: field 'stream.has_complete_event' not available on result type
        // skipped: field 'stream.event_count_min' not available on result type
    }

}

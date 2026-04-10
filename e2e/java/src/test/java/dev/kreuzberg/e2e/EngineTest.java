package dev.kreuzberg.e2e;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;

/** E2e tests for category: engine. */
class EngineTest {
    @Test
    void testEngineBatchBasic() throws Exception {
        // CrawlEngine with defaults batch scrapes like the free function
        var result = Kreuzcrawl.scrape();
        assertEquals(2, result.batch().completed_count());
        assertEquals(2, result.batch().total_count());
    }

    @Test
    void testEngineCrawlBasic() throws Exception {
        // CrawlEngine with defaults crawls multiple pages like the free function
        var result = Kreuzcrawl.scrape();
        assertEquals(3, result.crawl().pages_crawled());
        assertTrue(result.crawl().min_pages() >= 3, "expected >= 3");
    }

    @Test
    void testEngineMapBasic() throws Exception {
        // CrawlEngine with defaults discovers URLs like the free function
        var result = Kreuzcrawl.scrape();
        assertTrue(result.map().min_urls() >= 2, "expected >= 2");
    }

    @Test
    void testEngineScrapeBasic() throws Exception {
        // CrawlEngine with defaults scrapes a page identically to the free function
        var result = Kreuzcrawl.scrape();
        assertEquals(200, result.status_code());
        assertEquals("text/html", result.content_type());
        assertEquals("Engine Test", result.metadata().title().orElse(""));
        assertTrue(result.metadata().description_contains().contains("Testing the engine"), "expected to contain: " + "Testing the engine");
        assertTrue(result.links().min_count() >= 1, "expected >= 1");
        assertEquals(1, result.headings().h1_count());
        assertEquals("Hello Engine", result.headings().h1_text());
    }

    @Test
    void testEngineStreamBasic() throws Exception {
        // CrawlEngine with defaults streams events like the free function
        var result = Kreuzcrawl.scrape();
        assertEquals(true, result.stream().has_page_event());
        assertEquals(true, result.stream().has_complete_event());
        assertTrue(result.stream().event_count_min() >= 3, "expected >= 3");
    }

}

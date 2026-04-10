package dev.kreuzberg.e2e;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;

/** E2e tests for category: filter. */
class FilterTest {
    @Test
    void testFilterBm25CrawlIntegration() throws Exception {
        // BM25 filter works during multi-page crawl, keeping relevant pages
        var result = Kreuzcrawl.scrape();
        assertTrue(result.filter().remaining_contain_keyword().contains("rust"), "expected to contain: " + "rust");
    }

    @Test
    void testFilterBm25EmptyQuery() throws Exception {
        // BM25 filter with empty query passes all pages through
        var result = Kreuzcrawl.scrape();
        assertEquals(2, result.crawl().pages_crawled());
    }

    @Test
    void testFilterBm25HighThreshold() throws Exception {
        // BM25 filter with very high threshold filters out all pages
        var result = Kreuzcrawl.scrape();
        assertEquals(0, result.filter().pages_after_filter());
    }

    @Test
    void testFilterBm25RelevantPages() throws Exception {
        // BM25 filter keeps only pages relevant to the query
        var result = Kreuzcrawl.scrape();
        assertTrue(result.filter().remaining_contain_keyword().contains("rust"), "expected to contain: " + "rust");
    }

    @Test
    void testFilterBm25ThresholdZero() throws Exception {
        // BM25 filter with zero threshold passes all pages
        var result = Kreuzcrawl.scrape();
        assertEquals(2, result.crawl().pages_crawled());
    }

    @Test
    void testFilterNoopCrawlAllKept() throws Exception {
        // NoopFilter keeps all pages during a multi-page crawl
        var result = Kreuzcrawl.scrape();
        assertEquals(3, result.filter().pages_after_filter());
    }

    @Test
    void testFilterNoopPassesAll() throws Exception {
        // No content filter passes all crawled pages through
        var result = Kreuzcrawl.scrape();
        assertEquals(3, result.crawl().pages_crawled());
    }

}

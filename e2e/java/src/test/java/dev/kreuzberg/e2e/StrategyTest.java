package dev.kreuzberg.e2e;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;
import dev.kreuzberg.kreuzcrawl.Kreuzcrawl;

/** E2e tests for category: strategy. */
class StrategyTest {
    @Test
    void testStrategyBestFirstSeed() throws Exception {
        // BestFirst strategy always processes the seed URL first
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        // skipped: field 'crawl.pages_crawled' not available on result type
        // skipped: field 'strategy.first_page_url_contains' not available on result type
    }

    @Test
    void testStrategyBfsDefaultOrder() throws Exception {
        // BFS strategy visits pages in breadth-first order
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        // skipped: field 'crawl.pages_crawled' not available on result type
        // skipped: field 'strategy.crawl_order' not available on result type
    }

    @Test
    void testStrategyDfsDepthFirst() throws Exception {
        // DFS strategy visits pages in depth-first order
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        // skipped: field 'crawl.pages_crawled' not available on result type
        // skipped: field 'strategy.crawl_order' not available on result type
    }

}

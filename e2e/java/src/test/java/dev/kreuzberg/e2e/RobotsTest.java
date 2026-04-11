package dev.kreuzberg.e2e;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;
import dev.kreuzberg.kreuzcrawl.Kreuzcrawl;

/** E2e tests for category: robots. */
class RobotsTest {
    @Test
    void testRobotsAllowAll() throws Exception {
        // Permissive robots.txt allows all paths
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertEquals(true, result.isAllowed());
    }

    @Test
    void testRobotsAllowOverride() throws Exception {
        // Allow directive overrides Disallow for specific paths
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertEquals(true, result.isAllowed());
    }

    @Test
    void testRobotsCommentsHandling() throws Exception {
        // Correctly parses robots.txt with inline and line comments
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertEquals(true, result.isAllowed());
    }

    @Test
    void testRobotsCrawlDelay() throws Exception {
        // Respects crawl-delay directive from robots.txt
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertEquals(2, result.crawlDelay().orElse(""));
    }

    @Test
    void testRobotsDisallowPath() throws Exception {
        // Robots.txt disallows specific paths
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertEquals(false, result.isAllowed());
    }

    @Test
    void testRobotsMetaNofollow() throws Exception {
        // Detects nofollow meta robots tag and skips link extraction
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertEquals(true, result.nofollowDetected());
    }

    @Test
    void testRobotsMetaNoindex() throws Exception {
        // Detects noindex meta robots tag in HTML page
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertEquals(true, result.noindexDetected());
    }

    @Test
    void testRobotsMissing404() throws Exception {
        // Missing robots.txt (404) allows all crawling
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertEquals(true, result.isAllowed());
    }

    @Test
    void testRobotsMultipleUserAgents() throws Exception {
        // Picks the most specific user-agent block from robots.txt
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertEquals(true, result.isAllowed());
    }

    @Test
    void testRobotsRequestRate() throws Exception {
        // Parses request-rate directive from robots.txt
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertEquals(5, result.crawlDelay().orElse(""));
        assertEquals(true, result.isAllowed());
    }

    @Test
    void testRobotsSitemapDirective() throws Exception {
        // Discovers sitemap URL from Sitemap directive in robots.txt
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertEquals(true, result.isAllowed());
    }

    @Test
    void testRobotsUserAgentSpecific() throws Exception {
        // Matches user-agent specific rules in robots.txt
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertEquals(false, result.isAllowed());
    }

    @Test
    void testRobotsWildcardPaths() throws Exception {
        // Handles wildcard Disallow patterns in robots.txt
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertEquals(false, result.isAllowed());
    }

    @Test
    void testRobotsXRobotsTag() throws Exception {
        // Respects X-Robots-Tag HTTP header directives
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertEquals("noindex, nofollow", result.xRobotsTag().orElse(""));
        assertEquals(true, result.noindexDetected());
        assertEquals(true, result.nofollowDetected());
    }

}

package dev.kreuzberg.e2e;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;

/** E2e tests for category: robots. */
class RobotsTest {
    @Test
    void testRobotsAllowAll() throws Exception {
        // Permissive robots.txt allows all paths
        var result = Kreuzcrawl.scrape();
        assertEquals(true, result.robots().is_allowed());
    }

    @Test
    void testRobotsAllowOverride() throws Exception {
        // Allow directive overrides Disallow for specific paths
        var result = Kreuzcrawl.scrape();
        assertEquals(true, result.robots().is_allowed());
    }

    @Test
    void testRobotsCommentsHandling() throws Exception {
        // Correctly parses robots.txt with inline and line comments
        var result = Kreuzcrawl.scrape();
        assertEquals(true, result.robots().is_allowed());
    }

    @Test
    void testRobotsCrawlDelay() throws Exception {
        // Respects crawl-delay directive from robots.txt
        var result = Kreuzcrawl.scrape();
        assertEquals(2, result.robots().crawl_delay());
    }

    @Test
    void testRobotsDisallowPath() throws Exception {
        // Robots.txt disallows specific paths
        var result = Kreuzcrawl.scrape();
        assertEquals(false, result.robots().is_allowed());
    }

    @Test
    void testRobotsMetaNofollow() throws Exception {
        // Detects nofollow meta robots tag and skips link extraction
        var result = Kreuzcrawl.scrape();
        assertEquals(true, result.robots().nofollow_detected());
    }

    @Test
    void testRobotsMetaNoindex() throws Exception {
        // Detects noindex meta robots tag in HTML page
        var result = Kreuzcrawl.scrape();
        assertEquals(true, result.robots().noindex_detected());
    }

    @Test
    void testRobotsMissing404() throws Exception {
        // Missing robots.txt (404) allows all crawling
        var result = Kreuzcrawl.scrape();
        assertEquals(true, result.robots().is_allowed());
    }

    @Test
    void testRobotsMultipleUserAgents() throws Exception {
        // Picks the most specific user-agent block from robots.txt
        var result = Kreuzcrawl.scrape();
        assertEquals(true, result.robots().is_allowed());
    }

    @Test
    void testRobotsRequestRate() throws Exception {
        // Parses request-rate directive from robots.txt
        var result = Kreuzcrawl.scrape();
        assertEquals(5, result.robots().crawl_delay());
        assertEquals(true, result.robots().is_allowed());
    }

    @Test
    void testRobotsSitemapDirective() throws Exception {
        // Discovers sitemap URL from Sitemap directive in robots.txt
        var result = Kreuzcrawl.scrape();
        assertEquals(true, result.robots().is_allowed());
    }

    @Test
    void testRobotsUserAgentSpecific() throws Exception {
        // Matches user-agent specific rules in robots.txt
        var result = Kreuzcrawl.scrape();
        assertEquals(false, result.robots().is_allowed());
    }

    @Test
    void testRobotsWildcardPaths() throws Exception {
        // Handles wildcard Disallow patterns in robots.txt
        var result = Kreuzcrawl.scrape();
        assertEquals(false, result.robots().is_allowed());
    }

    @Test
    void testRobotsXRobotsTag() throws Exception {
        // Respects X-Robots-Tag HTTP header directives
        var result = Kreuzcrawl.scrape();
        assertEquals("noindex, nofollow", result.robots().x_robots_tag());
        assertEquals(true, result.robots().noindex_detected());
        assertEquals(true, result.robots().nofollow_detected());
    }

}

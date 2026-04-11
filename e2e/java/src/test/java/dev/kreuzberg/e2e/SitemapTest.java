package dev.kreuzberg.e2e;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;
import dev.kreuzberg.kreuzcrawl.Kreuzcrawl;

/** E2e tests for category: sitemap. */
class SitemapTest {
    @Test
    void testSitemapBasic() throws Exception {
        // Parses a standard urlset sitemap
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        // skipped: field 'urls.length' not available on result type
        // skipped: field 'has_lastmod' not available on result type
    }

    @Test
    void testSitemapCompressedGzip() throws Exception {
        // Parses a gzip-compressed sitemap file
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        // skipped: field 'urls.length' not available on result type
    }

    @Test
    void testSitemapEmpty() throws Exception {
        // Handles empty sitemap gracefully
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        // skipped: field 'urls.length' not available on result type
    }

    @Test
    void testSitemapFromRobotsTxt() throws Exception {
        // Discovers sitemap via robots.txt Sitemap directive
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        // skipped: field 'urls.length' not available on result type
    }

    @Test
    void testSitemapIndex() throws Exception {
        // Follows sitemap index to discover child sitemaps
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        // skipped: field 'urls.length' not available on result type
    }

    @Test
    void testSitemapLastmodFilter() throws Exception {
        // Filters sitemap URLs by lastmod date
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        // skipped: field 'urls.length' not available on result type
        // skipped: field 'has_lastmod' not available on result type
    }

    @Test
    void testSitemapOnlyMode() throws Exception {
        // Uses sitemap URLs exclusively without following page links
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        // skipped: field 'urls.length' not available on result type
    }

    @Test
    void testSitemapXhtmlLinks() throws Exception {
        // Parses sitemap with XHTML namespace alternate links
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        // skipped: field 'urls.length' not available on result type
        // skipped: field 'has_lastmod' not available on result type
    }

}

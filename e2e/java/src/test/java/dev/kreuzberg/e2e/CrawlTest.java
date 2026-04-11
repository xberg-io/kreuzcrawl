package dev.kreuzberg.e2e;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;
import dev.kreuzberg.kreuzcrawl.Kreuzcrawl;

/** E2e tests for category: crawl. */
class CrawlTest {
    @Test
    void testContentBinarySkip() throws Exception {
        // Skips image and video content types gracefully
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertEquals(true, result.wasSkipped());
    }

    @Test
    void testContentPdfLinkSkip() throws Exception {
        // Encounters PDF link and skips or marks as document type
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertEquals(true, result.wasSkipped());
    }

    @Test
    void testCrawlConcurrentDepth() throws Exception {
        // Concurrent crawl respects max_depth limit
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        // skipped: field 'pages.length' not available on result type
        // skipped: field 'stayed_on_domain' not available on result type
    }

    @Test
    void testCrawlConcurrentLimit() throws Exception {
        // Respects max concurrent requests limit during crawl
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        // skipped: field 'pages.length' not available on result type
    }

    @Test
    void testCrawlConcurrentMaxPages() throws Exception {
        // Concurrent crawl respects max_pages budget
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        // skipped: field 'pages.length' not available on result type
    }

    @Test
    void testCrawlCustomHeaders() throws Exception {
        // Sends custom headers on all crawl requests
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        // skipped: field 'pages.length' not available on result type
    }

    @Test
    void testCrawlDepthOne() throws Exception {
        // Follows links one level deep from start page
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        // skipped: field 'pages.length' not available on result type
        // skipped: field 'stayed_on_domain' not available on result type
    }

    @Test
    void testCrawlDepthPriority() throws Exception {
        // Crawls in breadth-first order, processing depth-0 pages before depth-1
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        // skipped: field 'pages.length' not available on result type
    }

    @Test
    void testCrawlDepthTwo() throws Exception {
        // Crawls 3 levels deep (depth 0, 1, 2)
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        // skipped: field 'pages.length' not available on result type
        // skipped: field 'pages.length' not available on result type
    }

    @Test
    void testCrawlDepthTwoChain() throws Exception {
        // Depth=2 crawl follows a chain of links across three levels
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        // skipped: field 'pages.length' not available on result type
    }

    @Test
    void testCrawlDoubleSlashNormalization() throws Exception {
        // Normalizes double slashes in URL paths (//page to /page)
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        // skipped: field 'unique_urls.length' not available on result type
    }

    @Test
    void testCrawlEmptyPageNoLinks() throws Exception {
        // Crawl completes when child page has no outgoing links
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        // skipped: field 'pages.length' not available on result type
    }

    @Test
    void testCrawlExcludePathPattern() throws Exception {
        // Skips URLs matching the exclude path pattern
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        // skipped: field 'pages.length' not available on result type
    }

    @Test
    void testCrawlExternalLinksIgnored() throws Exception {
        // External links are discovered but not followed when stay_on_domain is true
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        // skipped: field 'pages.length' not available on result type
        // skipped: field 'stayed_on_domain' not available on result type
    }

    @Test
    void testCrawlFragmentStripping() throws Exception {
        // Strips #fragment from URLs for deduplication
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        // skipped: field 'unique_urls.length' not available on result type
    }

    @Test
    void testCrawlIncludePathPattern() throws Exception {
        // Only follows URLs matching the include path pattern
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        // skipped: field 'pages.length' not available on result type
    }

    @Test
    void testCrawlMaxDepthZero() throws Exception {
        // max_depth=0 crawls only the seed page with no link following
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        // skipped: field 'pages.length' not available on result type
        // skipped: field 'pages.length' not available on result type
    }

    @Test
    void testCrawlMaxPages() throws Exception {
        // Stops crawling at page budget limit
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        // skipped: field 'pages.length' not available on result type
    }

    @Test
    void testCrawlMixedContentTypes() throws Exception {
        // Crawl handles links to non-HTML content types gracefully
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        // skipped: field 'pages.length' not available on result type
    }

    @Test
    void testCrawlMultipleRedirectsInTraversal() throws Exception {
        // Multiple linked pages with redirects are handled during crawl traversal
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        // skipped: field 'pages.length' not available on result type
    }

    @Test
    void testCrawlQueryParamDedup() throws Exception {
        // Deduplicates URLs with same query params in different order
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        // skipped: field 'unique_urls.length' not available on result type
    }

    @Test
    void testCrawlRedirectInTraversal() throws Exception {
        // Links that redirect are followed during crawl traversal
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        // skipped: field 'pages.length' not available on result type
    }

    @Test
    void testCrawlSelfLinkNoLoop() throws Exception {
        // Page linking to itself does not cause infinite crawl loop
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        // skipped: field 'pages.length' not available on result type
    }

    @Test
    void testCrawlSinglePageNoLinks() throws Exception {
        // Crawling a page with no links returns only the seed page
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        // skipped: field 'pages.length' not available on result type
    }

    @Test
    void testCrawlStayOnDomain() throws Exception {
        // Does not follow external links when stay_on_domain is true
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        // skipped: field 'pages.length' not available on result type
        // skipped: field 'stayed_on_domain' not available on result type
    }

    @Test
    void testCrawlSubdomainExclusion() throws Exception {
        // Stays on exact domain and skips subdomain links
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        // skipped: field 'pages.length' not available on result type
        // skipped: field 'stayed_on_domain' not available on result type
    }

    @Test
    void testCrawlSubdomainInclusion() throws Exception {
        // Crawls subdomains when allow_subdomains is enabled
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        // skipped: field 'pages.length' not available on result type
    }

    @Test
    void testCrawlTrailingSlashDedup() throws Exception {
        // Deduplicates /page and /page/ as the same URL
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        // skipped: field 'unique_urls.length' not available on result type
    }

    @Test
    void testCrawlUrlDeduplication() throws Exception {
        // Deduplicates URLs that differ only by fragment or query params
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        // skipped: field 'pages.length' not available on result type
    }

}

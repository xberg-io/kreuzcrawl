using Xunit;
using Kreuzcrawl;

namespace Kreuzberg.E2e;

/// <summary>E2e tests for category: crawl.</summary>
public class CrawlTests
{
    [Fact]
    public void Test_ContentBinarySkip()
    {
        // Skips image and video content types gracefully
        var result = KreuzcrawlLib.Scrape();
        Assert.Equal(true, result.Content.WasSkipped.Trim());
    }

    [Fact]
    public void Test_ContentPdfLinkSkip()
    {
        // Encounters PDF link and skips or marks as document type
        var result = KreuzcrawlLib.Scrape();
        Assert.Equal(true, result.Content.WasSkipped.Trim());
    }

    [Fact]
    public void Test_CrawlConcurrentDepth()
    {
        // Concurrent crawl respects max_depth limit
        var result = KreuzcrawlLib.Scrape();
        Assert.Equal(3, result.Pages.Count.Trim());
        Assert.Equal(true, result.StayedOnDomain.Trim());
    }

    [Fact]
    public void Test_CrawlConcurrentLimit()
    {
        // Respects max concurrent requests limit during crawl
        var result = KreuzcrawlLib.Scrape();
        Assert.Equal(5, result.Pages.Count.Trim());
    }

    [Fact]
    public void Test_CrawlConcurrentMaxPages()
    {
        // Concurrent crawl respects max_pages budget
        var result = KreuzcrawlLib.Scrape();
        Assert.True(result.Pages.Count <= 3, "expected <= 3");
    }

    [Fact]
    public void Test_CrawlCustomHeaders()
    {
        // Sends custom headers on all crawl requests
        var result = KreuzcrawlLib.Scrape();
        Assert.Equal(2, result.Pages.Count.Trim());
    }

    [Fact]
    public void Test_CrawlDepthOne()
    {
        // Follows links one level deep from start page
        var result = KreuzcrawlLib.Scrape();
        Assert.Equal(3, result.Pages.Count.Trim());
        Assert.Equal(true, result.StayedOnDomain.Trim());
    }

    [Fact]
    public void Test_CrawlDepthPriority()
    {
        // Crawls in breadth-first order, processing depth-0 pages before depth-1
        var result = KreuzcrawlLib.Scrape();
        Assert.Equal(4, result.Pages.Count.Trim());
    }

    [Fact]
    public void Test_CrawlDepthTwo()
    {
        // Crawls 3 levels deep (depth 0, 1, 2)
        var result = KreuzcrawlLib.Scrape();
        Assert.Equal(3, result.Pages.Count.Trim());
        Assert.True(result.Pages.Count >= 3, "expected >= 3");
    }

    [Fact]
    public void Test_CrawlDepthTwoChain()
    {
        // Depth=2 crawl follows a chain of links across three levels
        var result = KreuzcrawlLib.Scrape();
        Assert.Equal(3, result.Pages.Count.Trim());
    }

    [Fact]
    public void Test_CrawlDoubleSlashNormalization()
    {
        // Normalizes double slashes in URL paths (//page to /page)
        var result = KreuzcrawlLib.Scrape();
        Assert.Equal(2, result.UniqueUrls.Count.Trim());
    }

    [Fact]
    public void Test_CrawlEmptyPageNoLinks()
    {
        // Crawl completes when child page has no outgoing links
        var result = KreuzcrawlLib.Scrape();
        Assert.Equal(2, result.Pages.Count.Trim());
    }

    [Fact]
    public void Test_CrawlExcludePathPattern()
    {
        // Skips URLs matching the exclude path pattern
        var result = KreuzcrawlLib.Scrape();
        Assert.Equal(2, result.Pages.Count.Trim());
    }

    [Fact]
    public void Test_CrawlExternalLinksIgnored()
    {
        // External links are discovered but not followed when stay_on_domain is true
        var result = KreuzcrawlLib.Scrape();
        Assert.Equal(2, result.Pages.Count.Trim());
        Assert.Equal(true, result.StayedOnDomain.Trim());
    }

    [Fact]
    public void Test_CrawlFragmentStripping()
    {
        // Strips #fragment from URLs for deduplication
        var result = KreuzcrawlLib.Scrape();
        Assert.Equal(2, result.UniqueUrls.Count.Trim());
    }

    [Fact]
    public void Test_CrawlIncludePathPattern()
    {
        // Only follows URLs matching the include path pattern
        var result = KreuzcrawlLib.Scrape();
        Assert.Equal(2, result.Pages.Count.Trim());
    }

    [Fact]
    public void Test_CrawlMaxDepthZero()
    {
        // max_depth=0 crawls only the seed page with no link following
        var result = KreuzcrawlLib.Scrape();
        Assert.Equal(1, result.Pages.Count.Trim());
        Assert.True(result.Pages.Count <= 1, "expected <= 1");
    }

    [Fact]
    public void Test_CrawlMaxPages()
    {
        // Stops crawling at page budget limit
        var result = KreuzcrawlLib.Scrape();
        Assert.True(result.Pages.Count <= 3, "expected <= 3");
    }

    [Fact]
    public void Test_CrawlMixedContentTypes()
    {
        // Crawl handles links to non-HTML content types gracefully
        var result = KreuzcrawlLib.Scrape();
        Assert.True(result.Pages.Count >= 2, "expected >= 2");
    }

    [Fact]
    public void Test_CrawlMultipleRedirectsInTraversal()
    {
        // Multiple linked pages with redirects are handled during crawl traversal
        var result = KreuzcrawlLib.Scrape();
        Assert.True(result.Pages.Count >= 1, "expected >= 1");
    }

    [Fact]
    public void Test_CrawlQueryParamDedup()
    {
        // Deduplicates URLs with same query params in different order
        var result = KreuzcrawlLib.Scrape();
        Assert.Equal(2, result.UniqueUrls.Count.Trim());
    }

    [Fact]
    public void Test_CrawlRedirectInTraversal()
    {
        // Links that redirect are followed during crawl traversal
        var result = KreuzcrawlLib.Scrape();
        Assert.True(result.Pages.Count >= 1, "expected >= 1");
    }

    [Fact]
    public void Test_CrawlSelfLinkNoLoop()
    {
        // Page linking to itself does not cause infinite crawl loop
        var result = KreuzcrawlLib.Scrape();
        Assert.Equal(2, result.Pages.Count.Trim());
    }

    [Fact]
    public void Test_CrawlSinglePageNoLinks()
    {
        // Crawling a page with no links returns only the seed page
        var result = KreuzcrawlLib.Scrape();
        Assert.Equal(1, result.Pages.Count.Trim());
    }

    [Fact]
    public void Test_CrawlStayOnDomain()
    {
        // Does not follow external links when stay_on_domain is true
        var result = KreuzcrawlLib.Scrape();
        Assert.Equal(2, result.Pages.Count.Trim());
        Assert.Equal(true, result.StayedOnDomain.Trim());
    }

    [Fact]
    public void Test_CrawlSubdomainExclusion()
    {
        // Stays on exact domain and skips subdomain links
        var result = KreuzcrawlLib.Scrape();
        Assert.Equal(2, result.Pages.Count.Trim());
        Assert.Equal(true, result.StayedOnDomain.Trim());
    }

    [Fact]
    public void Test_CrawlSubdomainInclusion()
    {
        // Crawls subdomains when allow_subdomains is enabled
        var result = KreuzcrawlLib.Scrape();
        Assert.True(result.Pages.Count >= 2, "expected >= 2");
    }

    [Fact]
    public void Test_CrawlTrailingSlashDedup()
    {
        // Deduplicates /page and /page/ as the same URL
        var result = KreuzcrawlLib.Scrape();
        Assert.Equal(2, result.UniqueUrls.Count.Trim());
    }

    [Fact]
    public void Test_CrawlUrlDeduplication()
    {
        // Deduplicates URLs that differ only by fragment or query params
        var result = KreuzcrawlLib.Scrape();
        Assert.True(result.Pages.Count <= 2, "expected <= 2");
    }
}

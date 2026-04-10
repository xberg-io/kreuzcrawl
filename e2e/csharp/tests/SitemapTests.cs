using Xunit;
using Kreuzcrawl;

namespace Kreuzberg.E2e;

/// <summary>E2e tests for category: sitemap.</summary>
public class SitemapTests
{
    [Fact]
    public void Test_SitemapBasic()
    {
        // Parses a standard urlset sitemap
        var result = KreuzcrawlLib.Scrape();
        Assert.Equal(4, result.Urls.Count.Trim());
        Assert.Equal(true, result.HasLastmod.Trim());
    }

    [Fact]
    public void Test_SitemapCompressedGzip()
    {
        // Parses a gzip-compressed sitemap file
        var result = KreuzcrawlLib.Scrape();
        Assert.Equal(3, result.Urls.Count.Trim());
    }

    [Fact]
    public void Test_SitemapEmpty()
    {
        // Handles empty sitemap gracefully
        var result = KreuzcrawlLib.Scrape();
        Assert.Equal(0, result.Urls.Count.Trim());
    }

    [Fact]
    public void Test_SitemapFromRobotsTxt()
    {
        // Discovers sitemap via robots.txt Sitemap directive
        var result = KreuzcrawlLib.Scrape();
        Assert.Equal(4, result.Urls.Count.Trim());
    }

    [Fact]
    public void Test_SitemapIndex()
    {
        // Follows sitemap index to discover child sitemaps
        var result = KreuzcrawlLib.Scrape();
        Assert.Equal(3, result.Urls.Count.Trim());
    }

    [Fact]
    public void Test_SitemapLastmodFilter()
    {
        // Filters sitemap URLs by lastmod date
        var result = KreuzcrawlLib.Scrape();
        Assert.Equal(4, result.Urls.Count.Trim());
        Assert.Equal(true, result.HasLastmod.Trim());
    }

    [Fact]
    public void Test_SitemapOnlyMode()
    {
        // Uses sitemap URLs exclusively without following page links
        var result = KreuzcrawlLib.Scrape();
        Assert.Equal(4, result.Urls.Count.Trim());
    }

    [Fact]
    public void Test_SitemapXhtmlLinks()
    {
        // Parses sitemap with XHTML namespace alternate links
        var result = KreuzcrawlLib.Scrape();
        Assert.Equal(2, result.Urls.Count.Trim());
        Assert.Equal(false, result.HasLastmod.Trim());
    }
}

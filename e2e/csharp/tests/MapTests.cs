using Xunit;
using Kreuzcrawl;

namespace Kreuzberg.E2e;

/// <summary>E2e tests for category: map.</summary>
public class MapTests
{
    [Fact]
    public void Test_MapDiscoverUrls()
    {
        // Discovers all URLs on a site without fetching full content
        var result = KreuzcrawlLib.Scrape();
        Assert.True(result.Urls.Count >= 3, "expected >= 3");
    }

    [Fact]
    public void Test_MapExcludePatterns()
    {
        // Excludes URLs matching patterns from URL map
        var result = KreuzcrawlLib.Scrape();
        Assert.Equal(1, result.Urls.Count.Trim());
    }

    [Fact]
    public void Test_MapIncludeSubdomains()
    {
        // Includes subdomain URLs in URL map discovery
        var result = KreuzcrawlLib.Scrape();
        Assert.True(result.Urls.Count >= 2, "expected >= 2");
        Assert.Contains("blog.example.com", result.Urls);
    }

    [Fact]
    public void Test_MapLargeSitemap()
    {
        // Handles large sitemap with 100+ URLs
        var result = KreuzcrawlLib.Scrape();
        Assert.True(result.Urls.Count >= 100, "expected >= 100");
    }

    [Fact]
    public void Test_MapLimitPagination()
    {
        // Limits map result count to specified maximum
        var result = KreuzcrawlLib.Scrape();
        Assert.True(result.Urls.Count <= 5, "expected <= 5");
    }

    [Fact]
    public void Test_MapSearchFilter()
    {
        // Filters map results by search keyword
        var result = KreuzcrawlLib.Scrape();
        Assert.True(result.Urls.Count >= 2, "expected >= 2");
        Assert.Contains("blog", result.Urls);
    }
}

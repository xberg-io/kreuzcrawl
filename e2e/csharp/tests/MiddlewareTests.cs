using Xunit;
using Kreuzcrawl;

namespace Kreuzberg.E2e;

/// <summary>E2e tests for category: middleware.</summary>
public class MiddlewareTests
{
    [Fact]
    public void Test_MiddlewareEngineCrawlWithDefaults()
    {
        // Engine crawl with default middleware chain produces correct multi-page results
        var result = KreuzcrawlLib.Scrape();
        Assert.Equal(3, result.Crawl.PagesCrawled.Trim());
        Assert.True(result.Crawl.MinPages >= 3, "expected >= 3");
    }

    [Fact]
    public void Test_MiddlewareNoopNoEffect()
    {
        // Default middleware chain does not affect normal scraping
        var result = KreuzcrawlLib.Scrape();
        Assert.Equal(200, result.StatusCode.Trim());
        Assert.Equal("Middleware Test", result.Metadata.Title.Trim());
    }
}

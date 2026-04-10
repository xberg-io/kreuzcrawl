using Xunit;
using Kreuzcrawl;

namespace Kreuzberg.E2e;

/// <summary>E2e tests for category: engine.</summary>
public class EngineTests
{
    [Fact]
    public void Test_EngineBatchBasic()
    {
        // CrawlEngine with defaults batch scrapes like the free function
        var result = KreuzcrawlLib.Scrape();
        Assert.Equal(2, result.Batch.CompletedCount.Trim());
        Assert.Equal(2, result.Batch.TotalCount.Trim());
    }

    [Fact]
    public void Test_EngineCrawlBasic()
    {
        // CrawlEngine with defaults crawls multiple pages like the free function
        var result = KreuzcrawlLib.Scrape();
        Assert.Equal(3, result.Crawl.PagesCrawled.Trim());
        Assert.True(result.Crawl.MinPages >= 3, "expected >= 3");
    }

    [Fact]
    public void Test_EngineMapBasic()
    {
        // CrawlEngine with defaults discovers URLs like the free function
        var result = KreuzcrawlLib.Scrape();
        Assert.True(result.Map.MinUrls >= 2, "expected >= 2");
    }

    [Fact]
    public void Test_EngineScrapeBasic()
    {
        // CrawlEngine with defaults scrapes a page identically to the free function
        var result = KreuzcrawlLib.Scrape();
        Assert.Equal(200, result.StatusCode.Trim());
        Assert.Equal("text/html", result.ContentType.Trim());
        Assert.Equal("Engine Test", result.Metadata.Title.Trim());
        Assert.Contains("Testing the engine", result.Metadata.DescriptionContains);
        Assert.True(result.Links.MinCount >= 1, "expected >= 1");
        Assert.Equal(1, result.Headings.H1Count.Trim());
        Assert.Equal("Hello Engine", result.Headings.H1Text.Trim());
    }

    [Fact]
    public void Test_EngineStreamBasic()
    {
        // CrawlEngine with defaults streams events like the free function
        var result = KreuzcrawlLib.Scrape();
        Assert.Equal(true, result.Stream.HasPageEvent.Trim());
        Assert.Equal(true, result.Stream.HasCompleteEvent.Trim());
        Assert.True(result.Stream.EventCountMin >= 3, "expected >= 3");
    }
}

using Xunit;
using Kreuzcrawl;

namespace Kreuzberg.E2e;

/// <summary>E2e tests for category: filter.</summary>
public class FilterTests
{
    [Fact]
    public void Test_FilterBm25CrawlIntegration()
    {
        // BM25 filter works during multi-page crawl, keeping relevant pages
        var result = KreuzcrawlLib.Scrape();
        Assert.Contains("rust", result.Filter.RemainingContainKeyword);
    }

    [Fact]
    public void Test_FilterBm25EmptyQuery()
    {
        // BM25 filter with empty query passes all pages through
        var result = KreuzcrawlLib.Scrape();
        Assert.Equal(2, result.Crawl.PagesCrawled.Trim());
    }

    [Fact]
    public void Test_FilterBm25HighThreshold()
    {
        // BM25 filter with very high threshold filters out all pages
        var result = KreuzcrawlLib.Scrape();
        Assert.Equal(0, result.Filter.PagesAfterFilter.Trim());
    }

    [Fact]
    public void Test_FilterBm25RelevantPages()
    {
        // BM25 filter keeps only pages relevant to the query
        var result = KreuzcrawlLib.Scrape();
        Assert.Contains("rust", result.Filter.RemainingContainKeyword);
    }

    [Fact]
    public void Test_FilterBm25ThresholdZero()
    {
        // BM25 filter with zero threshold passes all pages
        var result = KreuzcrawlLib.Scrape();
        Assert.Equal(2, result.Crawl.PagesCrawled.Trim());
    }

    [Fact]
    public void Test_FilterNoopCrawlAllKept()
    {
        // NoopFilter keeps all pages during a multi-page crawl
        var result = KreuzcrawlLib.Scrape();
        Assert.Equal(3, result.Filter.PagesAfterFilter.Trim());
    }

    [Fact]
    public void Test_FilterNoopPassesAll()
    {
        // No content filter passes all crawled pages through
        var result = KreuzcrawlLib.Scrape();
        Assert.Equal(3, result.Crawl.PagesCrawled.Trim());
    }
}

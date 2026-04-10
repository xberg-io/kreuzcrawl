using Xunit;
using Kreuzcrawl;

namespace Kreuzberg.E2e;

/// <summary>E2e tests for category: strategy.</summary>
public class StrategyTests
{
    [Fact]
    public void Test_StrategyBestFirstSeed()
    {
        // BestFirst strategy always processes the seed URL first
        var result = KreuzcrawlLib.Scrape();
        Assert.Equal(3, result.Crawl.PagesCrawled.Trim());
        Assert.Contains("/", result.Strategy.FirstPageUrlContains);
    }

    [Fact]
    public void Test_StrategyBfsDefaultOrder()
    {
        // BFS strategy visits pages in breadth-first order
        var result = KreuzcrawlLib.Scrape();
        Assert.Equal(5, result.Crawl.PagesCrawled.Trim());
        Assert.Equal(new[] { "/", "/a", "/b", "/a/1", "/b/1" }, result.Strategy.CrawlOrder.Trim());
    }

    [Fact]
    public void Test_StrategyDfsDepthFirst()
    {
        // DFS strategy visits pages in depth-first order
        var result = KreuzcrawlLib.Scrape();
        Assert.Equal(5, result.Crawl.PagesCrawled.Trim());
        Assert.Equal(new[] { "/", "/b", "/b/1", "/a", "/a/1" }, result.Strategy.CrawlOrder.Trim());
    }
}

using Xunit;
using Kreuzcrawl;

namespace Kreuzberg.E2e;

/// <summary>E2e tests for category: concurrent.</summary>
public class ConcurrentTests
{
    [Fact]
    public void Test_ConcurrentBasic()
    {
        // Concurrent crawling fetches all pages with max_concurrent workers
        var result = KreuzcrawlLib.Scrape();
        Assert.Equal(6, result.Pages.Count.Trim());
        Assert.True(result.Pages.Count >= 6, "expected >= 6");
    }

    [Fact]
    public void Test_ConcurrentDepthTwoFanOut()
    {
        // Concurrent depth=2 crawl correctly fans out and deduplicates across levels
        var result = KreuzcrawlLib.Scrape();
        Assert.Equal(4, result.Pages.Count.Trim());
    }

    [Fact]
    public void Test_ConcurrentMaxPagesExact()
    {
        // Concurrent crawling does not exceed max_pages limit even with high concurrency
        var result = KreuzcrawlLib.Scrape();
        Assert.True(result.Pages.Count <= 3, "expected <= 3");
    }

    [Fact]
    public void Test_ConcurrentPartialErrors()
    {
        // Concurrent crawl handles partial failures gracefully
        var result = KreuzcrawlLib.Scrape();
        Assert.True(result.Pages.Count >= 2, "expected >= 2");
    }

    [Fact]
    public void Test_ConcurrentRespectsMaxPages()
    {
        // Concurrent crawling respects max_pages limit
        var result = KreuzcrawlLib.Scrape();
        Assert.True(result.Pages.Count <= 3, "expected <= 3");
    }
}

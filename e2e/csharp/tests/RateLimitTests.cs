using Xunit;
using Kreuzcrawl;

namespace Kreuzberg.E2e;

/// <summary>E2e tests for category: rate_limit.</summary>
public class RateLimitTests
{
    [Fact]
    public void Test_RateLimitBasicDelay()
    {
        // Rate limiter adds delay between requests to the same domain
        var result = KreuzcrawlLib.Scrape();
        Assert.Equal(3, result.Crawl.PagesCrawled.Trim());
        Assert.True(result.RateLimit.MinDurationMs >= 150, "expected >= 150");
    }

    [Fact]
    public void Test_RateLimitZeroNoDelay()
    {
        // Rate limiter with zero delay does not slow crawling
        var result = KreuzcrawlLib.Scrape();
        Assert.Equal(2, result.Crawl.PagesCrawled.Trim());
    }
}

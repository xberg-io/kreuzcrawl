using Xunit;
using Kreuzcrawl;

namespace Kreuzberg.E2e;

/// <summary>E2e tests for category: cache.</summary>
public class CacheTests
{
    [Fact]
    public void Test_CacheBasic()
    {
        // Crawling with disk cache enabled succeeds without errors
        var result = KreuzcrawlLib.Scrape();
        Assert.Equal(200, result.StatusCode.Trim());
    }
}

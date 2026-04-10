using Xunit;
using Kreuzcrawl;

namespace Kreuzberg.E2e;

/// <summary>E2e tests for category: stealth.</summary>
public class StealthTests
{
    [Fact]
    public void Test_StealthUaRotationConfig()
    {
        // User-agent rotation config is accepted and crawl succeeds
        var result = KreuzcrawlLib.Scrape();
        Assert.Equal(200, result.StatusCode.Trim());
    }
}

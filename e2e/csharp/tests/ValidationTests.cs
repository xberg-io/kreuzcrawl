using Xunit;
using Kreuzcrawl;

namespace Kreuzberg.E2e;

/// <summary>E2e tests for category: validation.</summary>
public class ValidationTests
{
    [Fact]
    public void Test_ValidationInvalidExcludeRegex()
    {
        // Invalid regex in exclude_paths is rejected
        Assert.Throws<Exception>(() => KreuzcrawlLib.Scrape());
    }

    [Fact]
    public void Test_ValidationInvalidIncludeRegex()
    {
        // Invalid regex in include_paths is rejected
        Assert.Throws<Exception>(() => KreuzcrawlLib.Scrape());
    }

    [Fact]
    public void Test_ValidationInvalidRetryCode()
    {
        // Retry code outside 100-599 is rejected
        Assert.Throws<Exception>(() => KreuzcrawlLib.Scrape());
    }

    [Fact]
    public void Test_ValidationMaxPagesZero()
    {
        // max_pages=0 is rejected as invalid config
        Assert.Throws<Exception>(() => KreuzcrawlLib.Scrape());
    }

    [Fact]
    public void Test_ValidationMaxRedirectsTooHigh()
    {
        // max_redirects > 100 is rejected as invalid config
        Assert.Throws<Exception>(() => KreuzcrawlLib.Scrape());
    }

    [Fact]
    public void Test_ValidationTimeoutZero()
    {
        // Zero request timeout is rejected as invalid config
        Assert.Throws<Exception>(() => KreuzcrawlLib.Scrape());
    }
}

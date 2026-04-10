using Xunit;
using Kreuzcrawl;

namespace Kreuzberg.E2e;

/// <summary>E2e tests for category: cookies.</summary>
public class CookiesTests
{
    [Fact]
    public void Test_CookiesPerDomain()
    {
        // Isolates cookies per domain during crawl
        var result = KreuzcrawlLib.Scrape();
        Assert.Equal(1, result.Cookies.Count.Trim());
        Assert.Contains("domain_cookie", result.Cookies);
    }

    [Fact]
    public void Test_CookiesPersistence()
    {
        // Maintains cookies across multiple crawl requests
        var result = KreuzcrawlLib.Scrape();
        Assert.Contains("session", result.Cookies);
    }

    [Fact]
    public void Test_CookiesSetCookieResponse()
    {
        // Respects Set-Cookie header from server responses
        var result = KreuzcrawlLib.Scrape();
        Assert.Contains("tracking", result.Cookies);
    }
}

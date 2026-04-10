using Xunit;
using Kreuzcrawl;

namespace Kreuzberg.E2e;

/// <summary>E2e tests for category: auth.</summary>
public class AuthTests
{
    [Fact]
    public void Test_AuthBasicHttp()
    {
        // Sends HTTP Basic authentication header
        var result = KreuzcrawlLib.Scrape();
        Assert.Equal(true, result.AuthHeaderSent.Trim());
        Assert.Equal(200, result.StatusCode.Trim());
    }

    [Fact]
    public void Test_AuthBearerToken()
    {
        // Sends Bearer token in Authorization header
        var result = KreuzcrawlLib.Scrape();
        Assert.Equal(true, result.AuthHeaderSent.Trim());
        Assert.Equal(200, result.StatusCode.Trim());
    }

    [Fact]
    public void Test_AuthCustomHeader()
    {
        // Sends authentication via custom header (X-API-Key)
        var result = KreuzcrawlLib.Scrape();
        Assert.Equal(true, result.AuthHeaderSent.Trim());
        Assert.Equal(200, result.StatusCode.Trim());
    }
}

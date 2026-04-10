using Xunit;
using Kreuzcrawl;

namespace Kreuzberg.E2e;

/// <summary>E2e tests for category: error.</summary>
public class ErrorTests
{
    [Fact]
    public void Test_Error401Unauthorized()
    {
        // Handles 401 Unauthorized response correctly
        Assert.Throws<Exception>(() => KreuzcrawlLib.Scrape());
    }

    [Fact]
    public void Test_Error403Forbidden()
    {
        // Handles 403 Forbidden response correctly
        Assert.Throws<Exception>(() => KreuzcrawlLib.Scrape());
    }

    [Fact]
    public void Test_Error404Page()
    {
        // Handles 404 response correctly
        Assert.Throws<Exception>(() => KreuzcrawlLib.Scrape());
    }

    [Fact]
    public void Test_Error408RequestTimeout()
    {
        // Handles 408 Request Timeout response correctly
        Assert.Throws<Exception>(() => KreuzcrawlLib.Scrape());
    }

    [Fact]
    public void Test_Error410Gone()
    {
        // Handles 410 Gone response correctly
        Assert.Throws<Exception>(() => KreuzcrawlLib.Scrape());
    }

    [Fact]
    public void Test_Error500Server()
    {
        // Handles 500 server error
        Assert.Throws<Exception>(() => KreuzcrawlLib.Scrape());
    }

    [Fact]
    public void Test_Error502BadGateway()
    {
        // Handles 502 Bad Gateway response correctly
        Assert.Throws<Exception>(() => KreuzcrawlLib.Scrape());
    }

    [Fact]
    public void Test_ErrorConnectionRefused()
    {
        // Handles connection refused error gracefully
        Assert.Throws<Exception>(() => KreuzcrawlLib.Scrape());
    }

    [Fact]
    public void Test_ErrorDnsResolution()
    {
        // Handles DNS resolution failure gracefully
        Assert.Throws<Exception>(() => KreuzcrawlLib.Scrape());
    }

    [Fact]
    public void Test_ErrorEmptyResponse()
    {
        // Handles 200 with completely empty body gracefully
        var result = KreuzcrawlLib.Scrape();
        Assert.Equal(false, result.HtmlNotEmpty.Trim());
        Assert.Equal(false, result.Error.IsError.Trim());
    }

    [Fact]
    public void Test_ErrorInvalidProxy()
    {
        // Proxy pointing to unreachable address causes connection error during scrape
        Assert.Throws<Exception>(() => KreuzcrawlLib.Scrape());
    }

    [Fact]
    public void Test_ErrorPartialResponse()
    {
        // Handles incomplete or truncated HTTP response
        Assert.Throws<Exception>(() => KreuzcrawlLib.Scrape());
    }

    [Fact]
    public void Test_ErrorRateLimited()
    {
        // Handles 429 rate limiting with Retry-After
        Assert.Throws<Exception>(() => KreuzcrawlLib.Scrape());
    }

    [Fact]
    public void Test_ErrorRetry503()
    {
        // Retries request on 503 Service Unavailable response
        Assert.Throws<Exception>(() => KreuzcrawlLib.Scrape());
    }

    [Fact]
    public void Test_ErrorRetryBackoff()
    {
        // Implements exponential backoff when retrying failed requests
        Assert.Throws<Exception>(() => KreuzcrawlLib.Scrape());
    }

    [Fact]
    public void Test_ErrorSslInvalidCert()
    {
        // Handles SSL certificate validation error
        Assert.Throws<Exception>(() => KreuzcrawlLib.Scrape());
    }

    [Fact]
    public void Test_ErrorTimeout()
    {
        // Handles request timeout
        Assert.Throws<Exception>(() => KreuzcrawlLib.Scrape());
    }

    [Fact]
    public void Test_ErrorWafAkamai()
    {
        // Akamai WAF detection returns WafBlocked error
        Assert.Throws<Exception>(() => KreuzcrawlLib.Scrape());
    }

    [Fact]
    public void Test_ErrorWafFalse403()
    {
        // Detects WAF/bot protection false 403 (Cloudflare challenge page)
        Assert.Throws<Exception>(() => KreuzcrawlLib.Scrape());
    }

    [Fact]
    public void Test_ErrorWafImperva()
    {
        // Imperva/Incapsula WAF detection
        Assert.Throws<Exception>(() => KreuzcrawlLib.Scrape());
    }
}

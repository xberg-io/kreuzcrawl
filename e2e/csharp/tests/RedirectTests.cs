using Xunit;
using Kreuzcrawl;

namespace Kreuzberg.E2e;

/// <summary>E2e tests for category: redirect.</summary>
public class RedirectTests
{
    [Fact]
    public void Test_Redirect301Permanent()
    {
        // Follows 301 permanent redirect and returns final page content
        var result = KreuzcrawlLib.Scrape();
        Assert.Contains("/target", result.FinalUrl);
        Assert.Equal(1, result.RedirectCount.Trim());
    }

    [Fact]
    public void Test_Redirect302Found()
    {
        // Follows 302 Found redirect correctly
        var result = KreuzcrawlLib.Scrape();
        Assert.Contains("/found-target", result.FinalUrl);
        Assert.Equal(1, result.RedirectCount.Trim());
    }

    [Fact]
    public void Test_Redirect303SeeOther()
    {
        // Follows 303 See Other redirect (method changes to GET)
        var result = KreuzcrawlLib.Scrape();
        Assert.Contains("/see-other", result.FinalUrl);
        Assert.Equal(1, result.RedirectCount.Trim());
    }

    [Fact]
    public void Test_Redirect307Temporary()
    {
        // Follows 307 Temporary Redirect (preserves method)
        var result = KreuzcrawlLib.Scrape();
        Assert.Contains("/temp-target", result.FinalUrl);
        Assert.Equal(1, result.RedirectCount.Trim());
    }

    [Fact]
    public void Test_Redirect308Permanent()
    {
        // Follows 308 Permanent Redirect (preserves method)
        var result = KreuzcrawlLib.Scrape();
        Assert.Contains("/perm-target", result.FinalUrl);
        Assert.Equal(1, result.RedirectCount.Trim());
    }

    [Fact]
    public void Test_RedirectChain()
    {
        // Follows a chain of redirects (301 -> 302 -> 200)
        var result = KreuzcrawlLib.Scrape();
        Assert.Contains("/step2", result.FinalUrl);
        Assert.Equal(2, result.RedirectCount.Trim());
    }

    [Fact]
    public void Test_RedirectCrossDomain()
    {
        // Reports cross-domain redirect target without following to external domain
        var result = KreuzcrawlLib.Scrape();
        Assert.Contains("/external-redirect", result.FinalUrl);
        Assert.Equal(1, result.RedirectCount.Trim());
    }

    [Fact]
    public void Test_RedirectLoop()
    {
        // Detects redirect loop (A -> B -> A) and returns error
        var result = KreuzcrawlLib.Scrape();
        Assert.Equal(true, result.IsError.Trim());
    }

    [Fact]
    public void Test_RedirectMaxExceeded()
    {
        // Aborts when redirect count exceeds max_redirects limit
        var result = KreuzcrawlLib.Scrape();
        Assert.Equal(true, result.IsError.Trim());
    }

    [Fact]
    public void Test_RedirectMetaRefresh()
    {
        // Follows HTML meta-refresh redirect to target page
        var result = KreuzcrawlLib.Scrape();
        Assert.Contains("/target", result.FinalUrl);
        Assert.Equal(1, result.RedirectCount.Trim());
    }

    [Fact]
    public void Test_RedirectRefreshHeader()
    {
        // Handles HTTP Refresh header redirect
        var result = KreuzcrawlLib.Scrape();
        Assert.Contains("/refreshed", result.FinalUrl);
        Assert.Equal(1, result.RedirectCount.Trim());
    }

    [Fact]
    public void Test_RedirectTo404()
    {
        // Redirect target returns 404 Not Found
        var result = KreuzcrawlLib.Scrape();
        Assert.Contains("/gone", result.FinalUrl);
        Assert.Equal(1, result.RedirectCount.Trim());
        Assert.Equal(true, result.IsError.Trim());
    }
}

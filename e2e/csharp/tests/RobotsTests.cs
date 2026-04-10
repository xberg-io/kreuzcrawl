using Xunit;
using Kreuzcrawl;

namespace Kreuzberg.E2e;

/// <summary>E2e tests for category: robots.</summary>
public class RobotsTests
{
    [Fact]
    public void Test_RobotsAllowAll()
    {
        // Permissive robots.txt allows all paths
        var result = KreuzcrawlLib.Scrape();
        Assert.Equal(true, result.Robots.IsAllowed.Trim());
    }

    [Fact]
    public void Test_RobotsAllowOverride()
    {
        // Allow directive overrides Disallow for specific paths
        var result = KreuzcrawlLib.Scrape();
        Assert.Equal(true, result.Robots.IsAllowed.Trim());
    }

    [Fact]
    public void Test_RobotsCommentsHandling()
    {
        // Correctly parses robots.txt with inline and line comments
        var result = KreuzcrawlLib.Scrape();
        Assert.Equal(true, result.Robots.IsAllowed.Trim());
    }

    [Fact]
    public void Test_RobotsCrawlDelay()
    {
        // Respects crawl-delay directive from robots.txt
        var result = KreuzcrawlLib.Scrape();
        Assert.Equal(2, result.Robots.CrawlDelay.Trim());
    }

    [Fact]
    public void Test_RobotsDisallowPath()
    {
        // Robots.txt disallows specific paths
        var result = KreuzcrawlLib.Scrape();
        Assert.Equal(false, result.Robots.IsAllowed.Trim());
    }

    [Fact]
    public void Test_RobotsMetaNofollow()
    {
        // Detects nofollow meta robots tag and skips link extraction
        var result = KreuzcrawlLib.Scrape();
        Assert.Equal(true, result.Robots.NofollowDetected.Trim());
    }

    [Fact]
    public void Test_RobotsMetaNoindex()
    {
        // Detects noindex meta robots tag in HTML page
        var result = KreuzcrawlLib.Scrape();
        Assert.Equal(true, result.Robots.NoindexDetected.Trim());
    }

    [Fact]
    public void Test_RobotsMissing404()
    {
        // Missing robots.txt (404) allows all crawling
        var result = KreuzcrawlLib.Scrape();
        Assert.Equal(true, result.Robots.IsAllowed.Trim());
    }

    [Fact]
    public void Test_RobotsMultipleUserAgents()
    {
        // Picks the most specific user-agent block from robots.txt
        var result = KreuzcrawlLib.Scrape();
        Assert.Equal(true, result.Robots.IsAllowed.Trim());
    }

    [Fact]
    public void Test_RobotsRequestRate()
    {
        // Parses request-rate directive from robots.txt
        var result = KreuzcrawlLib.Scrape();
        Assert.Equal(5, result.Robots.CrawlDelay.Trim());
        Assert.Equal(true, result.Robots.IsAllowed.Trim());
    }

    [Fact]
    public void Test_RobotsSitemapDirective()
    {
        // Discovers sitemap URL from Sitemap directive in robots.txt
        var result = KreuzcrawlLib.Scrape();
        Assert.Equal(true, result.Robots.IsAllowed.Trim());
    }

    [Fact]
    public void Test_RobotsUserAgentSpecific()
    {
        // Matches user-agent specific rules in robots.txt
        var result = KreuzcrawlLib.Scrape();
        Assert.Equal(false, result.Robots.IsAllowed.Trim());
    }

    [Fact]
    public void Test_RobotsWildcardPaths()
    {
        // Handles wildcard Disallow patterns in robots.txt
        var result = KreuzcrawlLib.Scrape();
        Assert.Equal(false, result.Robots.IsAllowed.Trim());
    }

    [Fact]
    public void Test_RobotsXRobotsTag()
    {
        // Respects X-Robots-Tag HTTP header directives
        var result = KreuzcrawlLib.Scrape();
        Assert.Equal("noindex, nofollow", result.Robots.XRobotsTag.Trim());
        Assert.Equal(true, result.Robots.NoindexDetected.Trim());
        Assert.Equal(true, result.Robots.NofollowDetected.Trim());
    }
}

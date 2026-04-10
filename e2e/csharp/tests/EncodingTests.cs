using Xunit;
using Kreuzcrawl;

namespace Kreuzberg.E2e;

/// <summary>E2e tests for category: encoding.</summary>
public class EncodingTests
{
    [Fact]
    public void Test_EncodingDoubleEncoded()
    {
        // Handles double-encoded URL characters (%25C3%25B6)
        var result = KreuzcrawlLib.Scrape();
        Assert.NotEmpty(result.Html);
        Assert.True(result.Links.Count >= 1, "expected >= 1");
    }

    [Fact]
    public void Test_EncodingMixedCharsetPage()
    {
        // Handles charset mismatch between HTTP header and HTML meta tag
        var result = KreuzcrawlLib.Scrape();
        Assert.NotEmpty(result.Html);
    }

    [Fact]
    public void Test_EncodingPercentEncodedPath()
    {
        // Handles percent-encoded spaces and characters in URL paths
        var result = KreuzcrawlLib.Scrape();
        Assert.NotEmpty(result.Html);
        Assert.True(result.Links.Count >= 2, "expected >= 2");
    }

    [Fact]
    public void Test_EncodingUnicodeUrl()
    {
        // Handles Unicode characters in URLs (Hebrew, Japanese, Cyrillic)
        var result = KreuzcrawlLib.Scrape();
        Assert.NotEmpty(result.Html);
    }
}

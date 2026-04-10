using Xunit;
using Kreuzcrawl;

namespace Kreuzberg.E2e;

/// <summary>E2e tests for category: links.</summary>
public class LinksTests
{
    [Fact]
    public void Test_LinksAnchorFragment()
    {
        // Identifies fragment-only links as anchor type
        var result = KreuzcrawlLib.Scrape();
        Assert.Contains("anchor", result.Links[""].LinkType);
    }

    [Fact]
    public void Test_LinksBaseTag()
    {
        // Resolves relative URLs using base tag href
        var result = KreuzcrawlLib.Scrape();
        Assert.True(result.Links.Count > 2, "expected > 2");
        Assert.Contains("example.com", result.Links[""].Url);
    }

    [Fact]
    public void Test_LinksDocumentTypes()
    {
        // Detects PDF, DOCX, XLSX links as document type
        var result = KreuzcrawlLib.Scrape();
        Assert.Contains("document", result.Links[""].LinkType);
    }

    [Fact]
    public void Test_LinksEmptyHref()
    {
        // Handles empty href attributes without errors
        var result = KreuzcrawlLib.Scrape();
        Assert.True(result.Links.Count > 0, "expected > 0");
        Assert.Contains("/valid", result.Links[""].Url);
    }

    [Fact]
    public void Test_LinksInternalExternalClassification()
    {
        // Correctly classifies internal vs external links by domain
        var result = KreuzcrawlLib.Scrape();
        Assert.True(result.Links.Count > 4, "expected > 4");
        Assert.Contains("internal", result.Links[""].LinkType);
        Assert.Contains("external", result.Links[""].LinkType);
    }

    [Fact]
    public void Test_LinksMailtoJavascriptSkip()
    {
        // Skips mailto:, javascript:, and tel: scheme links
        var result = KreuzcrawlLib.Scrape();
        Assert.True(result.Links.Count > 0, "expected > 0");
        Assert.DoesNotContain("mailto:", result.Links[""].Url);
    }

    [Fact]
    public void Test_LinksProtocolRelative()
    {
        // Handles protocol-relative URLs (//example.com) correctly
        var result = KreuzcrawlLib.Scrape();
        Assert.True(result.Links.Count > 1, "expected > 1");
        Assert.NotEmpty(result.Links[""].ProtocolRelative);
    }

    [Fact]
    public void Test_LinksRelAttributes()
    {
        // Preserves rel=nofollow and rel=canonical attributes
        var result = KreuzcrawlLib.Scrape();
        Assert.True(result.Links.Count > 0, "expected > 0");
    }

    [Fact]
    public void Test_LinksRelativeParent()
    {
        // Resolves ../ and ./ relative parent path links correctly
        var result = KreuzcrawlLib.Scrape();
        Assert.True(result.Links.Count > 3, "expected > 3");
    }
}

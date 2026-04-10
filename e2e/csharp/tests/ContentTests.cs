using Xunit;
using Kreuzcrawl;

namespace Kreuzberg.E2e;

/// <summary>E2e tests for category: content.</summary>
public class ContentTests
{
    [Fact]
    public void Test_Content204NoContent()
    {
        // Handles 204 No Content response gracefully
        var result = KreuzcrawlLib.Scrape();
        Assert.Equal(204, result.StatusCode.Trim());
        Assert.Empty(result.Html);
    }

    [Fact]
    public void Test_ContentCharsetIso8859()
    {
        // Handles ISO-8859-1 encoded page correctly
        var result = KreuzcrawlLib.Scrape();
        Assert.Equal("iso-8859-1", result.Content.DetectedCharset.Trim());
    }

    [Fact]
    public void Test_ContentEmptyBody()
    {
        // Handles 200 response with empty body gracefully
        var result = KreuzcrawlLib.Scrape();
        Assert.Equal(200, result.StatusCode.Trim());
    }

    [Fact]
    public void Test_ContentGzipCompressed()
    {
        // Handles response with Accept-Encoding gzip negotiation
        var result = KreuzcrawlLib.Scrape();
        Assert.NotEmpty(result.Html);
        Assert.Equal(200, result.StatusCode.Trim());
    }

    [Fact]
    public void Test_ContentLargePageLimit()
    {
        // Respects max body size limit and truncates or skips oversized pages
        var result = KreuzcrawlLib.Scrape();
        Assert.True(result.Content.BodySize < 1025, "expected < 1025");
    }

    [Fact]
    public void Test_ContentMainOnly()
    {
        // Extracts only main content area, excluding nav, sidebar, footer
        var result = KreuzcrawlLib.Scrape();
        Assert.Equal(true, result.Content.MainContentOnly.Trim());
    }

    [Fact]
    public void Test_ContentPdfNoExtension()
    {
        // Detects PDF content by Content-Type header when URL has no .pdf extension
        var result = KreuzcrawlLib.Scrape();
        Assert.Equal(true, result.Content.IsPdf.Trim());
    }

    [Fact]
    public void Test_ContentRemoveTags()
    {
        // Removes specified HTML elements by CSS selector before processing
        var result = KreuzcrawlLib.Scrape();
        Assert.NotEmpty(result.Html);
    }

    [Fact]
    public void Test_ContentUtf8Bom()
    {
        // Handles UTF-8 content with BOM marker correctly
        var result = KreuzcrawlLib.Scrape();
        Assert.Equal("utf-8", result.Content.DetectedCharset.Trim());
        Assert.NotEmpty(result.Html);
    }
}

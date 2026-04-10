using Xunit;
using Kreuzcrawl;

namespace Kreuzberg.E2e;

/// <summary>E2e tests for category: markdown.</summary>
public class MarkdownTests
{
    [Fact]
    public void Test_MarkdownBasicConversion()
    {
        // HTML is always converted to markdown alongside raw HTML
        var result = KreuzcrawlLib.Scrape();
        Assert.Equal(200, result.StatusCode.Trim());
        Assert.Equal("Test", result.Metadata.Title.Trim());
        Assert.NotEmpty(result.Html);
        Assert.NotEmpty(result.Markdown);
        Assert.Contains("Hello World", result.Markdown);
    }

    [Fact]
    public void Test_MarkdownCrawlAllPages()
    {
        // All crawled pages have markdown field populated
        var result = KreuzcrawlLib.Scrape();
        Assert.Equal(2, result.Crawl.PagesCrawled.Trim());
    }

    [Fact]
    public void Test_MarkdownFitContent()
    {
        // Fit markdown removes navigation and boilerplate content
        var result = KreuzcrawlLib.Scrape();
        Assert.Equal(200, result.StatusCode.Trim());
        Assert.NotEmpty(result.Markdown);
    }

    [Fact]
    public void Test_MarkdownHeadingsAndParagraphs()
    {
        // Markdown conversion preserves heading hierarchy and paragraph text
        var result = KreuzcrawlLib.Scrape();
        Assert.NotEmpty(result.Markdown);
        Assert.Contains("Main Title", result.Markdown);
    }

    [Fact]
    public void Test_MarkdownLinksConverted()
    {
        // HTML links are converted to markdown link syntax
        var result = KreuzcrawlLib.Scrape();
        Assert.Equal(200, result.StatusCode.Trim());
        Assert.NotEmpty(result.Html);
        Assert.NotEmpty(result.Markdown);
        Assert.Contains("Example", result.Markdown);
    }

    [Fact]
    public void Test_MarkdownWithCitations()
    {
        // Markdown includes citation conversion with numbered references
        var result = KreuzcrawlLib.Scrape();
        Assert.Equal(200, result.StatusCode.Trim());
        Assert.NotEmpty(result.Markdown);
    }
}

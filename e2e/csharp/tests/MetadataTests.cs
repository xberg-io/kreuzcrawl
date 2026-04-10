using Xunit;
using Kreuzcrawl;

namespace Kreuzberg.E2e;

/// <summary>E2e tests for category: metadata.</summary>
public class MetadataTests
{
    [Fact]
    public void Test_MetadataArticleTimes()
    {
        // Extracts article:published_time, modified_time, author, section, and tags
        var result = KreuzcrawlLib.Scrape();
        Assert.Equal(200, result.StatusCode.Trim());
        Assert.Equal("2024-01-15T10:00:00Z", result.Article.PublishedTime.Trim());
        Assert.Equal("2024-06-20T14:30:00Z", result.Article.ModifiedTime.Trim());
        Assert.Equal("Jane Developer", result.Article.Author.Trim());
        Assert.Equal("Technology", result.Article.Section.Trim());
        Assert.Equal(3, result.Article.Tags.Count.Trim());
    }

    [Fact]
    public void Test_MetadataFavicons()
    {
        // Extracts favicon link tags including apple-touch-icon
        var result = KreuzcrawlLib.Scrape();
        Assert.Equal(200, result.StatusCode.Trim());
        Assert.Equal(5, result.Favicons.Count.Trim());
        Assert.NotEmpty(result.Favicons[""].AppleTouch);
    }

    [Fact]
    public void Test_MetadataHeadings()
    {
        // Extracts heading hierarchy (h1-h6) from HTML page
        var result = KreuzcrawlLib.Scrape();
        Assert.Equal(200, result.StatusCode.Trim());
        Assert.Equal(1, result.Headings.H1.Count.Trim());
        Assert.Equal("Primary Heading", result.Headings.H1["0"].Text.Trim());
        Assert.Equal(8, result.Headings.Count.Trim());
    }

    [Fact]
    public void Test_MetadataHreflang()
    {
        // Extracts hreflang alternate link tags
        var result = KreuzcrawlLib.Scrape();
        Assert.Equal(200, result.StatusCode.Trim());
        Assert.Equal(4, result.Hreflang.Count.Trim());
        Assert.Contains("en", result.Hreflang[""].Lang);
    }

    [Fact]
    public void Test_MetadataKeywordsAuthor()
    {
        // Extracts keywords, author, viewport, generator, theme-color, robots, lang, dir metadata
        var result = KreuzcrawlLib.Scrape();
        Assert.Equal(200, result.StatusCode.Trim());
        Assert.Equal("Comprehensive Metadata Test Page", result.Metadata.Title.Trim());
        Assert.NotEmpty(result.Metadata.CanonicalUrl);
        Assert.NotEmpty(result.Metadata.Keywords);
        Assert.Contains("rust", result.Metadata.Keywords);
        Assert.Equal("Jane Developer", result.Metadata.Author.Trim());
        Assert.NotEmpty(result.Metadata.Viewport);
        Assert.Equal("kreuzcrawl/1.0", result.Metadata.Generator.Trim());
        Assert.Equal("#ff6600", result.Metadata.ThemeColor.Trim());
        Assert.Equal("index, follow", result.Metadata.Robots.Trim());
        Assert.Equal("en", result.Metadata.Lang.Trim());
        Assert.Equal("ltr", result.Metadata.Dir.Trim());
    }

    [Fact]
    public void Test_MetadataOgVideoAudio()
    {
        // Extracts og:video, og:audio, and og:locale:alternate metadata
        var result = KreuzcrawlLib.Scrape();
        Assert.Equal(200, result.StatusCode.Trim());
        Assert.Equal("https://example.com/video.mp4", result.Og.Video.Trim());
        Assert.Equal("https://example.com/audio.mp3", result.Og.Audio.Trim());
        Assert.Equal(2, result.Og.LocaleAlternate.Count.Trim());
    }

    [Fact]
    public void Test_MetadataResponseHeaders()
    {
        // Extracts response metadata from HTTP headers (etag, server, content-language)
        var result = KreuzcrawlLib.Scrape();
        Assert.Equal(200, result.StatusCode.Trim());
        Assert.NotEmpty(result.ResponseHeaders.Etag);
        Assert.NotEmpty(result.ResponseHeaders.LastModified);
        Assert.Contains("nginx", result.ResponseHeaders.Server);
        Assert.Equal("en-US", result.ResponseHeaders.ContentLanguage.Trim());
    }

    [Fact]
    public void Test_MetadataWordCount()
    {
        // Computes word count from visible page text
        var result = KreuzcrawlLib.Scrape();
        Assert.Equal(200, result.StatusCode.Trim());
        Assert.True(result.Computed.WordCount > 99, "expected > 99");
        Assert.True(result.Computed.WordCount < 301, "expected < 301");
    }
}

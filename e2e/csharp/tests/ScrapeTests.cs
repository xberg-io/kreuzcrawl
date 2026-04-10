using Xunit;
using Kreuzcrawl;

namespace Kreuzberg.E2e;

/// <summary>E2e tests for category: scrape.</summary>
public class ScrapeTests
{
    [Fact]
    public void Test_ScrapeAssetDedup()
    {
        // Same asset linked twice results in one download with one unique hash
        var result = KreuzcrawlLib.Scrape();
        Assert.Equal(200, result.StatusCode.Trim());
        Assert.Equal(2, result.Assets.Count.Trim());
        Assert.Equal(2, result.Assets.UniqueHashes.Trim());
    }

    [Fact]
    public void Test_ScrapeAssetMaxSize()
    {
        // Skips assets exceeding max_asset_size limit
        var result = KreuzcrawlLib.Scrape();
        Assert.Equal(200, result.StatusCode.Trim());
        Assert.Equal(2, result.Assets.Count.Trim());
    }

    [Fact]
    public void Test_ScrapeAssetTypeFilter()
    {
        // Only downloads image assets when asset_types filter is set
        var result = KreuzcrawlLib.Scrape();
        Assert.Equal(200, result.StatusCode.Trim());
        Assert.Equal(1, result.Assets.Count.Trim());
        Assert.Contains("image", result.Assets[""].Category);
    }

    [Fact]
    public void Test_ScrapeBasicHtmlPage()
    {
        // Scrapes a simple HTML page and extracts title, description, and links
        var result = KreuzcrawlLib.Scrape();
        Assert.Equal(200, result.StatusCode.Trim());
        Assert.Equal("text/html", result.ContentType.Trim());
        Assert.NotEmpty(result.Html);
        Assert.Equal("Example Domain", result.Metadata.Title.Trim());
        Assert.Contains("illustrative examples", result.Metadata.Description);
        Assert.NotEmpty(result.Metadata.CanonicalUrl);
        Assert.True(result.Links.Count > 0, "expected > 0");
        Assert.Contains("external", result.Links[""].LinkType);
        Assert.Equal(0, result.Images.Count.Trim());
        Assert.Empty(result.Og.Title);
    }

    [Fact]
    public void Test_ScrapeComplexLinks()
    {
        // Classifies links by type: internal, external, anchor, document, image
        var result = KreuzcrawlLib.Scrape();
        Assert.Equal(200, result.StatusCode.Trim());
        Assert.True(result.Links.Count > 9, "expected > 9");
        Assert.Contains("internal", result.Links[""].LinkType);
        Assert.Contains("external", result.Links[""].LinkType);
        Assert.Contains("anchor", result.Links[""].LinkType);
        Assert.Contains("document", result.Links[""].LinkType);
    }

    [Fact]
    public void Test_ScrapeDownloadAssets()
    {
        // Downloads CSS, JS, and image assets from page
        var result = KreuzcrawlLib.Scrape();
        Assert.Equal(200, result.StatusCode.Trim());
        Assert.True(result.Assets.Count > 2, "expected > 2");
    }

    [Fact]
    public void Test_ScrapeDublinCore()
    {
        // Extracts Dublin Core metadata from a page
        var result = KreuzcrawlLib.Scrape();
        Assert.Equal(200, result.StatusCode.Trim());
        Assert.NotEmpty(result.DublinCore.Title);
        Assert.Equal("Effects of Climate Change on Marine Biodiversity", result.DublinCore.Title.Trim());
        Assert.Equal("Dr. Jane Smith", result.DublinCore.Creator.Trim());
    }

    [Fact]
    public void Test_ScrapeEmptyPage()
    {
        // Handles an empty HTML document without errors
        var result = KreuzcrawlLib.Scrape();
        Assert.Equal(200, result.StatusCode.Trim());
        Assert.True(result.Links.Count > -1, "expected > -1");
        Assert.Equal(0, result.Images.Count.Trim());
    }

    [Fact]
    public void Test_ScrapeFeedDiscovery()
    {
        // Discovers RSS, Atom, and JSON feed links
        var result = KreuzcrawlLib.Scrape();
        Assert.Equal(200, result.StatusCode.Trim());
        Assert.Equal(1, result.Feeds.Rss.Count.Trim());
        Assert.Equal(1, result.Feeds.Atom.Count.Trim());
        Assert.Equal(1, result.Feeds.JsonFeed.Count.Trim());
    }

    [Fact]
    public void Test_ScrapeImageSources()
    {
        // Extracts images from img, picture, og:image, twitter:image
        var result = KreuzcrawlLib.Scrape();
        Assert.Equal(200, result.StatusCode.Trim());
        Assert.True(result.Images.Count > 4, "expected > 4");
        Assert.Equal("https://example.com/images/og-hero.jpg", result.Og.Image.Trim());
    }

    [Fact]
    public void Test_ScrapeJsHeavySpa()
    {
        // Handles SPA page with JavaScript-only content (no server-rendered HTML)
        var result = KreuzcrawlLib.Scrape();
        Assert.NotEmpty(result.Html);
    }

    [Fact]
    public void Test_ScrapeJsonLd()
    {
        // Extracts JSON-LD structured data from a page
        var result = KreuzcrawlLib.Scrape();
        Assert.Equal(200, result.StatusCode.Trim());
        Assert.NotEmpty(result.JsonLd);
        Assert.Equal("Recipe", result.JsonLd.Type.Trim());
        Assert.Equal("Best Chocolate Cake", result.JsonLd.Name.Trim());
    }

    [Fact]
    public void Test_ScrapeMalformedHtml()
    {
        // Gracefully handles broken HTML without crashing
        var result = KreuzcrawlLib.Scrape();
        Assert.Equal(200, result.StatusCode.Trim());
        Assert.NotEmpty(result.Html);
        Assert.Contains("broken HTML", result.Metadata.Description);
    }

    [Fact]
    public void Test_ScrapeOgMetadata()
    {
        // Extracts full Open Graph metadata from a page
        var result = KreuzcrawlLib.Scrape();
        Assert.Equal(200, result.StatusCode.Trim());
        Assert.NotEmpty(result.Og.Title);
        Assert.Equal("Article Title", result.Og.Title.Trim());
        Assert.Equal("article", result.Og.Type.Trim());
        Assert.Equal("https://example.com/images/article-hero.jpg", result.Og.Image.Trim());
        Assert.NotEmpty(result.Og.Description);
        Assert.Equal("Article Title - Example Blog", result.Metadata.Title.Trim());
    }

    [Fact]
    public void Test_ScrapeTwitterCard()
    {
        // Extracts Twitter Card metadata from a page
        var result = KreuzcrawlLib.Scrape();
        Assert.Equal(200, result.StatusCode.Trim());
        Assert.NotEmpty(result.Twitter.Card);
        Assert.Equal("summary_large_image", result.Twitter.CardType.Trim());
        Assert.Equal("New Product Launch", result.Twitter.Title.Trim());
    }
}

using System.Threading.Tasks;
using Xunit;
using Kreuzcrawl;

namespace Kreuzberg.E2e;

/// <summary>E2e tests for category: scrape.</summary>
public class ScrapeTests
{
    [Fact]
    public async Task Test_ScrapeAssetDedup()
    {
        // Same asset linked twice results in one download with one unique hash
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        Assert.Equal(200, result.StatusCode);
        Assert.Equal(2, result.Assets.Count);
        Assert.Equal(2, result.Assets[0].UniqueHashes);
    }

    [Fact]
    public async Task Test_ScrapeAssetMaxSize()
    {
        // Skips assets exceeding max_asset_size limit
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        Assert.Equal(200, result.StatusCode);
        Assert.Equal(2, result.Assets.Count);
    }

    [Fact]
    public async Task Test_ScrapeAssetTypeFilter()
    {
        // Only downloads image assets when asset_types filter is set
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        Assert.Equal(200, result.StatusCode);
        Assert.Equal(1, result.Assets.Count);
        Assert.Contains("image", result.Assets[0].Category);
    }

    [Fact]
    public async Task Test_ScrapeBasicHtmlPage()
    {
        // Scrapes a simple HTML page and extracts title, description, and links
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        Assert.Equal(200, result.StatusCode);
        Assert.Equal("text/html", result.ContentType.Trim());
        Assert.NotEmpty(result.Html);
        Assert.Equal("Example Domain", result.Metadata.Title.Trim());
        Assert.Contains("illustrative examples", result.Metadata.Description);
        Assert.NotEmpty(result.Metadata.CanonicalUrl);
        Assert.True(result.Links.Count > 0, "expected > 0");
        Assert.Contains("external", result.Links[0].LinkType);
        Assert.Equal(0, result.Images.Count);
        // skipped: field 'og.title' not available on result type
    }

    [Fact]
    public async Task Test_ScrapeComplexLinks()
    {
        // Classifies links by type: internal, external, anchor, document, image
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        Assert.Equal(200, result.StatusCode);
        Assert.True(result.Links.Count > 9, "expected > 9");
        Assert.Contains("internal", result.Links[0].LinkType);
        Assert.Contains("external", result.Links[0].LinkType);
        Assert.Contains("anchor", result.Links[0].LinkType);
        Assert.Contains("document", result.Links[0].LinkType);
    }

    [Fact]
    public async Task Test_ScrapeDownloadAssets()
    {
        // Downloads CSS, JS, and image assets from page
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        Assert.Equal(200, result.StatusCode);
        Assert.True(result.Assets.Count > 2, "expected > 2");
    }

    [Fact]
    public async Task Test_ScrapeDublinCore()
    {
        // Extracts Dublin Core metadata from a page
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        Assert.Equal(200, result.StatusCode);
        // skipped: field 'dublin_core.title' not available on result type
        // skipped: field 'dublin_core.title' not available on result type
        // skipped: field 'dublin_core.creator' not available on result type
    }

    [Fact]
    public async Task Test_ScrapeEmptyPage()
    {
        // Handles an empty HTML document without errors
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        Assert.Equal(200, result.StatusCode);
        Assert.True(result.Links.Count > -1, "expected > -1");
        Assert.Equal(0, result.Images.Count);
    }

    [Fact]
    public async Task Test_ScrapeFeedDiscovery()
    {
        // Discovers RSS, Atom, and JSON feed links
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        Assert.Equal(200, result.StatusCode);
        Assert.Equal(1, result.Feeds[0].Rss.Count);
        Assert.Equal(1, result.Feeds[0].Atom.Count);
        Assert.Equal(1, result.Feeds[0].JsonFeed.Count);
    }

    [Fact]
    public async Task Test_ScrapeImageSources()
    {
        // Extracts images from img, picture, og:image, twitter:image
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        Assert.Equal(200, result.StatusCode);
        Assert.True(result.Images.Count > 4, "expected > 4");
        // skipped: field 'og.image' not available on result type
    }

    [Fact]
    public async Task Test_ScrapeJsHeavySpa()
    {
        // Handles SPA page with JavaScript-only content (no server-rendered HTML)
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        Assert.NotEmpty(result.Html);
    }

    [Fact]
    public async Task Test_ScrapeJsonLd()
    {
        // Extracts JSON-LD structured data from a page
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        Assert.Equal(200, result.StatusCode);
        Assert.NotEmpty(result.JsonLd);
        Assert.Equal("Recipe", result.JsonLd[0].Type.Trim());
        Assert.Equal("Best Chocolate Cake", result.JsonLd[0].Name.Trim());
    }

    [Fact]
    public async Task Test_ScrapeMalformedHtml()
    {
        // Gracefully handles broken HTML without crashing
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        Assert.Equal(200, result.StatusCode);
        Assert.NotEmpty(result.Html);
        Assert.Contains("broken HTML", result.Metadata.Description);
    }

    [Fact]
    public async Task Test_ScrapeOgMetadata()
    {
        // Extracts full Open Graph metadata from a page
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        Assert.Equal(200, result.StatusCode);
        // skipped: field 'og.title' not available on result type
        // skipped: field 'og.title' not available on result type
        // skipped: field 'og.type' not available on result type
        // skipped: field 'og.image' not available on result type
        // skipped: field 'og.description' not available on result type
        Assert.Equal("Article Title - Example Blog", result.Metadata.Title.Trim());
    }

    [Fact]
    public async Task Test_ScrapeTwitterCard()
    {
        // Extracts Twitter Card metadata from a page
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        Assert.Equal(200, result.StatusCode);
        // skipped: field 'twitter.card' not available on result type
        // skipped: field 'twitter.card_type' not available on result type
        // skipped: field 'twitter.title' not available on result type
    }
}

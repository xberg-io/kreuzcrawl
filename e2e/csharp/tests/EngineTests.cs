using System.Threading.Tasks;
using Xunit;
using Kreuzcrawl;

namespace Kreuzberg.E2e;

/// <summary>E2e tests for category: engine.</summary>
public class EngineTests
{
    [Fact]
    public async Task Test_EngineBatchBasic()
    {
        // CrawlEngine with defaults batch scrapes like the free function
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        // skipped: field 'batch.completed_count' not available on result type
        // skipped: field 'batch.total_count' not available on result type
    }

    [Fact]
    public async Task Test_EngineCrawlBasic()
    {
        // CrawlEngine with defaults crawls multiple pages like the free function
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        // skipped: field 'crawl.pages_crawled' not available on result type
        // skipped: field 'crawl.min_pages' not available on result type
    }

    [Fact]
    public async Task Test_EngineMapBasic()
    {
        // CrawlEngine with defaults discovers URLs like the free function
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        // skipped: field 'map.min_urls' not available on result type
    }

    [Fact]
    public async Task Test_EngineScrapeBasic()
    {
        // CrawlEngine with defaults scrapes a page identically to the free function
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        Assert.Equal(200, result.StatusCode);
        Assert.Equal("text/html", result.ContentType.Trim());
        Assert.Equal("Engine Test", result.Metadata.Title.Trim());
        Assert.Contains("Testing the engine", result.Metadata.Description);
        Assert.True(result.Links.Count >= 1, "expected >= 1");
        // skipped: field 'headings.h1_text' not available on result type
    }

    [Fact]
    public async Task Test_EngineStreamBasic()
    {
        // CrawlEngine with defaults streams events like the free function
        var engine = KreuzcrawlLib.CreateEngine(null);
        var result = await KreuzcrawlLib.Scrape(engine, "");
        // skipped: field 'stream.has_page_event' not available on result type
        // skipped: field 'stream.has_complete_event' not available on result type
        // skipped: field 'stream.event_count_min' not available on result type
    }
}

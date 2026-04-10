using Xunit;
using Kreuzcrawl;

namespace Kreuzberg.E2e;

/// <summary>E2e tests for category: stream.</summary>
public class StreamTests
{
    [Fact]
    public void Test_CrawlStreamEvents()
    {
        // Crawl stream produces page and complete events
        var result = KreuzcrawlLib.Scrape();
        Assert.True(result.Stream.EventCountMin >= 4, "expected >= 4");
        Assert.Equal(true, result.Stream.HasPageEvent.Trim());
        Assert.Equal(true, result.Stream.HasCompleteEvent.Trim());
    }

    [Fact]
    public void Test_StreamDepthCrawl()
    {
        // Stream produces events for multi-depth crawl with link following
        var result = KreuzcrawlLib.Scrape();
        Assert.True(result.Stream.EventCountMin >= 5, "expected >= 5");
        Assert.Equal(true, result.Stream.HasPageEvent.Trim());
        Assert.Equal(true, result.Stream.HasCompleteEvent.Trim());
    }

    [Fact]
    public void Test_StreamWithErrorEvent()
    {
        // Stream emits page and complete events even when some pages fail
        var result = KreuzcrawlLib.Scrape();
        Assert.Equal(true, result.Stream.HasPageEvent.Trim());
        Assert.Equal(true, result.Stream.HasCompleteEvent.Trim());
        Assert.True(result.Stream.EventCountMin >= 2, "expected >= 2");
    }
}

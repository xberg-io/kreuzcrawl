using Xunit;
using Kreuzcrawl;

namespace Kreuzberg.E2e;

/// <summary>E2e tests for category: batch.</summary>
public class BatchTests
{
    [Fact]
    public void Test_ScrapeBatchBasic()
    {
        // Batch scrape of multiple URLs all succeeding
        var result = KreuzcrawlLib.Scrape();
        Assert.Equal(3, result.Batch.CompletedCount.Trim());
        Assert.Equal(0, result.Batch.FailedCount.Trim());
        Assert.Equal(3, result.Batch.TotalCount.Trim());
    }

    [Fact]
    public void Test_ScrapeBatchPartialFailure()
    {
        // Batch scrape with one URL failing returns partial results
        var result = KreuzcrawlLib.Scrape();
        Assert.Equal(2, result.Batch.CompletedCount.Trim());
        Assert.Equal(1, result.Batch.FailedCount.Trim());
        Assert.Equal(3, result.Batch.TotalCount.Trim());
    }

    [Fact]
    public void Test_ScrapeBatchProgress()
    {
        // Batch scrape results include specific URL
        var result = KreuzcrawlLib.Scrape();
        Assert.Equal(2, result.Batch.TotalCount.Trim());
        Assert.Contains("/target", result.Batch.Results);
    }
}

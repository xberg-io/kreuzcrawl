# E2e tests for category: rate_limit
defmodule E2e.RateLimitTest do
  use ExUnit.Case, async: true

  describe "rate_limit_basic_delay" do
    test "Rate limiter adds delay between requests to the same domain" do
      result = Kreuzcrawl.scrape!()
      assert String.trim(result.crawl.pages_crawled) == 3
      assert result.rate_limit.min_duration_ms >= 150
    end
  end

  describe "rate_limit_zero_no_delay" do
    test "Rate limiter with zero delay does not slow crawling" do
      result = Kreuzcrawl.scrape!()
      assert String.trim(result.crawl.pages_crawled) == 2
    end
  end
end

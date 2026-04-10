# E2e tests for category: concurrent
defmodule E2e.ConcurrentTest do
  use ExUnit.Case, async: true

  describe "concurrent_basic" do
    test "Concurrent crawling fetches all pages with max_concurrent workers" do
      result = Kreuzcrawl.scrape!()
      assert String.trim(length(result.pages)) == 6
      assert length(result.pages) >= 6
    end
  end

  describe "concurrent_depth_two_fan_out" do
    test "Concurrent depth=2 crawl correctly fans out and deduplicates across levels" do
      result = Kreuzcrawl.scrape!()
      assert String.trim(length(result.pages)) == 4
    end
  end

  describe "concurrent_max_pages_exact" do
    test "Concurrent crawling does not exceed max_pages limit even with high concurrency" do
      result = Kreuzcrawl.scrape!()
      assert length(result.pages) <= 3
    end
  end

  describe "concurrent_partial_errors" do
    test "Concurrent crawl handles partial failures gracefully" do
      result = Kreuzcrawl.scrape!()
      assert length(result.pages) >= 2
    end
  end

  describe "concurrent_respects_max_pages" do
    test "Concurrent crawling respects max_pages limit" do
      result = Kreuzcrawl.scrape!()
      assert length(result.pages) <= 3
    end
  end
end

# E2e tests for category: engine
defmodule E2e.EngineTest do
  use ExUnit.Case, async: true

  describe "engine_batch_basic" do
    test "CrawlEngine with defaults batch scrapes like the free function" do
      result = Kreuzcrawl.scrape!()
      assert String.trim(result.batch.completed_count) == 2
      assert String.trim(result.batch.total_count) == 2
    end
  end

  describe "engine_crawl_basic" do
    test "CrawlEngine with defaults crawls multiple pages like the free function" do
      result = Kreuzcrawl.scrape!()
      assert String.trim(result.crawl.pages_crawled) == 3
      assert result.crawl.min_pages >= 3
    end
  end

  describe "engine_map_basic" do
    test "CrawlEngine with defaults discovers URLs like the free function" do
      result = Kreuzcrawl.scrape!()
      assert result.map.min_urls >= 2
    end
  end

  describe "engine_scrape_basic" do
    test "CrawlEngine with defaults scrapes a page identically to the free function" do
      result = Kreuzcrawl.scrape!()
      assert String.trim(result.status_code) == 200
      assert String.trim(result.content_type) == "text/html"
      assert String.trim(result.metadata.title) == "Engine Test"
      assert String.contains?(result.metadata.description_contains, "Testing the engine")
      assert result.links.min_count >= 1
      assert String.trim(result.headings.h1_count) == 1
      assert String.trim(result.headings.h1_text) == "Hello Engine"
    end
  end

  describe "engine_stream_basic" do
    test "CrawlEngine with defaults streams events like the free function" do
      result = Kreuzcrawl.scrape!()
      assert String.trim(result.stream.has_page_event) == true
      assert String.trim(result.stream.has_complete_event) == true
      assert result.stream.event_count_min >= 3
    end
  end
end

# E2e tests for category: engine
defmodule E2e.EngineTest do
  use ExUnit.Case, async: true

  describe "engine_batch_basic" do
    test "CrawlEngine with defaults batch scrapes like the free function" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      # skipped: field 'batch.completed_count' not available on result type
      # skipped: field 'batch.total_count' not available on result type
    end
  end

  describe "engine_crawl_basic" do
    test "CrawlEngine with defaults crawls multiple pages like the free function" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      # skipped: field 'crawl.pages_crawled' not available on result type
      # skipped: field 'crawl.min_pages' not available on result type
    end
  end

  describe "engine_map_basic" do
    test "CrawlEngine with defaults discovers URLs like the free function" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      # skipped: field 'map.min_urls' not available on result type
    end
  end

  describe "engine_scrape_basic" do
    test "CrawlEngine with defaults scrapes a page identically to the free function" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      assert String.trim(result.status_code) == 200
      assert String.trim(result.content_type) == "text/html"
      assert String.trim(result.metadata.title) == "Engine Test"
      assert String.contains?(result.metadata.description, "Testing the engine")
      assert length(result.links) >= 1
      # skipped: field 'headings.h1_text' not available on result type
    end
  end

  describe "engine_stream_basic" do
    test "CrawlEngine with defaults streams events like the free function" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      # skipped: field 'stream.has_page_event' not available on result type
      # skipped: field 'stream.has_complete_event' not available on result type
      # skipped: field 'stream.event_count_min' not available on result type
    end
  end
end

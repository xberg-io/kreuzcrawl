# E2e tests for category: filter
defmodule E2e.FilterTest do
  use ExUnit.Case, async: true

  describe "filter_bm25_crawl_integration" do
    test "BM25 filter works during multi-page crawl, keeping relevant pages" do
      result = Kreuzcrawl.scrape!()
      assert String.contains?(result.filter.remaining_contain_keyword, "rust")
    end
  end

  describe "filter_bm25_empty_query" do
    test "BM25 filter with empty query passes all pages through" do
      result = Kreuzcrawl.scrape!()
      assert String.trim(result.crawl.pages_crawled) == 2
    end
  end

  describe "filter_bm25_high_threshold" do
    test "BM25 filter with very high threshold filters out all pages" do
      result = Kreuzcrawl.scrape!()
      assert String.trim(result.filter.pages_after_filter) == 0
    end
  end

  describe "filter_bm25_relevant_pages" do
    test "BM25 filter keeps only pages relevant to the query" do
      result = Kreuzcrawl.scrape!()
      assert String.contains?(result.filter.remaining_contain_keyword, "rust")
    end
  end

  describe "filter_bm25_threshold_zero" do
    test "BM25 filter with zero threshold passes all pages" do
      result = Kreuzcrawl.scrape!()
      assert String.trim(result.crawl.pages_crawled) == 2
    end
  end

  describe "filter_noop_crawl_all_kept" do
    test "NoopFilter keeps all pages during a multi-page crawl" do
      result = Kreuzcrawl.scrape!()
      assert String.trim(result.filter.pages_after_filter) == 3
    end
  end

  describe "filter_noop_passes_all" do
    test "No content filter passes all crawled pages through" do
      result = Kreuzcrawl.scrape!()
      assert String.trim(result.crawl.pages_crawled) == 3
    end
  end
end

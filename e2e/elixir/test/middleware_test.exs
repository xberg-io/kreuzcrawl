# E2e tests for category: middleware
defmodule E2e.MiddlewareTest do
  use ExUnit.Case, async: true

  describe "middleware_engine_crawl_with_defaults" do
    test "Engine crawl with default middleware chain produces correct multi-page results" do
      result = Kreuzcrawl.scrape!()
      assert String.trim(result.crawl.pages_crawled) == 3
      assert result.crawl.min_pages >= 3
    end
  end

  describe "middleware_noop_no_effect" do
    test "Default middleware chain does not affect normal scraping" do
      result = Kreuzcrawl.scrape!()
      assert String.trim(result.status_code) == 200
      assert String.trim(result.metadata.title) == "Middleware Test"
    end
  end
end

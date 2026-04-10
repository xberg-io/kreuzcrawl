# E2e tests for category: map
defmodule E2e.MapTest do
  use ExUnit.Case, async: true

  describe "map_discover_urls" do
    test "Discovers all URLs on a site without fetching full content" do
      result = Kreuzcrawl.scrape!()
      assert length(result.urls) >= 3
    end
  end

  describe "map_exclude_patterns" do
    test "Excludes URLs matching patterns from URL map" do
      result = Kreuzcrawl.scrape!()
      assert String.trim(length(result.urls)) == 1
    end
  end

  describe "map_include_subdomains" do
    test "Includes subdomain URLs in URL map discovery" do
      result = Kreuzcrawl.scrape!()
      assert length(result.urls) >= 2
      assert String.contains?(result.urls, "blog.example.com")
    end
  end

  describe "map_large_sitemap" do
    test "Handles large sitemap with 100+ URLs" do
      result = Kreuzcrawl.scrape!()
      assert length(result.urls) >= 100
    end
  end

  describe "map_limit_pagination" do
    test "Limits map result count to specified maximum" do
      result = Kreuzcrawl.scrape!()
      assert length(result.urls) <= 5
    end
  end

  describe "map_search_filter" do
    test "Filters map results by search keyword" do
      result = Kreuzcrawl.scrape!()
      assert length(result.urls) >= 2
      assert String.contains?(result.urls, "blog")
    end
  end
end

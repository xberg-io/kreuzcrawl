# E2e tests for category: sitemap
defmodule E2e.SitemapTest do
  use ExUnit.Case, async: true

  describe "sitemap_basic" do
    test "Parses a standard urlset sitemap" do
      result = Kreuzcrawl.scrape!()
      assert String.trim(length(result.urls)) == 4
      assert String.trim(result.has_lastmod) == true
    end
  end

  describe "sitemap_compressed_gzip" do
    test "Parses a gzip-compressed sitemap file" do
      result = Kreuzcrawl.scrape!()
      assert String.trim(length(result.urls)) == 3
    end
  end

  describe "sitemap_empty" do
    test "Handles empty sitemap gracefully" do
      result = Kreuzcrawl.scrape!()
      assert String.trim(length(result.urls)) == 0
    end
  end

  describe "sitemap_from_robots_txt" do
    test "Discovers sitemap via robots.txt Sitemap directive" do
      result = Kreuzcrawl.scrape!()
      assert String.trim(length(result.urls)) == 4
    end
  end

  describe "sitemap_index" do
    test "Follows sitemap index to discover child sitemaps" do
      result = Kreuzcrawl.scrape!()
      assert String.trim(length(result.urls)) == 3
    end
  end

  describe "sitemap_lastmod_filter" do
    test "Filters sitemap URLs by lastmod date" do
      result = Kreuzcrawl.scrape!()
      assert String.trim(length(result.urls)) == 4
      assert String.trim(result.has_lastmod) == true
    end
  end

  describe "sitemap_only_mode" do
    test "Uses sitemap URLs exclusively without following page links" do
      result = Kreuzcrawl.scrape!()
      assert String.trim(length(result.urls)) == 4
    end
  end

  describe "sitemap_xhtml_links" do
    test "Parses sitemap with XHTML namespace alternate links" do
      result = Kreuzcrawl.scrape!()
      assert String.trim(length(result.urls)) == 2
      assert String.trim(result.has_lastmod) == false
    end
  end
end

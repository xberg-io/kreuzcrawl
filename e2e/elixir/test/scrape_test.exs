# E2e tests for category: scrape
defmodule E2e.ScrapeTest do
  use ExUnit.Case, async: true

  describe "scrape_asset_dedup" do
    test "Same asset linked twice results in one download with one unique hash" do
      result = Kreuzcrawl.scrape!()
      assert String.trim(result.status_code) == 200
      assert String.trim(length(result.assets)) == 2
      assert String.trim(result.assets.unique_hashes) == 2
    end
  end

  describe "scrape_asset_max_size" do
    test "Skips assets exceeding max_asset_size limit" do
      result = Kreuzcrawl.scrape!()
      assert String.trim(result.status_code) == 200
      assert String.trim(length(result.assets)) == 2
    end
  end

  describe "scrape_asset_type_filter" do
    test "Only downloads image assets when asset_types filter is set" do
      result = Kreuzcrawl.scrape!()
      assert String.trim(result.status_code) == 200
      assert String.trim(length(result.assets)) == 1
      assert String.contains?(result.assets.get("").category, "image")
    end
  end

  describe "scrape_basic_html_page" do
    test "Scrapes a simple HTML page and extracts title, description, and links" do
      result = Kreuzcrawl.scrape!()
      assert String.trim(result.status_code) == 200
      assert String.trim(result.content_type) == "text/html"
      assert result.html != ""
      assert String.trim(result.metadata.title) == "Example Domain"
      assert String.contains?(result.metadata.description, "illustrative examples")
      assert result.metadata.canonical_url != ""
      assert length(result.links) > 0
      assert String.contains?(result.links.get("").link_type, "external")
      assert String.trim(length(result.images)) == 0
      assert String.trim(result.og.title) == ""
    end
  end

  describe "scrape_complex_links" do
    test "Classifies links by type: internal, external, anchor, document, image" do
      result = Kreuzcrawl.scrape!()
      assert String.trim(result.status_code) == 200
      assert length(result.links) > 9
      assert String.contains?(result.links.get("").link_type, "internal")
      assert String.contains?(result.links.get("").link_type, "external")
      assert String.contains?(result.links.get("").link_type, "anchor")
      assert String.contains?(result.links.get("").link_type, "document")
    end
  end

  describe "scrape_download_assets" do
    test "Downloads CSS, JS, and image assets from page" do
      result = Kreuzcrawl.scrape!()
      assert String.trim(result.status_code) == 200
      assert length(result.assets) > 2
    end
  end

  describe "scrape_dublin_core" do
    test "Extracts Dublin Core metadata from a page" do
      result = Kreuzcrawl.scrape!()
      assert String.trim(result.status_code) == 200
      assert result.dublin_core.title != ""
      assert String.trim(result.dublin_core.title) == "Effects of Climate Change on Marine Biodiversity"
      assert String.trim(result.dublin_core.creator) == "Dr. Jane Smith"
    end
  end

  describe "scrape_empty_page" do
    test "Handles an empty HTML document without errors" do
      result = Kreuzcrawl.scrape!()
      assert String.trim(result.status_code) == 200
      assert length(result.links) > -1
      assert String.trim(length(result.images)) == 0
    end
  end

  describe "scrape_feed_discovery" do
    test "Discovers RSS, Atom, and JSON feed links" do
      result = Kreuzcrawl.scrape!()
      assert String.trim(result.status_code) == 200
      assert String.trim(length(result.feeds.rss)) == 1
      assert String.trim(length(result.feeds.atom)) == 1
      assert String.trim(length(result.feeds.json_feed)) == 1
    end
  end

  describe "scrape_image_sources" do
    test "Extracts images from img, picture, og:image, twitter:image" do
      result = Kreuzcrawl.scrape!()
      assert String.trim(result.status_code) == 200
      assert length(result.images) > 4
      assert String.trim(result.og.image) == "https://example.com/images/og-hero.jpg"
    end
  end

  describe "scrape_js_heavy_spa" do
    test "Handles SPA page with JavaScript-only content (no server-rendered HTML)" do
      result = Kreuzcrawl.scrape!()
      assert result.html != ""
    end
  end

  describe "scrape_json_ld" do
    test "Extracts JSON-LD structured data from a page" do
      result = Kreuzcrawl.scrape!()
      assert String.trim(result.status_code) == 200
      assert result.json_ld != ""
      assert String.trim(result.json_ld.type) == "Recipe"
      assert String.trim(result.json_ld.name) == "Best Chocolate Cake"
    end
  end

  describe "scrape_malformed_html" do
    test "Gracefully handles broken HTML without crashing" do
      result = Kreuzcrawl.scrape!()
      assert String.trim(result.status_code) == 200
      assert result.html != ""
      assert String.contains?(result.metadata.description, "broken HTML")
    end
  end

  describe "scrape_og_metadata" do
    test "Extracts full Open Graph metadata from a page" do
      result = Kreuzcrawl.scrape!()
      assert String.trim(result.status_code) == 200
      assert result.og.title != ""
      assert String.trim(result.og.title) == "Article Title"
      assert String.trim(result.og.type) == "article"
      assert String.trim(result.og.image) == "https://example.com/images/article-hero.jpg"
      assert result.og.description != ""
      assert String.trim(result.metadata.title) == "Article Title - Example Blog"
    end
  end

  describe "scrape_twitter_card" do
    test "Extracts Twitter Card metadata from a page" do
      result = Kreuzcrawl.scrape!()
      assert String.trim(result.status_code) == 200
      assert result.twitter.card != ""
      assert String.trim(result.twitter.card_type) == "summary_large_image"
      assert String.trim(result.twitter.title) == "New Product Launch"
    end
  end
end

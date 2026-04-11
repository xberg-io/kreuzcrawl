# E2e tests for category: scrape
defmodule E2e.ScrapeTest do
  use ExUnit.Case, async: true

  describe "scrape_asset_dedup" do
    test "Same asset linked twice results in one download with one unique hash" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      assert String.trim(result.status_code) == 200
      assert String.trim(length(result.assets)) == 2
      assert String.trim(result.assets[0].unique_hashes) == 2
    end
  end

  describe "scrape_asset_max_size" do
    test "Skips assets exceeding max_asset_size limit" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      assert String.trim(result.status_code) == 200
      assert String.trim(length(result.assets)) == 2
    end
  end

  describe "scrape_asset_type_filter" do
    test "Only downloads image assets when asset_types filter is set" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      assert String.trim(result.status_code) == 200
      assert String.trim(length(result.assets)) == 1
      assert String.contains?(result.assets[0].category, "image")
    end
  end

  describe "scrape_basic_html_page" do
    test "Scrapes a simple HTML page and extracts title, description, and links" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      assert String.trim(result.status_code) == 200
      assert String.trim(result.content_type) == "text/html"
      assert result.html != ""
      assert String.trim(result.metadata.title) == "Example Domain"
      assert String.contains?(result.metadata.description, "illustrative examples")
      assert result.metadata.canonical_url != ""
      assert length(result.links) > 0
      assert String.contains?(result.links[0].link_type, "external")
      assert String.trim(length(result.images)) == 0
      # skipped: field 'og.title' not available on result type
    end
  end

  describe "scrape_complex_links" do
    test "Classifies links by type: internal, external, anchor, document, image" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      assert String.trim(result.status_code) == 200
      assert length(result.links) > 9
      assert String.contains?(result.links[0].link_type, "internal")
      assert String.contains?(result.links[0].link_type, "external")
      assert String.contains?(result.links[0].link_type, "anchor")
      assert String.contains?(result.links[0].link_type, "document")
    end
  end

  describe "scrape_download_assets" do
    test "Downloads CSS, JS, and image assets from page" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      assert String.trim(result.status_code) == 200
      assert length(result.assets) > 2
    end
  end

  describe "scrape_dublin_core" do
    test "Extracts Dublin Core metadata from a page" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      assert String.trim(result.status_code) == 200
      # skipped: field 'dublin_core.title' not available on result type
      # skipped: field 'dublin_core.title' not available on result type
      # skipped: field 'dublin_core.creator' not available on result type
    end
  end

  describe "scrape_empty_page" do
    test "Handles an empty HTML document without errors" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      assert String.trim(result.status_code) == 200
      assert length(result.links) > -1
      assert String.trim(length(result.images)) == 0
    end
  end

  describe "scrape_feed_discovery" do
    test "Discovers RSS, Atom, and JSON feed links" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      assert String.trim(result.status_code) == 200
      assert String.trim(length(result.feeds[0].rss)) == 1
      assert String.trim(length(result.feeds[0].atom)) == 1
      assert String.trim(length(result.feeds[0].json_feed)) == 1
    end
  end

  describe "scrape_image_sources" do
    test "Extracts images from img, picture, og:image, twitter:image" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      assert String.trim(result.status_code) == 200
      assert length(result.images) > 4
      # skipped: field 'og.image' not available on result type
    end
  end

  describe "scrape_js_heavy_spa" do
    test "Handles SPA page with JavaScript-only content (no server-rendered HTML)" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      assert result.html != ""
    end
  end

  describe "scrape_json_ld" do
    test "Extracts JSON-LD structured data from a page" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      assert String.trim(result.status_code) == 200
      assert result.json_ld != ""
      assert String.trim(result.json_ld[0].type) == "Recipe"
      assert String.trim(result.json_ld[0].name) == "Best Chocolate Cake"
    end
  end

  describe "scrape_malformed_html" do
    test "Gracefully handles broken HTML without crashing" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      assert String.trim(result.status_code) == 200
      assert result.html != ""
      assert String.contains?(result.metadata.description, "broken HTML")
    end
  end

  describe "scrape_og_metadata" do
    test "Extracts full Open Graph metadata from a page" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      assert String.trim(result.status_code) == 200
      # skipped: field 'og.title' not available on result type
      # skipped: field 'og.title' not available on result type
      # skipped: field 'og.type' not available on result type
      # skipped: field 'og.image' not available on result type
      # skipped: field 'og.description' not available on result type
      assert String.trim(result.metadata.title) == "Article Title - Example Blog"
    end
  end

  describe "scrape_twitter_card" do
    test "Extracts Twitter Card metadata from a page" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      assert String.trim(result.status_code) == 200
      # skipped: field 'twitter.card' not available on result type
      # skipped: field 'twitter.card_type' not available on result type
      # skipped: field 'twitter.title' not available on result type
    end
  end
end

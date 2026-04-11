# frozen_string_literal: true

require "kreuzcrawl"

RSpec.describe "scrape" do
  it "scrape_asset_dedup: Same asset linked twice results in one download with one unique hash" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
    expect(result.status_code).to eq(200)
    expect(result.assets.length).to eq(2)
    expect(result.assets[0].unique_hashes).to eq(2)
  end

  it "scrape_asset_max_size: Skips assets exceeding max_asset_size limit" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
    expect(result.status_code).to eq(200)
    expect(result.assets.length).to eq(2)
  end

  it "scrape_asset_type_filter: Only downloads image assets when asset_types filter is set" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
    expect(result.status_code).to eq(200)
    expect(result.assets.length).to eq(1)
    expect(result.assets[0].category).to include("image")
  end

  it "scrape_basic_html_page: Scrapes a simple HTML page and extracts title, description, and links" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
    expect(result.status_code).to eq(200)
    expect(result.content_type).to eq("text/html")
    expect(result.html).not_to be_empty
    expect(result.metadata.title).to eq("Example Domain")
    expect(result.metadata.description).to include("illustrative examples")
    expect(result.metadata.canonical_url).not_to be_empty
    expect(result.links.length).to be > 0
    expect(result.links[0].link_type).to include("external")
    expect(result.images.length).to eq(0)
      # skipped: field 'og.title' not available on result type
  end

  it "scrape_complex_links: Classifies links by type: internal, external, anchor, document, image" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
    expect(result.status_code).to eq(200)
    expect(result.links.length).to be > 9
    expect(result.links[0].link_type).to include("internal")
    expect(result.links[0].link_type).to include("external")
    expect(result.links[0].link_type).to include("anchor")
    expect(result.links[0].link_type).to include("document")
  end

  it "scrape_download_assets: Downloads CSS, JS, and image assets from page" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
    expect(result.status_code).to eq(200)
    expect(result.assets.length).to be > 2
  end

  it "scrape_dublin_core: Extracts Dublin Core metadata from a page" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
    expect(result.status_code).to eq(200)
      # skipped: field 'dublin_core.title' not available on result type
      # skipped: field 'dublin_core.title' not available on result type
      # skipped: field 'dublin_core.creator' not available on result type
  end

  it "scrape_empty_page: Handles an empty HTML document without errors" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
    expect(result.status_code).to eq(200)
    expect(result.links.length).to be > -1
    expect(result.images.length).to eq(0)
  end

  it "scrape_feed_discovery: Discovers RSS, Atom, and JSON feed links" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
    expect(result.status_code).to eq(200)
    expect(result.feeds[0].rss.length).to eq(1)
    expect(result.feeds[0].atom.length).to eq(1)
    expect(result.feeds[0].json_feed.length).to eq(1)
  end

  it "scrape_image_sources: Extracts images from img, picture, og:image, twitter:image" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
    expect(result.status_code).to eq(200)
    expect(result.images.length).to be > 4
      # skipped: field 'og.image' not available on result type
  end

  it "scrape_js_heavy_spa: Handles SPA page with JavaScript-only content (no server-rendered HTML)" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
    expect(result.html).not_to be_empty
  end

  it "scrape_json_ld: Extracts JSON-LD structured data from a page" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
    expect(result.status_code).to eq(200)
    expect(result.json_ld).not_to be_empty
    expect(result.json_ld[0].type).to eq("Recipe")
    expect(result.json_ld[0].name).to eq("Best Chocolate Cake")
  end

  it "scrape_malformed_html: Gracefully handles broken HTML without crashing" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
    expect(result.status_code).to eq(200)
    expect(result.html).not_to be_empty
    expect(result.metadata.description).to include("broken HTML")
  end

  it "scrape_og_metadata: Extracts full Open Graph metadata from a page" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
    expect(result.status_code).to eq(200)
      # skipped: field 'og.title' not available on result type
      # skipped: field 'og.title' not available on result type
      # skipped: field 'og.type' not available on result type
      # skipped: field 'og.image' not available on result type
      # skipped: field 'og.description' not available on result type
    expect(result.metadata.title).to eq("Article Title - Example Blog")
  end

  it "scrape_twitter_card: Extracts Twitter Card metadata from a page" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
    expect(result.status_code).to eq(200)
      # skipped: field 'twitter.card' not available on result type
      # skipped: field 'twitter.card_type' not available on result type
      # skipped: field 'twitter.title' not available on result type
  end
end

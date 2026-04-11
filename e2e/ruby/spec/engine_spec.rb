# frozen_string_literal: true

require "kreuzcrawl"

RSpec.describe "engine" do
  it "engine_batch_basic: CrawlEngine with defaults batch scrapes like the free function" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
      # skipped: field 'batch.completed_count' not available on result type
      # skipped: field 'batch.total_count' not available on result type
  end

  it "engine_crawl_basic: CrawlEngine with defaults crawls multiple pages like the free function" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
      # skipped: field 'crawl.pages_crawled' not available on result type
      # skipped: field 'crawl.min_pages' not available on result type
  end

  it "engine_map_basic: CrawlEngine with defaults discovers URLs like the free function" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
      # skipped: field 'map.min_urls' not available on result type
  end

  it "engine_scrape_basic: CrawlEngine with defaults scrapes a page identically to the free function" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
    expect(result.status_code).to eq(200)
    expect(result.content_type).to eq("text/html")
    expect(result.metadata.title).to eq("Engine Test")
    expect(result.metadata.description).to include("Testing the engine")
    expect(result.links.length).to be >= 1
      # skipped: field 'headings.h1_text' not available on result type
  end

  it "engine_stream_basic: CrawlEngine with defaults streams events like the free function" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
      # skipped: field 'stream.has_page_event' not available on result type
      # skipped: field 'stream.has_complete_event' not available on result type
      # skipped: field 'stream.event_count_min' not available on result type
  end
end

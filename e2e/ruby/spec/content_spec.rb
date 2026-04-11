# frozen_string_literal: true

require 'kreuzcrawl'

RSpec.describe 'content' do
  it 'content_204_no_content: Handles 204 No Content response gracefully' do
    engine = Kreuzcrawl.create_engine(nil)
    url = "#{ENV.fetch('MOCK_SERVER_URL')}/fixtures/content_204_no_content"
    result = Kreuzcrawl.scrape(engine, url)
    expect(result.status_code).to eq(204)
    expect(result.html).to be_empty
  end

  it 'content_charset_iso8859: Handles ISO-8859-1 encoded page correctly' do
    engine = Kreuzcrawl.create_engine(nil)
    url = "#{ENV.fetch('MOCK_SERVER_URL')}/fixtures/content_charset_iso8859"
    result = Kreuzcrawl.scrape(engine, url)
    expect(result.detected_charset).to eq('iso-8859-1')
  end

  it 'content_empty_body: Handles 200 response with empty body gracefully' do
    engine = Kreuzcrawl.create_engine(nil)
    url = "#{ENV.fetch('MOCK_SERVER_URL')}/fixtures/content_empty_body"
    result = Kreuzcrawl.scrape(engine, url)
    expect(result.status_code).to eq(200)
  end

  it 'content_gzip_compressed: Handles response with Accept-Encoding gzip negotiation' do
    engine = Kreuzcrawl.create_engine(nil)
    url = "#{ENV.fetch('MOCK_SERVER_URL')}/fixtures/content_gzip_compressed"
    result = Kreuzcrawl.scrape(engine, url)
    expect(result.html).not_to be_empty
    expect(result.status_code).to eq(200)
  end

  it 'content_large_page_limit: Respects max body size limit and truncates or skips oversized pages' do
    engine = Kreuzcrawl.create_engine(nil)
    url = "#{ENV.fetch('MOCK_SERVER_URL')}/fixtures/content_large_page_limit"
    result = Kreuzcrawl.scrape(engine, url)
    expect(result.body_size).to be < 1025
  end

  it 'content_main_only: Extracts only main content area, excluding nav, sidebar, footer' do
    engine = Kreuzcrawl.create_engine(nil)
    url = "#{ENV.fetch('MOCK_SERVER_URL')}/fixtures/content_main_only"
    result = Kreuzcrawl.scrape(engine, url)
    expect(result.main_content_only).to be(true)
  end

  it 'content_pdf_no_extension: Detects PDF content by Content-Type header when URL has no .pdf extension' do
    engine = Kreuzcrawl.create_engine(nil)
    url = "#{ENV.fetch('MOCK_SERVER_URL')}/fixtures/content_pdf_no_extension"
    result = Kreuzcrawl.scrape(engine, url)
    expect(result.is_pdf).to be(true)
  end

  it 'content_remove_tags: Removes specified HTML elements by CSS selector before processing' do
    engine = Kreuzcrawl.create_engine(nil)
    url = "#{ENV.fetch('MOCK_SERVER_URL')}/fixtures/content_remove_tags"
    result = Kreuzcrawl.scrape(engine, url)
    expect(result.html).not_to be_empty
  end

  it 'content_utf8_bom: Handles UTF-8 content with BOM marker correctly' do
    engine = Kreuzcrawl.create_engine(nil)
    url = "#{ENV.fetch('MOCK_SERVER_URL')}/fixtures/content_utf8_bom"
    result = Kreuzcrawl.scrape(engine, url)
    expect(result.detected_charset).to eq('utf-8')
    expect(result.html).not_to be_empty
  end
end

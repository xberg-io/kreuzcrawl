# frozen_string_literal: true

require 'kreuzcrawl'

RSpec.describe 'crawl' do
  it 'content_binary_skip: Skips image and video content types gracefully' do
    engine = Kreuzcrawl.create_engine(nil)
    url = "#{ENV.fetch('MOCK_SERVER_URL')}/fixtures/content_binary_skip"
    result = Kreuzcrawl.scrape(engine, url)
    expect(result.was_skipped).to be(true)
  end

  it 'content_pdf_link_skip: Encounters PDF link and skips or marks as document type' do
    engine = Kreuzcrawl.create_engine(nil)
    url = "#{ENV.fetch('MOCK_SERVER_URL')}/fixtures/content_pdf_link_skip"
    result = Kreuzcrawl.scrape(engine, url)
    expect(result.was_skipped).to be(true)
  end
end

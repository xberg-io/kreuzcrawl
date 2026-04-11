# frozen_string_literal: true

require 'kreuzcrawl'

RSpec.describe 'encoding' do
  it 'encoding_double_encoded: Handles double-encoded URL characters (%25C3%25B6)' do
    engine = Kreuzcrawl.create_engine(nil)
    url = "#{ENV.fetch('MOCK_SERVER_URL')}/fixtures/encoding_double_encoded"
    result = Kreuzcrawl.scrape(engine, url)
    expect(result.html).not_to be_empty
    expect(result.links.length).to be >= 1
  end

  it 'encoding_mixed_charset_page: Handles charset mismatch between HTTP header and HTML meta tag' do
    engine = Kreuzcrawl.create_engine(nil)
    url = "#{ENV.fetch('MOCK_SERVER_URL')}/fixtures/encoding_mixed_charset_page"
    result = Kreuzcrawl.scrape(engine, url)
    expect(result.html).not_to be_empty
  end

  it 'encoding_percent_encoded_path: Handles percent-encoded spaces and characters in URL paths' do
    engine = Kreuzcrawl.create_engine(nil)
    url = "#{ENV.fetch('MOCK_SERVER_URL')}/fixtures/encoding_percent_encoded_path"
    result = Kreuzcrawl.scrape(engine, url)
    expect(result.html).not_to be_empty
    expect(result.links.length).to be >= 2
  end

  it 'encoding_unicode_url: Handles Unicode characters in URLs (Hebrew, Japanese, Cyrillic)' do
    engine = Kreuzcrawl.create_engine(nil)
    url = "#{ENV.fetch('MOCK_SERVER_URL')}/fixtures/encoding_unicode_url"
    result = Kreuzcrawl.scrape(engine, url)
    expect(result.html).not_to be_empty
  end
end

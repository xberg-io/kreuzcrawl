# frozen_string_literal: true

require 'kreuzcrawl'

RSpec.describe 'validation' do
  it 'validation_invalid_exclude_regex: Invalid regex in exclude_paths is rejected' do
    engine = Kreuzcrawl.create_engine(nil)
    url = "#{ENV.fetch('MOCK_SERVER_URL')}/fixtures/validation_invalid_exclude_regex"
    expect { Kreuzcrawl.scrape(engine, url) }.to raise_error
  end

  it 'validation_invalid_include_regex: Invalid regex in include_paths is rejected' do
    engine = Kreuzcrawl.create_engine(nil)
    url = "#{ENV.fetch('MOCK_SERVER_URL')}/fixtures/validation_invalid_include_regex"
    expect { Kreuzcrawl.scrape(engine, url) }.to raise_error
  end

  it 'validation_invalid_retry_code: Retry code outside 100-599 is rejected' do
    engine = Kreuzcrawl.create_engine(nil)
    url = "#{ENV.fetch('MOCK_SERVER_URL')}/fixtures/validation_invalid_retry_code"
    expect { Kreuzcrawl.scrape(engine, url) }.to raise_error
  end

  it 'validation_max_pages_zero: max_pages=0 is rejected as invalid config' do
    engine = Kreuzcrawl.create_engine(nil)
    url = "#{ENV.fetch('MOCK_SERVER_URL')}/fixtures/validation_max_pages_zero"
    expect { Kreuzcrawl.scrape(engine, url) }.to raise_error
  end

  it 'validation_max_redirects_too_high: max_redirects > 100 is rejected as invalid config' do
    engine = Kreuzcrawl.create_engine(nil)
    url = "#{ENV.fetch('MOCK_SERVER_URL')}/fixtures/validation_max_redirects_too_high"
    expect { Kreuzcrawl.scrape(engine, url) }.to raise_error
  end

  it 'validation_timeout_zero: Zero request timeout is rejected as invalid config' do
    engine = Kreuzcrawl.create_engine(nil)
    url = "#{ENV.fetch('MOCK_SERVER_URL')}/fixtures/validation_timeout_zero"
    expect { Kreuzcrawl.scrape(engine, url) }.to raise_error
  end
end

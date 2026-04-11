# frozen_string_literal: true

require 'kreuzcrawl'

RSpec.describe 'cache' do
  it 'cache_basic: Crawling with disk cache enabled succeeds without errors' do
    engine = Kreuzcrawl.create_engine(nil)
    url = "#{ENV.fetch('MOCK_SERVER_URL')}/fixtures/cache_basic"
    result = Kreuzcrawl.scrape(engine, url)
    expect(result.status_code).to eq(200)
  end
end

# frozen_string_literal: true

require 'kreuzcrawl'

RSpec.describe 'middleware' do
  it 'middleware_noop_no_effect: Default middleware chain does not affect normal scraping' do
    engine = Kreuzcrawl.create_engine(nil)
    url = "#{ENV.fetch('MOCK_SERVER_URL')}/fixtures/middleware_noop_no_effect"
    result = Kreuzcrawl.scrape(engine, url)
    expect(result.status_code).to eq(200)
    expect(result.metadata.title).to eq('Middleware Test')
  end
end

# frozen_string_literal: true

require 'kreuzcrawl'

RSpec.describe 'stealth' do
  it 'stealth_ua_rotation_config: User-agent rotation config is accepted and crawl succeeds' do
    engine = Kreuzcrawl.create_engine(nil)
    url = "#{ENV.fetch('MOCK_SERVER_URL')}/fixtures/stealth_ua_rotation_config"
    result = Kreuzcrawl.scrape(engine, url)
    expect(result.status_code).to eq(200)
  end
end

# frozen_string_literal: true

require 'kreuzcrawl'

RSpec.describe 'auth' do
  it 'auth_basic_http: Sends HTTP Basic authentication header' do
    engine = Kreuzcrawl.create_engine(nil)
    url = "#{ENV.fetch('MOCK_SERVER_URL')}/fixtures/auth_basic_http"
    result = Kreuzcrawl.scrape(engine, url)
    expect(result.auth_header_sent).to be(true)
    expect(result.status_code).to eq(200)
  end

  it 'auth_bearer_token: Sends Bearer token in Authorization header' do
    engine = Kreuzcrawl.create_engine(nil)
    url = "#{ENV.fetch('MOCK_SERVER_URL')}/fixtures/auth_bearer_token"
    result = Kreuzcrawl.scrape(engine, url)
    expect(result.auth_header_sent).to be(true)
    expect(result.status_code).to eq(200)
  end

  it 'auth_custom_header: Sends authentication via custom header (X-API-Key)' do
    engine = Kreuzcrawl.create_engine(nil)
    url = "#{ENV.fetch('MOCK_SERVER_URL')}/fixtures/auth_custom_header"
    result = Kreuzcrawl.scrape(engine, url)
    expect(result.auth_header_sent).to be(true)
    expect(result.status_code).to eq(200)
  end
end

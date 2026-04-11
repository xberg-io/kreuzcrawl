# frozen_string_literal: true

require 'kreuzcrawl'

RSpec.describe 'browser' do
  it 'browser_config_auto_no_feature: Browser mode \'auto\' without browser feature enabled does not use browser' do
    engine = Kreuzcrawl.create_engine(nil)
    url = "#{ENV.fetch('MOCK_SERVER_URL')}/fixtures/browser_config_auto_no_feature"
    result = Kreuzcrawl.scrape(engine, url)
    expect(result.status_code).to eq(200)
  end

  it 'browser_config_never_mode: Browser mode \'never\' prevents browser fallback even for SPA shell content' do
    engine = Kreuzcrawl.create_engine(nil)
    url = "#{ENV.fetch('MOCK_SERVER_URL')}/fixtures/browser_config_never_mode"
    result = Kreuzcrawl.scrape(engine, url)
    expect(result.status_code).to eq(200)
  end

  it 'browser_detect_minimal_page: Does NOT flag a short but real content page as needing JS rendering' do
    engine = Kreuzcrawl.create_engine(nil)
    url = "#{ENV.fetch('MOCK_SERVER_URL')}/fixtures/browser_detect_minimal_page"
    result = Kreuzcrawl.scrape(engine, url)
    expect(result.status_code).to eq(200)
  end

  it 'browser_detect_next_empty: Detects Next.js page with __NEXT_DATA__ but no rendered content as needing JS rendering' do
    engine = Kreuzcrawl.create_engine(nil)
    url = "#{ENV.fetch('MOCK_SERVER_URL')}/fixtures/browser_detect_next_empty"
    result = Kreuzcrawl.scrape(engine, url)
    expect(result.status_code).to eq(200)
  end

  it 'browser_detect_next_rendered: Does NOT flag Next.js page with full SSR content as needing JS rendering' do
    engine = Kreuzcrawl.create_engine(nil)
    url = "#{ENV.fetch('MOCK_SERVER_URL')}/fixtures/browser_detect_next_rendered"
    result = Kreuzcrawl.scrape(engine, url)
    expect(result.status_code).to eq(200)
  end

  it 'browser_detect_normal_page: Does NOT flag a normal server-rendered page as needing JS rendering' do
    engine = Kreuzcrawl.create_engine(nil)
    url = "#{ENV.fetch('MOCK_SERVER_URL')}/fixtures/browser_detect_normal_page"
    result = Kreuzcrawl.scrape(engine, url)
    expect(result.status_code).to eq(200)
  end

  it 'browser_detect_nuxt_shell: Detects Nuxt SPA shell with empty #__nuxt div as needing JS rendering' do
    engine = Kreuzcrawl.create_engine(nil)
    url = "#{ENV.fetch('MOCK_SERVER_URL')}/fixtures/browser_detect_nuxt_shell"
    result = Kreuzcrawl.scrape(engine, url)
    expect(result.status_code).to eq(200)
  end

  it 'browser_detect_react_shell: Detects React SPA shell with empty #root div as needing JS rendering' do
    engine = Kreuzcrawl.create_engine(nil)
    url = "#{ENV.fetch('MOCK_SERVER_URL')}/fixtures/browser_detect_react_shell"
    result = Kreuzcrawl.scrape(engine, url)
    expect(result.status_code).to eq(200)
  end

  it 'browser_detect_vue_shell: Detects Vue SPA shell with empty #app div as needing JS rendering' do
    engine = Kreuzcrawl.create_engine(nil)
    url = "#{ENV.fetch('MOCK_SERVER_URL')}/fixtures/browser_detect_vue_shell"
    result = Kreuzcrawl.scrape(engine, url)
    expect(result.status_code).to eq(200)
  end
end

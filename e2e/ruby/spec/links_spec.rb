# frozen_string_literal: true

require 'kreuzcrawl'

RSpec.describe 'links' do
  it 'links_anchor_fragment: Identifies fragment-only links as anchor type' do
    engine = Kreuzcrawl.create_engine(nil)
    url = "#{ENV.fetch('MOCK_SERVER_URL')}/fixtures/links_anchor_fragment"
    result = Kreuzcrawl.scrape(engine, url)
    expect(result.links[0].link_type).to include('anchor')
  end

  it 'links_base_tag: Resolves relative URLs using base tag href' do
    engine = Kreuzcrawl.create_engine(nil)
    url = "#{ENV.fetch('MOCK_SERVER_URL')}/fixtures/links_base_tag"
    result = Kreuzcrawl.scrape(engine, url)
    expect(result.links.length).to be > 2
    expect(result.links[0].url).to include('example.com')
  end

  it 'links_document_types: Detects PDF, DOCX, XLSX links as document type' do
    engine = Kreuzcrawl.create_engine(nil)
    url = "#{ENV.fetch('MOCK_SERVER_URL')}/fixtures/links_document_types"
    result = Kreuzcrawl.scrape(engine, url)
    expect(result.links[0].link_type).to include('document')
  end

  it 'links_empty_href: Handles empty href attributes without errors' do
    engine = Kreuzcrawl.create_engine(nil)
    url = "#{ENV.fetch('MOCK_SERVER_URL')}/fixtures/links_empty_href"
    result = Kreuzcrawl.scrape(engine, url)
    expect(result.links.length).to be > 0
    expect(result.links[0].url).to include('/valid')
  end

  it 'links_internal_external_classification: Correctly classifies internal vs external links by domain' do
    engine = Kreuzcrawl.create_engine(nil)
    url = "#{ENV.fetch('MOCK_SERVER_URL')}/fixtures/links_internal_external_classification"
    result = Kreuzcrawl.scrape(engine, url)
    expect(result.links.length).to be > 4
    expect(result.links[0].link_type).to include('internal')
    expect(result.links[0].link_type).to include('external')
  end

  it 'links_mailto_javascript_skip: Skips mailto:, javascript:, and tel: scheme links' do
    engine = Kreuzcrawl.create_engine(nil)
    url = "#{ENV.fetch('MOCK_SERVER_URL')}/fixtures/links_mailto_javascript_skip"
    result = Kreuzcrawl.scrape(engine, url)
    expect(result.links.length).to be > 0
    expect(result.links[0].url).not_to include('mailto:')
  end

  it 'links_protocol_relative: Handles protocol-relative URLs (//example.com) correctly' do
    engine = Kreuzcrawl.create_engine(nil)
    url = "#{ENV.fetch('MOCK_SERVER_URL')}/fixtures/links_protocol_relative"
    result = Kreuzcrawl.scrape(engine, url)
    expect(result.links.length).to be > 1
    expect(result.links[0].url).to include('//')
  end

  it 'links_rel_attributes: Preserves rel=nofollow and rel=canonical attributes' do
    engine = Kreuzcrawl.create_engine(nil)
    url = "#{ENV.fetch('MOCK_SERVER_URL')}/fixtures/links_rel_attributes"
    result = Kreuzcrawl.scrape(engine, url)
    expect(result.links.length).to be > 0
  end

  it 'links_relative_parent: Resolves ../ and ./ relative parent path links correctly' do
    engine = Kreuzcrawl.create_engine(nil)
    url = "#{ENV.fetch('MOCK_SERVER_URL')}/fixtures/links_relative_parent"
    result = Kreuzcrawl.scrape(engine, url)
    expect(result.links.length).to be > 3
  end
end

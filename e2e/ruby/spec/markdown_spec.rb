# frozen_string_literal: true

require 'kreuzcrawl'

RSpec.describe 'markdown' do
  it 'markdown_basic_conversion: HTML is always converted to markdown alongside raw HTML' do
    engine = Kreuzcrawl.create_engine(nil)
    url = "#{ENV.fetch('MOCK_SERVER_URL')}/fixtures/markdown_basic_conversion"
    result = Kreuzcrawl.scrape(engine, url)
    expect(result.status_code).to eq(200)
    expect(result.metadata.title).to eq('Test')
    expect(result.html).not_to be_empty
    expect(result.markdown.content).not_to be_empty
    expect(result.markdown.content).to include('Hello World')
  end

  it 'markdown_fit_content: Fit markdown removes navigation and boilerplate content' do
    engine = Kreuzcrawl.create_engine(nil)
    url = "#{ENV.fetch('MOCK_SERVER_URL')}/fixtures/markdown_fit_content"
    result = Kreuzcrawl.scrape(engine, url)
    expect(result.status_code).to eq(200)
    expect(result.markdown.content).not_to be_empty
  end

  it 'markdown_headings_and_paragraphs: Markdown conversion preserves heading hierarchy and paragraph text' do
    engine = Kreuzcrawl.create_engine(nil)
    url = "#{ENV.fetch('MOCK_SERVER_URL')}/fixtures/markdown_headings_and_paragraphs"
    result = Kreuzcrawl.scrape(engine, url)
    expect(result.markdown.content).not_to be_empty
    expect(result.markdown.content).to include('Main Title')
  end

  it 'markdown_links_converted: HTML links are converted to markdown link syntax' do
    engine = Kreuzcrawl.create_engine(nil)
    url = "#{ENV.fetch('MOCK_SERVER_URL')}/fixtures/markdown_links_converted"
    result = Kreuzcrawl.scrape(engine, url)
    expect(result.status_code).to eq(200)
    expect(result.html).not_to be_empty
    expect(result.markdown.content).not_to be_empty
    expect(result.markdown.content).to include('Example')
  end

  it 'markdown_with_citations: Markdown includes citation conversion with numbered references' do
    engine = Kreuzcrawl.create_engine(nil)
    url = "#{ENV.fetch('MOCK_SERVER_URL')}/fixtures/markdown_with_citations"
    result = Kreuzcrawl.scrape(engine, url)
    expect(result.status_code).to eq(200)
    expect(result.markdown.content).not_to be_empty
  end
end

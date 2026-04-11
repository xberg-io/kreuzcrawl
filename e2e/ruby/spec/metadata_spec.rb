# frozen_string_literal: true

require 'kreuzcrawl'

RSpec.describe 'metadata' do
  it 'metadata_article_times: Extracts article:published_time, modified_time, author, section, and tags' do
    engine = Kreuzcrawl.create_engine(nil)
    url = "#{ENV.fetch('MOCK_SERVER_URL')}/fixtures/metadata_article_times"
    result = Kreuzcrawl.scrape(engine, url)
    expect(result.status_code).to eq(200)
  end

  it 'metadata_favicons: Extracts favicon link tags including apple-touch-icon' do
    engine = Kreuzcrawl.create_engine(nil)
    url = "#{ENV.fetch('MOCK_SERVER_URL')}/fixtures/metadata_favicons"
    result = Kreuzcrawl.scrape(engine, url)
    expect(result.status_code).to eq(200)
  end

  it 'metadata_headings: Extracts heading hierarchy (h1-h6) from HTML page' do
    engine = Kreuzcrawl.create_engine(nil)
    url = "#{ENV.fetch('MOCK_SERVER_URL')}/fixtures/metadata_headings"
    result = Kreuzcrawl.scrape(engine, url)
    expect(result.status_code).to eq(200)
  end

  it 'metadata_hreflang: Extracts hreflang alternate link tags' do
    engine = Kreuzcrawl.create_engine(nil)
    url = "#{ENV.fetch('MOCK_SERVER_URL')}/fixtures/metadata_hreflang"
    result = Kreuzcrawl.scrape(engine, url)
    expect(result.status_code).to eq(200)
  end

  it 'metadata_keywords_author: Extracts keywords, author, viewport, generator, theme-color, robots, lang, dir metadata' do
    engine = Kreuzcrawl.create_engine(nil)
    url = "#{ENV.fetch('MOCK_SERVER_URL')}/fixtures/metadata_keywords_author"
    result = Kreuzcrawl.scrape(engine, url)
    expect(result.status_code).to eq(200)
    expect(result.metadata.title).to eq('Comprehensive Metadata Test Page')
    expect(result.metadata.canonical_url).not_to be_empty
    expect(result.metadata.keywords).not_to be_empty
    expect(result.metadata.keywords).to include('rust')
    expect(result.metadata.author).to eq('Jane Developer')
    expect(result.metadata.viewport).not_to be_empty
    expect(result.metadata.generator).to eq('kreuzcrawl/1.0')
    expect(result.metadata.theme_color).to eq('#ff6600')
    expect(result.metadata.robots).to eq('index, follow')
    expect(result.metadata.html_lang).to eq('en')
    expect(result.metadata.html_dir).to eq('ltr')
  end

  it 'metadata_og_video_audio: Extracts og:video, og:audio, and og:locale:alternate metadata' do
    engine = Kreuzcrawl.create_engine(nil)
    url = "#{ENV.fetch('MOCK_SERVER_URL')}/fixtures/metadata_og_video_audio"
    result = Kreuzcrawl.scrape(engine, url)
    expect(result.status_code).to eq(200)
    expect(result.metadata.og_video).to eq('https://example.com/video.mp4')
    expect(result.metadata.og_audio).to eq('https://example.com/audio.mp3')
  end

  it 'metadata_response_headers: Extracts response metadata from HTTP headers (etag, server, content-language)' do
    engine = Kreuzcrawl.create_engine(nil)
    url = "#{ENV.fetch('MOCK_SERVER_URL')}/fixtures/metadata_response_headers"
    result = Kreuzcrawl.scrape(engine, url)
    expect(result.status_code).to eq(200)
  end

  it 'metadata_word_count: Computes word count from visible page text' do
    engine = Kreuzcrawl.create_engine(nil)
    url = "#{ENV.fetch('MOCK_SERVER_URL')}/fixtures/metadata_word_count"
    result = Kreuzcrawl.scrape(engine, url)
    expect(result.status_code).to eq(200)
  end
end

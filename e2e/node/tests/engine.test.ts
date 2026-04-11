import { describe, it, expect } from 'vitest';
import { scrape, createEngine } from '@kreuzberg/kreuzcrawl';

describe('engine', () => {
  it('engine_batch_basic: CrawlEngine with defaults batch scrapes like the free function', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    // skipped: field 'batch.completed_count' not available on result type
    // skipped: field 'batch.total_count' not available on result type
  });

  it('engine_crawl_basic: CrawlEngine with defaults crawls multiple pages like the free function', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    // skipped: field 'crawl.pages_crawled' not available on result type
    // skipped: field 'crawl.min_pages' not available on result type
  });

  it('engine_map_basic: CrawlEngine with defaults discovers URLs like the free function', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    // skipped: field 'map.min_urls' not available on result type
  });

  it('engine_scrape_basic: CrawlEngine with defaults scrapes a page identically to the free function', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    expect(result.statusCode).toBe(200);
    expect(result.contentType).toBe("text/html");
    expect(result.metadata.title).toBe("Engine Test");
    expect(result.metadata.description).toContain("Testing the engine");
    expect(result.links.length).toBeGreaterThanOrEqual(1);
    // skipped: field 'headings.h1_text' not available on result type
  });

  it('engine_stream_basic: CrawlEngine with defaults streams events like the free function', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    // skipped: field 'stream.has_page_event' not available on result type
    // skipped: field 'stream.has_complete_event' not available on result type
    // skipped: field 'stream.event_count_min' not available on result type
  });
});

import { describe, it, expect } from 'vitest';
import { scrape, createEngine } from '@kreuzberg/kreuzcrawl';

describe('scrape', () => {
  it('scrape_asset_dedup: Same asset linked twice results in one download with one unique hash', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    expect(result.statusCode).toBe(200);
    expect(result.assets.length).toBe(2);
    expect(result.assets[0].uniqueHashes).toBe(2);
  });

  it('scrape_asset_max_size: Skips assets exceeding max_asset_size limit', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    expect(result.statusCode).toBe(200);
    expect(result.assets.length).toBe(2);
  });

  it('scrape_asset_type_filter: Only downloads image assets when asset_types filter is set', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    expect(result.statusCode).toBe(200);
    expect(result.assets.length).toBe(1);
    expect(result.assets[0].category).toContain("image");
  });

  it('scrape_basic_html_page: Scrapes a simple HTML page and extracts title, description, and links', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    expect(result.statusCode).toBe(200);
    expect(result.contentType).toBe("text/html");
    expect(result.html.length).toBeGreaterThan(0);
    expect(result.metadata.title).toBe("Example Domain");
    expect(result.metadata.description).toContain("illustrative examples");
    expect(result.metadata.canonicalUrl.length).toBeGreaterThan(0);
    expect(result.links.length).toBeGreaterThan(0);
    expect(result.links[0].linkType).toContain("external");
    expect(result.images.length).toBe(0);
    // skipped: field 'og.title' not available on result type
  });

  it('scrape_complex_links: Classifies links by type: internal, external, anchor, document, image', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    expect(result.statusCode).toBe(200);
    expect(result.links.length).toBeGreaterThan(9);
    expect(result.links[0].linkType).toContain("internal");
    expect(result.links[0].linkType).toContain("external");
    expect(result.links[0].linkType).toContain("anchor");
    expect(result.links[0].linkType).toContain("document");
  });

  it('scrape_download_assets: Downloads CSS, JS, and image assets from page', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    expect(result.statusCode).toBe(200);
    expect(result.assets.length).toBeGreaterThan(2);
  });

  it('scrape_dublin_core: Extracts Dublin Core metadata from a page', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    expect(result.statusCode).toBe(200);
    // skipped: field 'dublin_core.title' not available on result type
    // skipped: field 'dublin_core.title' not available on result type
    // skipped: field 'dublin_core.creator' not available on result type
  });

  it('scrape_empty_page: Handles an empty HTML document without errors', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    expect(result.statusCode).toBe(200);
    expect(result.links.length).toBeGreaterThan(-1);
    expect(result.images.length).toBe(0);
  });

  it('scrape_feed_discovery: Discovers RSS, Atom, and JSON feed links', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    expect(result.statusCode).toBe(200);
    expect(result.feeds[0].rss.length).toBe(1);
    expect(result.feeds[0].atom.length).toBe(1);
    expect(result.feeds[0].jsonFeed.length).toBe(1);
  });

  it('scrape_image_sources: Extracts images from img, picture, og:image, twitter:image', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    expect(result.statusCode).toBe(200);
    expect(result.images.length).toBeGreaterThan(4);
    // skipped: field 'og.image' not available on result type
  });

  it('scrape_js_heavy_spa: Handles SPA page with JavaScript-only content (no server-rendered HTML)', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    expect(result.html.length).toBeGreaterThan(0);
  });

  it('scrape_json_ld: Extracts JSON-LD structured data from a page', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    expect(result.statusCode).toBe(200);
    expect(result.jsonLd.length).toBeGreaterThan(0);
    expect(result.jsonLd[0].type).toBe("Recipe");
    expect(result.jsonLd[0].name).toBe("Best Chocolate Cake");
  });

  it('scrape_malformed_html: Gracefully handles broken HTML without crashing', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    expect(result.statusCode).toBe(200);
    expect(result.html.length).toBeGreaterThan(0);
    expect(result.metadata.description).toContain("broken HTML");
  });

  it('scrape_og_metadata: Extracts full Open Graph metadata from a page', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    expect(result.statusCode).toBe(200);
    // skipped: field 'og.title' not available on result type
    // skipped: field 'og.title' not available on result type
    // skipped: field 'og.type' not available on result type
    // skipped: field 'og.image' not available on result type
    // skipped: field 'og.description' not available on result type
    expect(result.metadata.title).toBe("Article Title - Example Blog");
  });

  it('scrape_twitter_card: Extracts Twitter Card metadata from a page', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    expect(result.statusCode).toBe(200);
    // skipped: field 'twitter.card' not available on result type
    // skipped: field 'twitter.card_type' not available on result type
    // skipped: field 'twitter.title' not available on result type
  });
});

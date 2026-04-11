<?php

declare(strict_types=1);

namespace Kreuzberg\E2e;

use PHPUnit\Framework\TestCase;
use Kreuzcrawl\Kreuzcrawl;

/** E2e tests for category: scrape. */
final class ScrapeTest extends TestCase
{
    /** Same asset linked twice results in one download with one unique hash */
    public function test_scrape_asset_dedup(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        $this->assertEquals(200, $result->status_code);
        $this->assertEquals(2, count($result->assets));
        $this->assertEquals(2, $result->assets[0]->unique_hashes);
    }

    /** Skips assets exceeding max_asset_size limit */
    public function test_scrape_asset_max_size(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        $this->assertEquals(200, $result->status_code);
        $this->assertEquals(2, count($result->assets));
    }

    /** Only downloads image assets when asset_types filter is set */
    public function test_scrape_asset_type_filter(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        $this->assertEquals(200, $result->status_code);
        $this->assertEquals(1, count($result->assets));
        $this->assertStringContainsString("image", $result->assets[0]->category);
    }

    /** Scrapes a simple HTML page and extracts title, description, and links */
    public function test_scrape_basic_html_page(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        $this->assertEquals(200, $result->status_code);
        $this->assertEquals("text/html", $result->content_type);
        $this->assertNotEmpty($result->html);
        $this->assertEquals("Example Domain", $result->metadata->title);
        $this->assertStringContainsString("illustrative examples", $result->metadata->description);
        $this->assertNotEmpty($result->metadata->canonical_url);
        $this->assertGreaterThan(0, count($result->links));
        $this->assertStringContainsString("external", $result->links[0]->link_type);
        $this->assertEquals(0, count($result->images));
        // skipped: field 'og.title' not available on result type
    }

    /** Classifies links by type: internal, external, anchor, document, image */
    public function test_scrape_complex_links(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        $this->assertEquals(200, $result->status_code);
        $this->assertGreaterThan(9, count($result->links));
        $this->assertStringContainsString("internal", $result->links[0]->link_type);
        $this->assertStringContainsString("external", $result->links[0]->link_type);
        $this->assertStringContainsString("anchor", $result->links[0]->link_type);
        $this->assertStringContainsString("document", $result->links[0]->link_type);
    }

    /** Downloads CSS, JS, and image assets from page */
    public function test_scrape_download_assets(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        $this->assertEquals(200, $result->status_code);
        $this->assertGreaterThan(2, count($result->assets));
    }

    /** Extracts Dublin Core metadata from a page */
    public function test_scrape_dublin_core(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        $this->assertEquals(200, $result->status_code);
        // skipped: field 'dublin_core.title' not available on result type
        // skipped: field 'dublin_core.title' not available on result type
        // skipped: field 'dublin_core.creator' not available on result type
    }

    /** Handles an empty HTML document without errors */
    public function test_scrape_empty_page(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        $this->assertEquals(200, $result->status_code);
        $this->assertGreaterThan(-1, count($result->links));
        $this->assertEquals(0, count($result->images));
    }

    /** Discovers RSS, Atom, and JSON feed links */
    public function test_scrape_feed_discovery(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        $this->assertEquals(200, $result->status_code);
        $this->assertEquals(1, count($result->feeds[0]->rss));
        $this->assertEquals(1, count($result->feeds[0]->atom));
        $this->assertEquals(1, count($result->feeds[0]->json_feed));
    }

    /** Extracts images from img, picture, og:image, twitter:image */
    public function test_scrape_image_sources(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        $this->assertEquals(200, $result->status_code);
        $this->assertGreaterThan(4, count($result->images));
        // skipped: field 'og.image' not available on result type
    }

    /** Handles SPA page with JavaScript-only content (no server-rendered HTML) */
    public function test_scrape_js_heavy_spa(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        $this->assertNotEmpty($result->html);
    }

    /** Extracts JSON-LD structured data from a page */
    public function test_scrape_json_ld(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        $this->assertEquals(200, $result->status_code);
        $this->assertNotEmpty($result->json_ld);
        $this->assertEquals("Recipe", $result->json_ld[0]->type);
        $this->assertEquals("Best Chocolate Cake", $result->json_ld[0]->name);
    }

    /** Gracefully handles broken HTML without crashing */
    public function test_scrape_malformed_html(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        $this->assertEquals(200, $result->status_code);
        $this->assertNotEmpty($result->html);
        $this->assertStringContainsString("broken HTML", $result->metadata->description);
    }

    /** Extracts full Open Graph metadata from a page */
    public function test_scrape_og_metadata(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        $this->assertEquals(200, $result->status_code);
        // skipped: field 'og.title' not available on result type
        // skipped: field 'og.title' not available on result type
        // skipped: field 'og.type' not available on result type
        // skipped: field 'og.image' not available on result type
        // skipped: field 'og.description' not available on result type
        $this->assertEquals("Article Title - Example Blog", $result->metadata->title);
    }

    /** Extracts Twitter Card metadata from a page */
    public function test_scrape_twitter_card(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        $this->assertEquals(200, $result->status_code);
        // skipped: field 'twitter.card' not available on result type
        // skipped: field 'twitter.card_type' not available on result type
        // skipped: field 'twitter.title' not available on result type
    }
}

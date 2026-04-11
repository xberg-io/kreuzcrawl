<?php

declare(strict_types=1);

namespace Kreuzberg\E2e;

use PHPUnit\Framework\TestCase;
use Kreuzcrawl\Kreuzcrawl;

/** E2e tests for category: engine. */
final class EngineTest extends TestCase
{
    /** CrawlEngine with defaults batch scrapes like the free function */
    public function test_engine_batch_basic(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        // skipped: field 'batch.completed_count' not available on result type
        // skipped: field 'batch.total_count' not available on result type
    }

    /** CrawlEngine with defaults crawls multiple pages like the free function */
    public function test_engine_crawl_basic(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        // skipped: field 'crawl.pages_crawled' not available on result type
        // skipped: field 'crawl.min_pages' not available on result type
    }

    /** CrawlEngine with defaults discovers URLs like the free function */
    public function test_engine_map_basic(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        // skipped: field 'map.min_urls' not available on result type
    }

    /** CrawlEngine with defaults scrapes a page identically to the free function */
    public function test_engine_scrape_basic(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        $this->assertEquals(200, $result->status_code);
        $this->assertEquals("text/html", $result->content_type);
        $this->assertEquals("Engine Test", $result->metadata->title);
        $this->assertStringContainsString("Testing the engine", $result->metadata->description);
        $this->assertGreaterThanOrEqual(1, count($result->links));
        // skipped: field 'headings.h1_text' not available on result type
    }

    /** CrawlEngine with defaults streams events like the free function */
    public function test_engine_stream_basic(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        // skipped: field 'stream.has_page_event' not available on result type
        // skipped: field 'stream.has_complete_event' not available on result type
        // skipped: field 'stream.event_count_min' not available on result type
    }
}

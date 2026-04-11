<?php

declare(strict_types=1);

namespace Kreuzberg\E2e;

use PHPUnit\Framework\TestCase;
use Kreuzcrawl\Kreuzcrawl;

/** E2e tests for category: links. */
final class LinksTest extends TestCase
{
    /** Identifies fragment-only links as anchor type */
    public function test_links_anchor_fragment(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        $this->assertStringContainsString("anchor", $result->links[0]->link_type);
    }

    /** Resolves relative URLs using base tag href */
    public function test_links_base_tag(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        $this->assertGreaterThan(2, count($result->links));
        $this->assertStringContainsString("example.com", $result->links[0]->url);
    }

    /** Detects PDF, DOCX, XLSX links as document type */
    public function test_links_document_types(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        $this->assertStringContainsString("document", $result->links[0]->link_type);
    }

    /** Handles empty href attributes without errors */
    public function test_links_empty_href(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        $this->assertGreaterThan(0, count($result->links));
        $this->assertStringContainsString("/valid", $result->links[0]->url);
    }

    /** Correctly classifies internal vs external links by domain */
    public function test_links_internal_external_classification(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        $this->assertGreaterThan(4, count($result->links));
        $this->assertStringContainsString("internal", $result->links[0]->link_type);
        $this->assertStringContainsString("external", $result->links[0]->link_type);
    }

    /** Skips mailto:, javascript:, and tel: scheme links */
    public function test_links_mailto_javascript_skip(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        $this->assertGreaterThan(0, count($result->links));
        $this->assertStringNotContainsString("mailto:", $result->links[0]->url);
    }

    /** Handles protocol-relative URLs (//example.com) correctly */
    public function test_links_protocol_relative(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        $this->assertGreaterThan(1, count($result->links));
        $this->assertStringContainsString("//", $result->links[0]->url);
    }

    /** Preserves rel=nofollow and rel=canonical attributes */
    public function test_links_rel_attributes(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        $this->assertGreaterThan(0, count($result->links));
    }

    /** Resolves ../ and ./ relative parent path links correctly */
    public function test_links_relative_parent(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        $this->assertGreaterThan(3, count($result->links));
    }
}

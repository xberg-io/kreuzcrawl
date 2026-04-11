package dev.kreuzberg.e2e;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;
import dev.kreuzberg.kreuzcrawl.Kreuzcrawl;

/** E2e tests for category: markdown. */
class MarkdownTest {
    @Test
    void testMarkdownBasicConversion() throws Exception {
        // HTML is always converted to markdown alongside raw HTML
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertEquals(200, result.statusCode());
        assertEquals("Test", result.metadata().title().orElse(""));
        assertFalse(result.html().isEmpty(), "expected non-empty value");
        assertFalse(result.markdown().orElseThrow().content().isEmpty(), "expected non-empty value");
        assertTrue(result.markdown().orElseThrow().content().contains("Hello World"), "expected to contain: " + "Hello World");
    }

    @Test
    void testMarkdownCrawlAllPages() throws Exception {
        // All crawled pages have markdown field populated
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        // skipped: field 'crawl.pages_crawled' not available on result type
    }

    @Test
    void testMarkdownFitContent() throws Exception {
        // Fit markdown removes navigation and boilerplate content
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertEquals(200, result.statusCode());
        assertFalse(result.markdown().orElseThrow().content().isEmpty(), "expected non-empty value");
    }

    @Test
    void testMarkdownHeadingsAndParagraphs() throws Exception {
        // Markdown conversion preserves heading hierarchy and paragraph text
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertFalse(result.markdown().orElseThrow().content().isEmpty(), "expected non-empty value");
        assertTrue(result.markdown().orElseThrow().content().contains("Main Title"), "expected to contain: " + "Main Title");
    }

    @Test
    void testMarkdownLinksConverted() throws Exception {
        // HTML links are converted to markdown link syntax
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertEquals(200, result.statusCode());
        assertFalse(result.html().isEmpty(), "expected non-empty value");
        assertFalse(result.markdown().orElseThrow().content().isEmpty(), "expected non-empty value");
        assertTrue(result.markdown().orElseThrow().content().contains("Example"), "expected to contain: " + "Example");
    }

    @Test
    void testMarkdownWithCitations() throws Exception {
        // Markdown includes citation conversion with numbered references
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertEquals(200, result.statusCode());
        assertFalse(result.markdown().orElseThrow().content().isEmpty(), "expected non-empty value");
    }

}

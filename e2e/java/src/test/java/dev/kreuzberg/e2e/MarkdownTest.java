package dev.kreuzberg.e2e;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;

/** E2e tests for category: markdown. */
class MarkdownTest {
    @Test
    void testMarkdownBasicConversion() throws Exception {
        // HTML is always converted to markdown alongside raw HTML
        var result = Kreuzcrawl.scrape();
        assertEquals(200, result.status_code());
        assertEquals("Test", result.metadata().title().orElse(""));
        assertFalse(result.html().isEmpty(), "expected non-empty value");
        assertFalse(result.markdown().orElse("").isEmpty(), "expected non-empty value");
        assertTrue(result.markdown().orElse("").contains("Hello World"), "expected to contain: " + "Hello World");
    }

    @Test
    void testMarkdownCrawlAllPages() throws Exception {
        // All crawled pages have markdown field populated
        var result = Kreuzcrawl.scrape();
        assertEquals(2, result.crawl().pages_crawled());
    }

    @Test
    void testMarkdownFitContent() throws Exception {
        // Fit markdown removes navigation and boilerplate content
        var result = Kreuzcrawl.scrape();
        assertEquals(200, result.status_code());
        assertFalse(result.markdown().orElse("").isEmpty(), "expected non-empty value");
    }

    @Test
    void testMarkdownHeadingsAndParagraphs() throws Exception {
        // Markdown conversion preserves heading hierarchy and paragraph text
        var result = Kreuzcrawl.scrape();
        assertFalse(result.markdown().orElse("").isEmpty(), "expected non-empty value");
        assertTrue(result.markdown().orElse("").contains("Main Title"), "expected to contain: " + "Main Title");
    }

    @Test
    void testMarkdownLinksConverted() throws Exception {
        // HTML links are converted to markdown link syntax
        var result = Kreuzcrawl.scrape();
        assertEquals(200, result.status_code());
        assertFalse(result.html().isEmpty(), "expected non-empty value");
        assertFalse(result.markdown().orElse("").isEmpty(), "expected non-empty value");
        assertTrue(result.markdown().orElse("").contains("Example"), "expected to contain: " + "Example");
    }

    @Test
    void testMarkdownWithCitations() throws Exception {
        // Markdown includes citation conversion with numbered references
        var result = Kreuzcrawl.scrape();
        assertEquals(200, result.status_code());
        assertFalse(result.markdown().orElse("").isEmpty(), "expected non-empty value");
    }

}

package dev.kreuzberg.e2e;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;
import dev.kreuzberg.kreuzcrawl.Kreuzcrawl;

/** E2e tests for category: content. */
class ContentTest {
    @Test
    void testContent204NoContent() throws Exception {
        // Handles 204 No Content response gracefully
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertEquals(204, result.statusCode());
        assertTrue(result.html().isEmpty(), "expected empty value");
    }

    @Test
    void testContentCharsetIso8859() throws Exception {
        // Handles ISO-8859-1 encoded page correctly
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertEquals("iso-8859-1", result.detectedCharset().orElse(""));
    }

    @Test
    void testContentEmptyBody() throws Exception {
        // Handles 200 response with empty body gracefully
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertEquals(200, result.statusCode());
    }

    @Test
    void testContentGzipCompressed() throws Exception {
        // Handles response with Accept-Encoding gzip negotiation
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertFalse(result.html().isEmpty(), "expected non-empty value");
        assertEquals(200, result.statusCode());
    }

    @Test
    void testContentLargePageLimit() throws Exception {
        // Respects max body size limit and truncates or skips oversized pages
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertTrue(result.bodySize() < 1025, "expected < 1025");
    }

    @Test
    void testContentMainOnly() throws Exception {
        // Extracts only main content area, excluding nav, sidebar, footer
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertEquals(true, result.mainContentOnly());
    }

    @Test
    void testContentPdfNoExtension() throws Exception {
        // Detects PDF content by Content-Type header when URL has no .pdf extension
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertEquals(true, result.isPdf());
    }

    @Test
    void testContentRemoveTags() throws Exception {
        // Removes specified HTML elements by CSS selector before processing
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertFalse(result.html().isEmpty(), "expected non-empty value");
    }

    @Test
    void testContentUtf8Bom() throws Exception {
        // Handles UTF-8 content with BOM marker correctly
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertEquals("utf-8", result.detectedCharset().orElse(""));
        assertFalse(result.html().isEmpty(), "expected non-empty value");
    }

}

package dev.kreuzberg.e2e;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;

/** E2e tests for category: links. */
class LinksTest {
    @Test
    void testLinksAnchorFragment() throws Exception {
        // Identifies fragment-only links as anchor type
        var result = Kreuzcrawl.scrape();
        assertTrue(result.links().get("").link_type().contains("anchor"), "expected to contain: " + "anchor");
    }

    @Test
    void testLinksBaseTag() throws Exception {
        // Resolves relative URLs using base tag href
        var result = Kreuzcrawl.scrape();
        assertTrue(result.links().size() > 2, "expected > 2");
        assertTrue(result.links().get("").url().contains("example.com"), "expected to contain: " + "example.com");
    }

    @Test
    void testLinksDocumentTypes() throws Exception {
        // Detects PDF, DOCX, XLSX links as document type
        var result = Kreuzcrawl.scrape();
        assertTrue(result.links().get("").link_type().contains("document"), "expected to contain: " + "document");
    }

    @Test
    void testLinksEmptyHref() throws Exception {
        // Handles empty href attributes without errors
        var result = Kreuzcrawl.scrape();
        assertTrue(result.links().size() > 0, "expected > 0");
        assertTrue(result.links().get("").url().contains("/valid"), "expected to contain: " + "/valid");
    }

    @Test
    void testLinksInternalExternalClassification() throws Exception {
        // Correctly classifies internal vs external links by domain
        var result = Kreuzcrawl.scrape();
        assertTrue(result.links().size() > 4, "expected > 4");
        assertTrue(result.links().get("").link_type().contains("internal"), "expected to contain: " + "internal");
        assertTrue(result.links().get("").link_type().contains("external"), "expected to contain: " + "external");
    }

    @Test
    void testLinksMailtoJavascriptSkip() throws Exception {
        // Skips mailto:, javascript:, and tel: scheme links
        var result = Kreuzcrawl.scrape();
        assertTrue(result.links().size() > 0, "expected > 0");
        assertFalse(result.links().get("").url().contains("mailto:"), "expected NOT to contain: " + "mailto:");
    }

    @Test
    void testLinksProtocolRelative() throws Exception {
        // Handles protocol-relative URLs (//example.com) correctly
        var result = Kreuzcrawl.scrape();
        assertTrue(result.links().size() > 1, "expected > 1");
        assertFalse(result.links().get("").protocol_relative().isEmpty(), "expected non-empty value");
    }

    @Test
    void testLinksRelAttributes() throws Exception {
        // Preserves rel=nofollow and rel=canonical attributes
        var result = Kreuzcrawl.scrape();
        assertTrue(result.links().size() > 0, "expected > 0");
    }

    @Test
    void testLinksRelativeParent() throws Exception {
        // Resolves ../ and ./ relative parent path links correctly
        var result = Kreuzcrawl.scrape();
        assertTrue(result.links().size() > 3, "expected > 3");
    }

}

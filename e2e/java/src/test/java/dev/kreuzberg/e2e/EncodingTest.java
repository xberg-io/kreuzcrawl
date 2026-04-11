package dev.kreuzberg.e2e;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;
import dev.kreuzberg.kreuzcrawl.Kreuzcrawl;

/** E2e tests for category: encoding. */
class EncodingTest {
    @Test
    void testEncodingDoubleEncoded() throws Exception {
        // Handles double-encoded URL characters (%25C3%25B6)
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertFalse(result.html().isEmpty(), "expected non-empty value");
        assertTrue(result.links().size() >= 1, "expected >= 1");
    }

    @Test
    void testEncodingMixedCharsetPage() throws Exception {
        // Handles charset mismatch between HTTP header and HTML meta tag
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertFalse(result.html().isEmpty(), "expected non-empty value");
    }

    @Test
    void testEncodingPercentEncodedPath() throws Exception {
        // Handles percent-encoded spaces and characters in URL paths
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertFalse(result.html().isEmpty(), "expected non-empty value");
        assertTrue(result.links().size() >= 2, "expected >= 2");
    }

    @Test
    void testEncodingUnicodeUrl() throws Exception {
        // Handles Unicode characters in URLs (Hebrew, Japanese, Cyrillic)
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertFalse(result.html().isEmpty(), "expected non-empty value");
    }

}

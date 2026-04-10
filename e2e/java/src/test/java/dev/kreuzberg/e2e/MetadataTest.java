package dev.kreuzberg.e2e;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;

/** E2e tests for category: metadata. */
class MetadataTest {
    @Test
    void testMetadataArticleTimes() throws Exception {
        // Extracts article:published_time, modified_time, author, section, and tags
        var result = Kreuzcrawl.scrape();
        assertEquals(200, result.status_code());
        assertEquals("2024-01-15T10:00:00Z", result.article().published_time());
        assertEquals("2024-06-20T14:30:00Z", result.article().modified_time());
        assertEquals("Jane Developer", result.article().author());
        assertEquals("Technology", result.article().section());
        assertEquals(3, result.article().tags().size());
    }

    @Test
    void testMetadataFavicons() throws Exception {
        // Extracts favicon link tags including apple-touch-icon
        var result = Kreuzcrawl.scrape();
        assertEquals(200, result.status_code());
        assertEquals(5, result.favicons().size());
        assertFalse(result.favicons().get("").apple_touch().isEmpty(), "expected non-empty value");
    }

    @Test
    void testMetadataHeadings() throws Exception {
        // Extracts heading hierarchy (h1-h6) from HTML page
        var result = Kreuzcrawl.scrape();
        assertEquals(200, result.status_code());
        assertEquals(1, result.headings().h1().size());
        assertEquals("Primary Heading", result.headings().h1().get("0").text());
        assertEquals(8, result.headings().size());
    }

    @Test
    void testMetadataHreflang() throws Exception {
        // Extracts hreflang alternate link tags
        var result = Kreuzcrawl.scrape();
        assertEquals(200, result.status_code());
        assertEquals(4, result.hreflang().size());
        assertTrue(result.hreflang().get("").lang().contains("en"), "expected to contain: " + "en");
    }

    @Test
    void testMetadataKeywordsAuthor() throws Exception {
        // Extracts keywords, author, viewport, generator, theme-color, robots, lang, dir metadata
        var result = Kreuzcrawl.scrape();
        assertEquals(200, result.status_code());
        assertEquals("Comprehensive Metadata Test Page", result.metadata().title().orElse(""));
        assertFalse(result.metadata().canonical_url().orElse("").isEmpty(), "expected non-empty value");
        assertFalse(result.metadata().keywords().isEmpty(), "expected non-empty value");
        assertTrue(result.metadata().keywords().contains("rust"), "expected to contain: " + "rust");
        assertEquals("Jane Developer", result.metadata().author());
        assertFalse(result.metadata().viewport().isEmpty(), "expected non-empty value");
        assertEquals("kreuzcrawl/1.0", result.metadata().generator());
        assertEquals("#ff6600", result.metadata().theme_color());
        assertEquals("index, follow", result.metadata().robots());
        assertEquals("en", result.metadata().lang());
        assertEquals("ltr", result.metadata().dir());
    }

    @Test
    void testMetadataOgVideoAudio() throws Exception {
        // Extracts og:video, og:audio, and og:locale:alternate metadata
        var result = Kreuzcrawl.scrape();
        assertEquals(200, result.status_code());
        assertEquals("https://example.com/video.mp4", result.og().video());
        assertEquals("https://example.com/audio.mp3", result.og().audio());
        assertEquals(2, result.og().locale_alternate().size());
    }

    @Test
    void testMetadataResponseHeaders() throws Exception {
        // Extracts response metadata from HTTP headers (etag, server, content-language)
        var result = Kreuzcrawl.scrape();
        assertEquals(200, result.status_code());
        assertFalse(result.response_headers().etag().isEmpty(), "expected non-empty value");
        assertFalse(result.response_headers().last_modified().isEmpty(), "expected non-empty value");
        assertTrue(result.response_headers().server().contains("nginx"), "expected to contain: " + "nginx");
        assertEquals("en-US", result.response_headers().content_language());
    }

    @Test
    void testMetadataWordCount() throws Exception {
        // Computes word count from visible page text
        var result = Kreuzcrawl.scrape();
        assertEquals(200, result.status_code());
        assertTrue(result.computed().word_count() > 99, "expected > 99");
        assertTrue(result.computed().word_count() < 301, "expected < 301");
    }

}

package dev.kreuzberg.e2e;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;
import dev.kreuzberg.kreuzcrawl.Kreuzcrawl;

/** E2e tests for category: metadata. */
class MetadataTest {
    @Test
    void testMetadataArticleTimes() throws Exception {
        // Extracts article:published_time, modified_time, author, section, and tags
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertEquals(200, result.statusCode());
        // skipped: field 'article.published_time' not available on result type
        // skipped: field 'article.modified_time' not available on result type
        // skipped: field 'article.author' not available on result type
        // skipped: field 'article.section' not available on result type
        // skipped: field 'article.tags.length' not available on result type
    }

    @Test
    void testMetadataFavicons() throws Exception {
        // Extracts favicon link tags including apple-touch-icon
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertEquals(200, result.statusCode());
        // skipped: field 'favicons.length' not available on result type
        // skipped: field 'favicons[].apple_touch' not available on result type
    }

    @Test
    void testMetadataHeadings() throws Exception {
        // Extracts heading hierarchy (h1-h6) from HTML page
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertEquals(200, result.statusCode());
        // skipped: field 'headings.h1.length' not available on result type
        // skipped: field 'headings.h1[0].text' not available on result type
        // skipped: field 'headings.length' not available on result type
    }

    @Test
    void testMetadataHreflang() throws Exception {
        // Extracts hreflang alternate link tags
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertEquals(200, result.statusCode());
        // skipped: field 'hreflang.length' not available on result type
        // skipped: field 'hreflang[].lang' not available on result type
    }

    @Test
    void testMetadataKeywordsAuthor() throws Exception {
        // Extracts keywords, author, viewport, generator, theme-color, robots, lang, dir metadata
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertEquals(200, result.statusCode());
        assertEquals("Comprehensive Metadata Test Page", result.metadata().title().orElse(""));
        assertFalse(result.metadata().canonicalUrl().orElse("").isEmpty(), "expected non-empty value");
        assertFalse(result.metadata().keywords().orElse("").isEmpty(), "expected non-empty value");
        assertTrue(result.metadata().keywords().orElse("").contains("rust"), "expected to contain: " + "rust");
        assertEquals("Jane Developer", result.metadata().author().orElse(""));
        assertFalse(result.metadata().viewport().orElse("").isEmpty(), "expected non-empty value");
        assertEquals("kreuzcrawl/1.0", result.metadata().generator().orElse(""));
        assertEquals("#ff6600", result.metadata().themeColor().orElse(""));
        assertEquals("index, follow", result.metadata().robots().orElse(""));
        assertEquals("en", result.metadata().htmlLang().orElse(""));
        assertEquals("ltr", result.metadata().htmlDir().orElse(""));
    }

    @Test
    void testMetadataOgVideoAudio() throws Exception {
        // Extracts og:video, og:audio, and og:locale:alternate metadata
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertEquals(200, result.statusCode());
        assertEquals("https://example.com/video.mp4", result.metadata().ogVideo().orElse(""));
        assertEquals("https://example.com/audio.mp3", result.metadata().ogAudio().orElse(""));
        // skipped: field 'og.locale_alternate.length' not available on result type
    }

    @Test
    void testMetadataResponseHeaders() throws Exception {
        // Extracts response metadata from HTTP headers (etag, server, content-language)
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertEquals(200, result.statusCode());
        // skipped: field 'response_headers.etag' not available on result type
        // skipped: field 'response_headers.last_modified' not available on result type
        // skipped: field 'response_headers.server' not available on result type
        // skipped: field 'response_headers.content_language' not available on result type
    }

    @Test
    void testMetadataWordCount() throws Exception {
        // Computes word count from visible page text
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertEquals(200, result.statusCode());
        // skipped: field 'computed.word_count' not available on result type
        // skipped: field 'computed.word_count' not available on result type
    }

}

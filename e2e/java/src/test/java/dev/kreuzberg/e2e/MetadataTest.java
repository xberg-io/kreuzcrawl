package dev.kreuzberg.e2e;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;

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
        assertEquals("Comprehensive Metadata Test Page", result.metadata().orElseThrow().title().orElse(""));
        assertFalse(result.metadata().orElseThrow().canonicalUrl().orElse("").isEmpty(), "expected non-empty value");
        assertFalse(result.metadata().orElseThrow().keywords().isEmpty(), "expected non-empty value");
        assertTrue(result.metadata().orElseThrow().keywords().contains("rust"), "expected to contain: " + "rust");
        assertEquals("Jane Developer", result.metadata().orElseThrow().author());
        assertFalse(result.metadata().orElseThrow().viewport().isEmpty(), "expected non-empty value");
        assertEquals("kreuzcrawl/1.0", result.metadata().orElseThrow().generator());
        assertEquals("#ff6600", result.metadata().orElseThrow().themeColor());
        assertEquals("index, follow", result.metadata().orElseThrow().robots());
        assertEquals("en", result.metadata().orElseThrow().lang());
        assertEquals("ltr", result.metadata().orElseThrow().dir());
    }

    @Test
    void testMetadataOgVideoAudio() throws Exception {
        // Extracts og:video, og:audio, and og:locale:alternate metadata
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertEquals(200, result.statusCode());
        // skipped: field 'og.video' not available on result type
        // skipped: field 'og.audio' not available on result type
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

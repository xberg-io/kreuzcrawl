// E2e tests for category: markdown
package e2e_test

import (
	"strings"
	"testing"

	pkg "github.com/kreuzberg-dev/kreuzcrawl"
)

func Test_MarkdownBasicConversion(t *testing.T) {
	// HTML is always converted to markdown alongside raw HTML
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	result, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	var metadata_title string
	if result.Metadata.Title != nil {
		metadata_title = *result.Metadata.Title
	}
	var markdown string
	if result.Markdown != nil {
		markdown = *result.Markdown
	}
	if result.StatusCode != 200 {
		t.Errorf("equals mismatch: got %q", result.StatusCode)
	}
	if metadata_title != `Test` {
		t.Errorf("equals mismatch: got %q", metadata_title)
	}
	if len(result.Html) == 0 {
		t.Errorf("expected non-empty value")
	}
	if len(markdown) == 0 {
		t.Errorf("expected non-empty value")
	}
	if !strings.Contains(markdown, `Hello World`) {
		t.Errorf("expected to contain %s, got %q", `Hello World`, markdown)
	}
}

func Test_MarkdownCrawlAllPages(t *testing.T) {
	// All crawled pages have markdown field populated
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	_, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	// skipped: field 'crawl.pages_crawled' not available on result type
}

func Test_MarkdownFitContent(t *testing.T) {
	// Fit markdown removes navigation and boilerplate content
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	result, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	var markdown string
	if result.Markdown != nil {
		markdown = *result.Markdown
	}
	if result.StatusCode != 200 {
		t.Errorf("equals mismatch: got %q", result.StatusCode)
	}
	if len(markdown) == 0 {
		t.Errorf("expected non-empty value")
	}
}

func Test_MarkdownHeadingsAndParagraphs(t *testing.T) {
	// Markdown conversion preserves heading hierarchy and paragraph text
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	result, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	var markdown string
	if result.Markdown != nil {
		markdown = *result.Markdown
	}
	if len(markdown) == 0 {
		t.Errorf("expected non-empty value")
	}
	if !strings.Contains(markdown, `Main Title`) {
		t.Errorf("expected to contain %s, got %q", `Main Title`, markdown)
	}
}

func Test_MarkdownLinksConverted(t *testing.T) {
	// HTML links are converted to markdown link syntax
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	result, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	var markdown string
	if result.Markdown != nil {
		markdown = *result.Markdown
	}
	if result.StatusCode != 200 {
		t.Errorf("equals mismatch: got %q", result.StatusCode)
	}
	if len(result.Html) == 0 {
		t.Errorf("expected non-empty value")
	}
	if len(markdown) == 0 {
		t.Errorf("expected non-empty value")
	}
	if !strings.Contains(markdown, `Example`) {
		t.Errorf("expected to contain %s, got %q", `Example`, markdown)
	}
}

func Test_MarkdownWithCitations(t *testing.T) {
	// Markdown includes citation conversion with numbered references
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	result, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	var markdown string
	if result.Markdown != nil {
		markdown = *result.Markdown
	}
	if result.StatusCode != 200 {
		t.Errorf("equals mismatch: got %q", result.StatusCode)
	}
	if len(markdown) == 0 {
		t.Errorf("expected non-empty value")
	}
}

// E2e tests for category: middleware
package e2e_test

import (
	"strings"
	"testing"

	pkg "github.com/kreuzberg-dev/kreuzcrawl"
)

func Test_MiddlewareEngineCrawlWithDefaults(t *testing.T) {
	// Engine crawl with default middleware chain produces correct multi-page results
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	_, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	// skipped: field 'crawl.pages_crawled' not available on result type
	// skipped: field 'crawl.min_pages' not available on result type
}

func Test_MiddlewareNoopNoEffect(t *testing.T) {
	// Default middleware chain does not affect normal scraping
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
	if result.StatusCode != 200 {
		t.Errorf("equals mismatch: got %q", result.StatusCode)
	}
	if metadata_title != `Middleware Test` {
		t.Errorf("equals mismatch: got %q", metadata_title)
	}
}

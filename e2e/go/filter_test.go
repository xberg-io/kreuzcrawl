// E2e tests for category: filter
package e2e_test

import (
	"strings"
	"testing"

	pkg "github.com/kreuzberg-dev/kreuzcrawl"
)

func Test_FilterBm25CrawlIntegration(t *testing.T) {
	// BM25 filter works during multi-page crawl, keeping relevant pages
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	_, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	// skipped: field 'filter.remaining_contain_keyword' not available on result type
}

func Test_FilterBm25EmptyQuery(t *testing.T) {
	// BM25 filter with empty query passes all pages through
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

func Test_FilterBm25HighThreshold(t *testing.T) {
	// BM25 filter with very high threshold filters out all pages
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	_, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	// skipped: field 'filter.pages_after_filter' not available on result type
}

func Test_FilterBm25RelevantPages(t *testing.T) {
	// BM25 filter keeps only pages relevant to the query
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	_, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	// skipped: field 'filter.remaining_contain_keyword' not available on result type
}

func Test_FilterBm25ThresholdZero(t *testing.T) {
	// BM25 filter with zero threshold passes all pages
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

func Test_FilterNoopCrawlAllKept(t *testing.T) {
	// NoopFilter keeps all pages during a multi-page crawl
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	_, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	// skipped: field 'filter.pages_after_filter' not available on result type
}

func Test_FilterNoopPassesAll(t *testing.T) {
	// No content filter passes all crawled pages through
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

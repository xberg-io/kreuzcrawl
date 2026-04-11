// E2e tests for category: batch
package e2e_test

import (
	"strings"
	"testing"

	pkg "github.com/kreuzberg-dev/kreuzcrawl"
)

func Test_ScrapeBatchBasic(t *testing.T) {
	// Batch scrape of multiple URLs all succeeding
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	_, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	// skipped: field 'batch.completed_count' not available on result type
	// skipped: field 'batch.failed_count' not available on result type
	// skipped: field 'batch.total_count' not available on result type
}

func Test_ScrapeBatchPartialFailure(t *testing.T) {
	// Batch scrape with one URL failing returns partial results
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	_, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	// skipped: field 'batch.completed_count' not available on result type
	// skipped: field 'batch.failed_count' not available on result type
	// skipped: field 'batch.total_count' not available on result type
}

func Test_ScrapeBatchProgress(t *testing.T) {
	// Batch scrape results include specific URL
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	_, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	// skipped: field 'batch.total_count' not available on result type
	// skipped: field 'batch.results' not available on result type
}

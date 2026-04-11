// E2e tests for category: stream
package e2e_test

import (
	"strings"
	"testing"

	pkg "github.com/kreuzberg-dev/kreuzcrawl"
)

func Test_CrawlStreamEvents(t *testing.T) {
	// Crawl stream produces page and complete events
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	_, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	// skipped: field 'stream.event_count_min' not available on result type
	// skipped: field 'stream.has_page_event' not available on result type
	// skipped: field 'stream.has_complete_event' not available on result type
}

func Test_StreamDepthCrawl(t *testing.T) {
	// Stream produces events for multi-depth crawl with link following
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	_, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	// skipped: field 'stream.event_count_min' not available on result type
	// skipped: field 'stream.has_page_event' not available on result type
	// skipped: field 'stream.has_complete_event' not available on result type
}

func Test_StreamWithErrorEvent(t *testing.T) {
	// Stream emits page and complete events even when some pages fail
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	_, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	// skipped: field 'stream.has_page_event' not available on result type
	// skipped: field 'stream.has_complete_event' not available on result type
	// skipped: field 'stream.event_count_min' not available on result type
}

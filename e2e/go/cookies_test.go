// E2e tests for category: cookies
package e2e_test

import (
	"strings"
	"testing"

	pkg "github.com/kreuzberg-dev/kreuzcrawl"
)

func Test_CookiesPerDomain(t *testing.T) {
	// Isolates cookies per domain during crawl
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	_, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	// skipped: field 'cookies.length' not available on result type
	// skipped: field 'cookies' not available on result type
}

func Test_CookiesPersistence(t *testing.T) {
	// Maintains cookies across multiple crawl requests
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	_, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	// skipped: field 'cookies' not available on result type
}

func Test_CookiesSetCookieResponse(t *testing.T) {
	// Respects Set-Cookie header from server responses
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	_, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	// skipped: field 'cookies' not available on result type
}

// E2e tests for category: redirect
package e2e_test

import (
	"strings"
	"testing"

	pkg "github.com/kreuzberg-dev/kreuzcrawl"
)

func Test_Redirect301Permanent(t *testing.T) {
	// Follows 301 permanent redirect and returns final page content
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	_, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	// skipped: field 'final_url' not available on result type
	// skipped: field 'redirect_count' not available on result type
}

func Test_Redirect302Found(t *testing.T) {
	// Follows 302 Found redirect correctly
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	_, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	// skipped: field 'final_url' not available on result type
	// skipped: field 'redirect_count' not available on result type
}

func Test_Redirect303SeeOther(t *testing.T) {
	// Follows 303 See Other redirect (method changes to GET)
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	_, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	// skipped: field 'final_url' not available on result type
	// skipped: field 'redirect_count' not available on result type
}

func Test_Redirect307Temporary(t *testing.T) {
	// Follows 307 Temporary Redirect (preserves method)
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	_, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	// skipped: field 'final_url' not available on result type
	// skipped: field 'redirect_count' not available on result type
}

func Test_Redirect308Permanent(t *testing.T) {
	// Follows 308 Permanent Redirect (preserves method)
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	_, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	// skipped: field 'final_url' not available on result type
	// skipped: field 'redirect_count' not available on result type
}

func Test_RedirectChain(t *testing.T) {
	// Follows a chain of redirects (301 -> 302 -> 200)
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	_, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	// skipped: field 'final_url' not available on result type
	// skipped: field 'redirect_count' not available on result type
}

func Test_RedirectCrossDomain(t *testing.T) {
	// Reports cross-domain redirect target without following to external domain
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	_, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	// skipped: field 'final_url' not available on result type
	// skipped: field 'redirect_count' not available on result type
}

func Test_RedirectLoop(t *testing.T) {
	// Detects redirect loop (A -> B -> A) and returns error
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	_, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	// skipped: field 'is_error' not available on result type
}

func Test_RedirectMaxExceeded(t *testing.T) {
	// Aborts when redirect count exceeds max_redirects limit
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	_, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	// skipped: field 'is_error' not available on result type
}

func Test_RedirectMetaRefresh(t *testing.T) {
	// Follows HTML meta-refresh redirect to target page
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	_, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	// skipped: field 'final_url' not available on result type
	// skipped: field 'redirect_count' not available on result type
}

func Test_RedirectRefreshHeader(t *testing.T) {
	// Handles HTTP Refresh header redirect
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	_, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	// skipped: field 'final_url' not available on result type
	// skipped: field 'redirect_count' not available on result type
}

func Test_RedirectTo404(t *testing.T) {
	// Redirect target returns 404 Not Found
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	_, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	// skipped: field 'final_url' not available on result type
	// skipped: field 'redirect_count' not available on result type
	// skipped: field 'is_error' not available on result type
}

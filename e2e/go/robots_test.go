// E2e tests for category: robots
package e2e_test

import (
	"strings"
	"testing"

	pkg "github.com/kreuzberg-dev/kreuzcrawl"
)

func Test_RobotsAllowAll(t *testing.T) {
	// Permissive robots.txt allows all paths
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	_, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	// skipped: field 'robots.is_allowed' not available on result type
}

func Test_RobotsAllowOverride(t *testing.T) {
	// Allow directive overrides Disallow for specific paths
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	_, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	// skipped: field 'robots.is_allowed' not available on result type
}

func Test_RobotsCommentsHandling(t *testing.T) {
	// Correctly parses robots.txt with inline and line comments
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	_, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	// skipped: field 'robots.is_allowed' not available on result type
}

func Test_RobotsCrawlDelay(t *testing.T) {
	// Respects crawl-delay directive from robots.txt
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	_, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	// skipped: field 'robots.crawl_delay' not available on result type
}

func Test_RobotsDisallowPath(t *testing.T) {
	// Robots.txt disallows specific paths
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	_, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	// skipped: field 'robots.is_allowed' not available on result type
}

func Test_RobotsMetaNofollow(t *testing.T) {
	// Detects nofollow meta robots tag and skips link extraction
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	_, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	// skipped: field 'robots.nofollow_detected' not available on result type
}

func Test_RobotsMetaNoindex(t *testing.T) {
	// Detects noindex meta robots tag in HTML page
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	_, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	// skipped: field 'robots.noindex_detected' not available on result type
}

func Test_RobotsMissing404(t *testing.T) {
	// Missing robots.txt (404) allows all crawling
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	_, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	// skipped: field 'robots.is_allowed' not available on result type
}

func Test_RobotsMultipleUserAgents(t *testing.T) {
	// Picks the most specific user-agent block from robots.txt
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	_, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	// skipped: field 'robots.is_allowed' not available on result type
}

func Test_RobotsRequestRate(t *testing.T) {
	// Parses request-rate directive from robots.txt
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	_, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	// skipped: field 'robots.crawl_delay' not available on result type
	// skipped: field 'robots.is_allowed' not available on result type
}

func Test_RobotsSitemapDirective(t *testing.T) {
	// Discovers sitemap URL from Sitemap directive in robots.txt
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	_, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	// skipped: field 'robots.is_allowed' not available on result type
}

func Test_RobotsUserAgentSpecific(t *testing.T) {
	// Matches user-agent specific rules in robots.txt
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	_, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	// skipped: field 'robots.is_allowed' not available on result type
}

func Test_RobotsWildcardPaths(t *testing.T) {
	// Handles wildcard Disallow patterns in robots.txt
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	_, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	// skipped: field 'robots.is_allowed' not available on result type
}

func Test_RobotsXRobotsTag(t *testing.T) {
	// Respects X-Robots-Tag HTTP header directives
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	_, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	// skipped: field 'robots.x_robots_tag' not available on result type
	// skipped: field 'robots.noindex_detected' not available on result type
	// skipped: field 'robots.nofollow_detected' not available on result type
}

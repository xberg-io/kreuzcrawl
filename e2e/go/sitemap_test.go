// E2e tests for category: sitemap
package e2e_test

import (
	"strings"
	"testing"

	pkg "github.com/kreuzberg-dev/kreuzcrawl"
)

func Test_SitemapBasic(t *testing.T) {
	// Parses a standard urlset sitemap
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	_, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	// skipped: field 'urls.length' not available on result type
	// skipped: field 'has_lastmod' not available on result type
}

func Test_SitemapCompressedGzip(t *testing.T) {
	// Parses a gzip-compressed sitemap file
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	_, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	// skipped: field 'urls.length' not available on result type
}

func Test_SitemapEmpty(t *testing.T) {
	// Handles empty sitemap gracefully
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	_, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	// skipped: field 'urls.length' not available on result type
}

func Test_SitemapFromRobotsTxt(t *testing.T) {
	// Discovers sitemap via robots.txt Sitemap directive
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	_, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	// skipped: field 'urls.length' not available on result type
}

func Test_SitemapIndex(t *testing.T) {
	// Follows sitemap index to discover child sitemaps
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	_, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	// skipped: field 'urls.length' not available on result type
}

func Test_SitemapLastmodFilter(t *testing.T) {
	// Filters sitemap URLs by lastmod date
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	_, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	// skipped: field 'urls.length' not available on result type
	// skipped: field 'has_lastmod' not available on result type
}

func Test_SitemapOnlyMode(t *testing.T) {
	// Uses sitemap URLs exclusively without following page links
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	_, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	// skipped: field 'urls.length' not available on result type
}

func Test_SitemapXhtmlLinks(t *testing.T) {
	// Parses sitemap with XHTML namespace alternate links
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	_, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	// skipped: field 'urls.length' not available on result type
	// skipped: field 'has_lastmod' not available on result type
}

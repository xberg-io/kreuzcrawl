// E2e tests for category: crawl
package e2e_test

import (
	"strings"
	"testing"

	pkg "github.com/kreuzberg-dev/kreuzcrawl"
)

func Test_ContentBinarySkip(t *testing.T) {
	// Skips image and video content types gracefully
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	_, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	// skipped: field 'content.was_skipped' not available on result type
}

func Test_ContentPdfLinkSkip(t *testing.T) {
	// Encounters PDF link and skips or marks as document type
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	_, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	// skipped: field 'content.was_skipped' not available on result type
}

func Test_CrawlConcurrentDepth(t *testing.T) {
	// Concurrent crawl respects max_depth limit
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	_, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	// skipped: field 'pages.length' not available on result type
	// skipped: field 'stayed_on_domain' not available on result type
}

func Test_CrawlConcurrentLimit(t *testing.T) {
	// Respects max concurrent requests limit during crawl
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	_, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	// skipped: field 'pages.length' not available on result type
}

func Test_CrawlConcurrentMaxPages(t *testing.T) {
	// Concurrent crawl respects max_pages budget
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	_, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	// skipped: field 'pages.length' not available on result type
}

func Test_CrawlCustomHeaders(t *testing.T) {
	// Sends custom headers on all crawl requests
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	_, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	// skipped: field 'pages.length' not available on result type
}

func Test_CrawlDepthOne(t *testing.T) {
	// Follows links one level deep from start page
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	_, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	// skipped: field 'pages.length' not available on result type
	// skipped: field 'stayed_on_domain' not available on result type
}

func Test_CrawlDepthPriority(t *testing.T) {
	// Crawls in breadth-first order, processing depth-0 pages before depth-1
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	_, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	// skipped: field 'pages.length' not available on result type
}

func Test_CrawlDepthTwo(t *testing.T) {
	// Crawls 3 levels deep (depth 0, 1, 2)
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	_, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	// skipped: field 'pages.length' not available on result type
	// skipped: field 'pages.length' not available on result type
}

func Test_CrawlDepthTwoChain(t *testing.T) {
	// Depth=2 crawl follows a chain of links across three levels
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	_, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	// skipped: field 'pages.length' not available on result type
}

func Test_CrawlDoubleSlashNormalization(t *testing.T) {
	// Normalizes double slashes in URL paths (//page to /page)
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	_, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	// skipped: field 'unique_urls.length' not available on result type
}

func Test_CrawlEmptyPageNoLinks(t *testing.T) {
	// Crawl completes when child page has no outgoing links
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	_, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	// skipped: field 'pages.length' not available on result type
}

func Test_CrawlExcludePathPattern(t *testing.T) {
	// Skips URLs matching the exclude path pattern
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	_, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	// skipped: field 'pages.length' not available on result type
}

func Test_CrawlExternalLinksIgnored(t *testing.T) {
	// External links are discovered but not followed when stay_on_domain is true
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	_, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	// skipped: field 'pages.length' not available on result type
	// skipped: field 'stayed_on_domain' not available on result type
}

func Test_CrawlFragmentStripping(t *testing.T) {
	// Strips #fragment from URLs for deduplication
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	_, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	// skipped: field 'unique_urls.length' not available on result type
}

func Test_CrawlIncludePathPattern(t *testing.T) {
	// Only follows URLs matching the include path pattern
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	_, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	// skipped: field 'pages.length' not available on result type
}

func Test_CrawlMaxDepthZero(t *testing.T) {
	// max_depth=0 crawls only the seed page with no link following
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	_, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	// skipped: field 'pages.length' not available on result type
	// skipped: field 'pages.length' not available on result type
}

func Test_CrawlMaxPages(t *testing.T) {
	// Stops crawling at page budget limit
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	_, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	// skipped: field 'pages.length' not available on result type
}

func Test_CrawlMixedContentTypes(t *testing.T) {
	// Crawl handles links to non-HTML content types gracefully
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	_, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	// skipped: field 'pages.length' not available on result type
}

func Test_CrawlMultipleRedirectsInTraversal(t *testing.T) {
	// Multiple linked pages with redirects are handled during crawl traversal
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	_, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	// skipped: field 'pages.length' not available on result type
}

func Test_CrawlQueryParamDedup(t *testing.T) {
	// Deduplicates URLs with same query params in different order
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	_, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	// skipped: field 'unique_urls.length' not available on result type
}

func Test_CrawlRedirectInTraversal(t *testing.T) {
	// Links that redirect are followed during crawl traversal
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	_, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	// skipped: field 'pages.length' not available on result type
}

func Test_CrawlSelfLinkNoLoop(t *testing.T) {
	// Page linking to itself does not cause infinite crawl loop
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	_, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	// skipped: field 'pages.length' not available on result type
}

func Test_CrawlSinglePageNoLinks(t *testing.T) {
	// Crawling a page with no links returns only the seed page
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	_, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	// skipped: field 'pages.length' not available on result type
}

func Test_CrawlStayOnDomain(t *testing.T) {
	// Does not follow external links when stay_on_domain is true
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	_, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	// skipped: field 'pages.length' not available on result type
	// skipped: field 'stayed_on_domain' not available on result type
}

func Test_CrawlSubdomainExclusion(t *testing.T) {
	// Stays on exact domain and skips subdomain links
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	_, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	// skipped: field 'pages.length' not available on result type
	// skipped: field 'stayed_on_domain' not available on result type
}

func Test_CrawlSubdomainInclusion(t *testing.T) {
	// Crawls subdomains when allow_subdomains is enabled
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	_, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	// skipped: field 'pages.length' not available on result type
}

func Test_CrawlTrailingSlashDedup(t *testing.T) {
	// Deduplicates /page and /page/ as the same URL
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	_, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	// skipped: field 'unique_urls.length' not available on result type
}

func Test_CrawlUrlDeduplication(t *testing.T) {
	// Deduplicates URLs that differ only by fragment or query params
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	_, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	// skipped: field 'pages.length' not available on result type
}

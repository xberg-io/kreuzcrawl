// E2e tests for category: links
package e2e_test

import (
	"strings"
	"testing"

	pkg "github.com/kreuzberg-dev/kreuzcrawl"
)

func Test_LinksAnchorFragment(t *testing.T) {
	// Identifies fragment-only links as anchor type
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	result, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if !strings.Contains(result.Links[0].LinkType, `anchor`) {
		t.Errorf("expected to contain %s", `anchor`)
	}
}

func Test_LinksBaseTag(t *testing.T) {
	// Resolves relative URLs using base tag href
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	result, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if len(result.Links) <= 2 {
		t.Errorf("expected > 2, got %v", len(result.Links))
	}
	if !strings.Contains(result.Links[0].Url, `example.com`) {
		t.Errorf("expected to contain %s, got %q", `example.com`, result.Links[0].Url)
	}
}

func Test_LinksDocumentTypes(t *testing.T) {
	// Detects PDF, DOCX, XLSX links as document type
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	result, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if !strings.Contains(result.Links[0].LinkType, `document`) {
		t.Errorf("expected to contain %s", `document`)
	}
}

func Test_LinksEmptyHref(t *testing.T) {
	// Handles empty href attributes without errors
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	result, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if len(result.Links) <= 0 {
		t.Errorf("expected > 0, got %v", len(result.Links))
	}
	if !strings.Contains(result.Links[0].Url, `/valid`) {
		t.Errorf("expected to contain %s, got %q", `/valid`, result.Links[0].Url)
	}
}

func Test_LinksInternalExternalClassification(t *testing.T) {
	// Correctly classifies internal vs external links by domain
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	result, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if len(result.Links) <= 4 {
		t.Errorf("expected > 4, got %v", len(result.Links))
	}
	if !strings.Contains(result.Links[0].LinkType, `internal`) {
		t.Errorf("expected to contain %s", `internal`)
	}
	if !strings.Contains(result.Links[0].LinkType, `external`) {
		t.Errorf("expected to contain %s", `external`)
	}
}

func Test_LinksMailtoJavascriptSkip(t *testing.T) {
	// Skips mailto:, javascript:, and tel: scheme links
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	result, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if len(result.Links) <= 0 {
		t.Errorf("expected > 0, got %v", len(result.Links))
	}
	if strings.Contains(result.Links[0].Url, `mailto:`) {
		t.Errorf("expected NOT to contain %s, got %q", `mailto:`, result.Links[0].Url)
	}
}

func Test_LinksProtocolRelative(t *testing.T) {
	// Handles protocol-relative URLs (//example.com) correctly
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	result, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if len(result.Links) <= 1 {
		t.Errorf("expected > 1, got %v", len(result.Links))
	}
	if !strings.Contains(result.Links[0].Url, `//`) {
		t.Errorf("expected to contain %s, got %q", `//`, result.Links[0].Url)
	}
}

func Test_LinksRelAttributes(t *testing.T) {
	// Preserves rel=nofollow and rel=canonical attributes
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	result, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if len(result.Links) <= 0 {
		t.Errorf("expected > 0, got %v", len(result.Links))
	}
}

func Test_LinksRelativeParent(t *testing.T) {
	// Resolves ../ and ./ relative parent path links correctly
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	result, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if len(result.Links) <= 3 {
		t.Errorf("expected > 3, got %v", len(result.Links))
	}
}

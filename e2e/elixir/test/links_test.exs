# E2e tests for category: links
defmodule E2e.LinksTest do
  use ExUnit.Case, async: true

  describe "links_anchor_fragment" do
    test "Identifies fragment-only links as anchor type" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      assert String.contains?(result.links[0].link_type, "anchor")
    end
  end

  describe "links_base_tag" do
    test "Resolves relative URLs using base tag href" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      assert length(result.links) > 2
      assert String.contains?(result.links[0].url, "example.com")
    end
  end

  describe "links_document_types" do
    test "Detects PDF, DOCX, XLSX links as document type" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      assert String.contains?(result.links[0].link_type, "document")
    end
  end

  describe "links_empty_href" do
    test "Handles empty href attributes without errors" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      assert length(result.links) > 0
      assert String.contains?(result.links[0].url, "/valid")
    end
  end

  describe "links_internal_external_classification" do
    test "Correctly classifies internal vs external links by domain" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      assert length(result.links) > 4
      assert String.contains?(result.links[0].link_type, "internal")
      assert String.contains?(result.links[0].link_type, "external")
    end
  end

  describe "links_mailto_javascript_skip" do
    test "Skips mailto:, javascript:, and tel: scheme links" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      assert length(result.links) > 0
      refute String.contains?(result.links[0].url, "mailto:")
    end
  end

  describe "links_protocol_relative" do
    test "Handles protocol-relative URLs (//example.com) correctly" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      assert length(result.links) > 1
      assert String.contains?(result.links[0].url, "//")
    end
  end

  describe "links_rel_attributes" do
    test "Preserves rel=nofollow and rel=canonical attributes" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      assert length(result.links) > 0
    end
  end

  describe "links_relative_parent" do
    test "Resolves ../ and ./ relative parent path links correctly" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      assert length(result.links) > 3
    end
  end
end

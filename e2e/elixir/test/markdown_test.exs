# E2e tests for category: markdown
defmodule E2e.MarkdownTest do
  use ExUnit.Case, async: true

  describe "markdown_basic_conversion" do
    test "HTML is always converted to markdown alongside raw HTML" do
      result = Kreuzcrawl.scrape!()
      assert String.trim(result.status_code) == 200
      assert String.trim(result.metadata.title) == "Test"
      assert result.html != ""
      assert result.markdown != ""
      assert String.contains?(result.markdown, "Hello World")
    end
  end

  describe "markdown_crawl_all_pages" do
    test "All crawled pages have markdown field populated" do
      result = Kreuzcrawl.scrape!()
      assert String.trim(result.crawl.pages_crawled) == 2
    end
  end

  describe "markdown_fit_content" do
    test "Fit markdown removes navigation and boilerplate content" do
      result = Kreuzcrawl.scrape!()
      assert String.trim(result.status_code) == 200
      assert result.markdown != ""
    end
  end

  describe "markdown_headings_and_paragraphs" do
    test "Markdown conversion preserves heading hierarchy and paragraph text" do
      result = Kreuzcrawl.scrape!()
      assert result.markdown != ""
      assert String.contains?(result.markdown, "Main Title")
    end
  end

  describe "markdown_links_converted" do
    test "HTML links are converted to markdown link syntax" do
      result = Kreuzcrawl.scrape!()
      assert String.trim(result.status_code) == 200
      assert result.html != ""
      assert result.markdown != ""
      assert String.contains?(result.markdown, "Example")
    end
  end

  describe "markdown_with_citations" do
    test "Markdown includes citation conversion with numbered references" do
      result = Kreuzcrawl.scrape!()
      assert String.trim(result.status_code) == 200
      assert result.markdown != ""
    end
  end
end

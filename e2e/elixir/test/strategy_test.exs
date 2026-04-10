# E2e tests for category: strategy
defmodule E2e.StrategyTest do
  use ExUnit.Case, async: true

  describe "strategy_best_first_seed" do
    test "BestFirst strategy always processes the seed URL first" do
      result = Kreuzcrawl.scrape!()
      assert String.trim(result.crawl.pages_crawled) == 3
      assert String.contains?(result.strategy.first_page_url_contains, "/")
    end
  end

  describe "strategy_bfs_default_order" do
    test "BFS strategy visits pages in breadth-first order" do
      result = Kreuzcrawl.scrape!()
      assert String.trim(result.crawl.pages_crawled) == 5
      assert String.trim(result.strategy.crawl_order) == ["/", "/a", "/b", "/a/1", "/b/1"]
    end
  end

  describe "strategy_dfs_depth_first" do
    test "DFS strategy visits pages in depth-first order" do
      result = Kreuzcrawl.scrape!()
      assert String.trim(result.crawl.pages_crawled) == 5
      assert String.trim(result.strategy.crawl_order) == ["/", "/b", "/b/1", "/a", "/a/1"]
    end
  end
end

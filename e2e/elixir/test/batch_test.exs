# E2e tests for category: batch
defmodule E2e.BatchTest do
  use ExUnit.Case, async: true

  describe "scrape_batch_basic" do
    test "Batch scrape of multiple URLs all succeeding" do
      result = Kreuzcrawl.scrape!()
      assert String.trim(result.batch.completed_count) == 3
      assert String.trim(result.batch.failed_count) == 0
      assert String.trim(result.batch.total_count) == 3
    end
  end

  describe "scrape_batch_partial_failure" do
    test "Batch scrape with one URL failing returns partial results" do
      result = Kreuzcrawl.scrape!()
      assert String.trim(result.batch.completed_count) == 2
      assert String.trim(result.batch.failed_count) == 1
      assert String.trim(result.batch.total_count) == 3
    end
  end

  describe "scrape_batch_progress" do
    test "Batch scrape results include specific URL" do
      result = Kreuzcrawl.scrape!()
      assert String.trim(result.batch.total_count) == 2
      assert String.contains?(result.batch.results, "/target")
    end
  end
end

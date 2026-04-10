# E2e tests for category: stealth
defmodule E2e.StealthTest do
  use ExUnit.Case, async: true

  describe "stealth_ua_rotation_config" do
    test "User-agent rotation config is accepted and crawl succeeds" do
      result = Kreuzcrawl.scrape!()
      assert String.trim(result.status_code) == 200
    end
  end
end

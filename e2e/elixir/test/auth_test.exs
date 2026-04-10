# E2e tests for category: auth
defmodule E2e.AuthTest do
  use ExUnit.Case, async: true

  describe "auth_basic_http" do
    test "Sends HTTP Basic authentication header" do
      result = Kreuzcrawl.scrape!()
      assert String.trim(result.auth_header_sent) == true
      assert String.trim(result.status_code) == 200
    end
  end

  describe "auth_bearer_token" do
    test "Sends Bearer token in Authorization header" do
      result = Kreuzcrawl.scrape!()
      assert String.trim(result.auth_header_sent) == true
      assert String.trim(result.status_code) == 200
    end
  end

  describe "auth_custom_header" do
    test "Sends authentication via custom header (X-API-Key)" do
      result = Kreuzcrawl.scrape!()
      assert String.trim(result.auth_header_sent) == true
      assert String.trim(result.status_code) == 200
    end
  end
end

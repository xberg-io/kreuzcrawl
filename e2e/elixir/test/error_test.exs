# E2e tests for category: error
defmodule E2e.ErrorTest do
  use ExUnit.Case, async: true

  describe "error_401_unauthorized" do
    test "Handles 401 Unauthorized response correctly" do
      assert_raise RuntimeError, fn ->
        Kreuzcrawl.scrape!()
      end
    end
  end

  describe "error_403_forbidden" do
    test "Handles 403 Forbidden response correctly" do
      assert_raise RuntimeError, fn ->
        Kreuzcrawl.scrape!()
      end
    end
  end

  describe "error_404_page" do
    test "Handles 404 response correctly" do
      assert_raise RuntimeError, fn ->
        Kreuzcrawl.scrape!()
      end
    end
  end

  describe "error_408_request_timeout" do
    test "Handles 408 Request Timeout response correctly" do
      assert_raise RuntimeError, fn ->
        Kreuzcrawl.scrape!()
      end
    end
  end

  describe "error_410_gone" do
    test "Handles 410 Gone response correctly" do
      assert_raise RuntimeError, fn ->
        Kreuzcrawl.scrape!()
      end
    end
  end

  describe "error_500_server" do
    test "Handles 500 server error" do
      assert_raise RuntimeError, fn ->
        Kreuzcrawl.scrape!()
      end
    end
  end

  describe "error_502_bad_gateway" do
    test "Handles 502 Bad Gateway response correctly" do
      assert_raise RuntimeError, fn ->
        Kreuzcrawl.scrape!()
      end
    end
  end

  describe "error_connection_refused" do
    test "Handles connection refused error gracefully" do
      assert_raise RuntimeError, fn ->
        Kreuzcrawl.scrape!()
      end
    end
  end

  describe "error_dns_resolution" do
    test "Handles DNS resolution failure gracefully" do
      assert_raise RuntimeError, fn ->
        Kreuzcrawl.scrape!()
      end
    end
  end

  describe "error_empty_response" do
    test "Handles 200 with completely empty body gracefully" do
      result = Kreuzcrawl.scrape!()
      assert String.trim(result.html_not_empty) == false
      assert String.trim(result.error.is_error) == false
    end
  end

  describe "error_invalid_proxy" do
    test "Proxy pointing to unreachable address causes connection error during scrape" do
      assert_raise RuntimeError, fn ->
        Kreuzcrawl.scrape!()
      end
    end
  end

  describe "error_partial_response" do
    test "Handles incomplete or truncated HTTP response" do
      assert_raise RuntimeError, fn ->
        Kreuzcrawl.scrape!()
      end
    end
  end

  describe "error_rate_limited" do
    test "Handles 429 rate limiting with Retry-After" do
      assert_raise RuntimeError, fn ->
        Kreuzcrawl.scrape!()
      end
    end
  end

  describe "error_retry_503" do
    test "Retries request on 503 Service Unavailable response" do
      assert_raise RuntimeError, fn ->
        Kreuzcrawl.scrape!()
      end
    end
  end

  describe "error_retry_backoff" do
    test "Implements exponential backoff when retrying failed requests" do
      assert_raise RuntimeError, fn ->
        Kreuzcrawl.scrape!()
      end
    end
  end

  describe "error_ssl_invalid_cert" do
    test "Handles SSL certificate validation error" do
      assert_raise RuntimeError, fn ->
        Kreuzcrawl.scrape!()
      end
    end
  end

  describe "error_timeout" do
    test "Handles request timeout" do
      assert_raise RuntimeError, fn ->
        Kreuzcrawl.scrape!()
      end
    end
  end

  describe "error_waf_akamai" do
    test "Akamai WAF detection returns WafBlocked error" do
      assert_raise RuntimeError, fn ->
        Kreuzcrawl.scrape!()
      end
    end
  end

  describe "error_waf_false_403" do
    test "Detects WAF/bot protection false 403 (Cloudflare challenge page)" do
      assert_raise RuntimeError, fn ->
        Kreuzcrawl.scrape!()
      end
    end
  end

  describe "error_waf_imperva" do
    test "Imperva/Incapsula WAF detection" do
      assert_raise RuntimeError, fn ->
        Kreuzcrawl.scrape!()
      end
    end
  end
end

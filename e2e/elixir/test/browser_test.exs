# E2e tests for category: browser
defmodule E2e.BrowserTest do
  use ExUnit.Case, async: true

  describe "browser_config_auto_no_feature" do
    test "Browser mode 'auto' without browser feature enabled does not use browser" do
      result = Kreuzcrawl.scrape!()
      assert String.trim(result.status_code) == 200
      assert String.trim(result.browser.js_render_hint) == true
      assert String.trim(result.browser.browser_used) == false
    end
  end

  describe "browser_config_never_mode" do
    test "Browser mode 'never' prevents browser fallback even for SPA shell content" do
      result = Kreuzcrawl.scrape!()
      assert String.trim(result.status_code) == 200
      assert String.trim(result.browser.js_render_hint) == true
      assert String.trim(result.browser.browser_used) == false
    end
  end

  describe "browser_detect_minimal_page" do
    test "Does NOT flag a short but real content page as needing JS rendering" do
      result = Kreuzcrawl.scrape!()
      assert String.trim(result.status_code) == 200
      assert String.trim(result.browser.js_render_hint) == false
      assert String.trim(result.browser.browser_used) == false
    end
  end

  describe "browser_detect_next_empty" do
    test "Detects Next.js page with __NEXT_DATA__ but no rendered content as needing JS rendering" do
      result = Kreuzcrawl.scrape!()
      assert String.trim(result.status_code) == 200
      assert String.trim(result.browser.js_render_hint) == true
      assert String.trim(result.browser.browser_used) == false
    end
  end

  describe "browser_detect_next_rendered" do
    test "Does NOT flag Next.js page with full SSR content as needing JS rendering" do
      result = Kreuzcrawl.scrape!()
      assert String.trim(result.status_code) == 200
      assert String.trim(result.html_not_empty) == true
      assert String.trim(result.browser.js_render_hint) == false
      assert String.trim(result.browser.browser_used) == false
    end
  end

  describe "browser_detect_normal_page" do
    test "Does NOT flag a normal server-rendered page as needing JS rendering" do
      result = Kreuzcrawl.scrape!()
      assert String.trim(result.status_code) == 200
      assert String.trim(result.browser.js_render_hint) == false
      assert String.trim(result.browser.browser_used) == false
    end
  end

  describe "browser_detect_nuxt_shell" do
    test "Detects Nuxt SPA shell with empty #__nuxt div as needing JS rendering" do
      result = Kreuzcrawl.scrape!()
      assert String.trim(result.status_code) == 200
      assert String.trim(result.browser.js_render_hint) == true
      assert String.trim(result.browser.browser_used) == false
    end
  end

  describe "browser_detect_react_shell" do
    test "Detects React SPA shell with empty #root div as needing JS rendering" do
      result = Kreuzcrawl.scrape!()
      assert String.trim(result.status_code) == 200
      assert String.trim(result.html_not_empty) == true
      assert String.trim(result.browser.js_render_hint) == true
      assert String.trim(result.browser.browser_used) == false
    end
  end

  describe "browser_detect_vue_shell" do
    test "Detects Vue SPA shell with empty #app div as needing JS rendering" do
      result = Kreuzcrawl.scrape!()
      assert String.trim(result.status_code) == 200
      assert String.trim(result.browser.js_render_hint) == true
      assert String.trim(result.browser.browser_used) == false
    end
  end

  describe "browser_fallback_spa_render" do
    test "Browser auto re-fetches SPA shell when JS rendering is detected" do
      result = Kreuzcrawl.scrape!()
      assert String.trim(result.browser.js_render_hint) == true
      assert String.trim(result.browser.browser_used) == true
    end
  end

  describe "browser_fallback_waf_blocked" do
    test "Browser fallback triggers when WAF blocks the HTTP request (Cloudflare 403)" do
      result = Kreuzcrawl.scrape!()
      assert String.trim(result.browser.browser_used) == true
    end
  end

  describe "browser_mode_always" do
    test "Browser mode 'always' uses browser even for normal server-rendered pages" do
      result = Kreuzcrawl.scrape!()
      assert String.trim(result.browser.browser_used) == true
    end
  end
end

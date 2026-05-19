#![allow(clippy::unwrap_used, clippy::panic)]

#[cfg(feature = "browser-native")]
use std::sync::OnceLock;
#[cfg(any(feature = "browser-chromiumoxide", feature = "browser-native"))]
use std::time::Duration;

use kreuzcrawl::ScrollDirection;
#[cfg(any(feature = "browser-chromiumoxide", feature = "browser-native"))]
use kreuzcrawl::{BrowserBackend, BrowserConfig, BrowserMode, CrawlConfig};
use kreuzcrawl::{CrawlError, PageAction, create_engine, interact, validate_actions};

#[cfg(any(feature = "browser-chromiumoxide", feature = "browser-native"))]
use wiremock::matchers::{method, path};
#[cfg(any(feature = "browser-chromiumoxide", feature = "browser-native"))]
use wiremock::{Mock, MockServer, ResponseTemplate};

#[cfg(feature = "browser-native")]
static ALLOW_PRIVATE: OnceLock<()> = OnceLock::new();

#[cfg(feature = "browser-native")]
fn allow_private_network() {
    ALLOW_PRIVATE.get_or_init(|| {
        // SAFETY: tests run in a single process; the env var is written once
        // from `OnceLock::get_or_init` before any network call is made.
        #[allow(unsafe_code)]
        unsafe {
            std::env::set_var("KREUZCRAWL_ALLOW_PRIVATE_NETWORK", "1");
        }
    });
}

#[cfg(feature = "browser-chromiumoxide")]
#[tokio::test]
async fn chromiumoxide_interact_click_wait_screenshot_and_scrape() {
    let mock = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(
            r#"
                    <html>
                      <body style="height: 2000px">
                        <button id="go">Go</button>
                        <div id="status">idle</div>
                        <script>
                          document.getElementById('go').addEventListener('click', () => {
                            document.getElementById('status').textContent = 'clicked';
                            const done = document.createElement('div');
                            done.id = 'done';
                            done.textContent = 'ready';
                            document.body.appendChild(done);
                          });
                        </script>
                      </body>
                    </html>
                    "#,
            "text/html",
        ))
        .mount(&mock)
        .await;

    let config = CrawlConfig {
        browser: BrowserConfig {
            backend: BrowserBackend::Chromiumoxide,
            mode: BrowserMode::Always,
            timeout: Duration::from_secs(15),
            eval_script: Some("document.body.setAttribute('data-eval-script', 'ran')".to_string()),
            ..BrowserConfig::default()
        },
        ..CrawlConfig::default()
    };
    let engine = create_engine(Some(config)).unwrap();

    let result = interact(
        &engine,
        &mock.uri(),
        vec![
            PageAction::Click {
                selector: "#go".to_string(),
            },
            PageAction::Wait {
                milliseconds: None,
                selector: Some("#done".to_string()),
            },
            PageAction::Scroll {
                direction: ScrollDirection::Down,
                selector: None,
                amount: Some(100),
            },
            PageAction::Screenshot { full_page: Some(false) },
            PageAction::Scrape,
        ],
    )
    .await;

    let result = match result {
        Ok(result) => result,
        Err(CrawlError::BrowserError(message)) if message.contains("failed to launch browser") => {
            eprintln!("skipping chromiumoxide interact test because Chrome could not launch: {message}");
            return;
        }
        Err(error) => panic!("interact should succeed: {error:?}"),
    };

    assert_eq!(result.action_results.len(), 5);
    assert!(
        result.action_results.iter().all(|action| action.success),
        "all actions should succeed: {:?}",
        result.action_results
    );
    assert!(result.final_html.contains("clicked"));
    assert!(result.final_html.contains("id=\"done\""));
    assert!(result.final_html.contains("data-eval-script=\"ran\""));
    assert!(result.screenshot.as_ref().is_some_and(|bytes| !bytes.is_empty()));
    let scrape_data = result
        .action_results
        .last()
        .and_then(|action| action.data.as_ref())
        .and_then(|data| data.get("html"))
        .and_then(serde_json::Value::as_str)
        .unwrap_or_default();
    assert!(scrape_data.contains("clicked"));
}

#[cfg(feature = "browser-native")]
#[tokio::test]
async fn native_interact_click_type_wait_scroll_execute_js_and_scrape() {
    allow_private_network();
    let mock = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(
            r#"
                    <html>
                      <body style="height: 2000px">
                        <button id="go">Go</button>
                        <input id="name" value="">
                        <div id="status">idle</div>
                        <script>
                          document.getElementById('go').addEventListener('click', () => {
                            document.getElementById('status').textContent = 'clicked';
                            const done = document.createElement('div');
                            done.id = 'done';
                            done.textContent = 'ready';
                            document.body.appendChild(done);
                          });
                        </script>
                      </body>
                    </html>
                    "#,
            "text/html",
        ))
        .mount(&mock)
        .await;

    let config = CrawlConfig {
        browser: BrowserConfig {
            backend: BrowserBackend::Native,
            mode: BrowserMode::Always,
            timeout: Duration::from_secs(15),
            eval_script: Some("document.body.setAttribute('data-eval-script', 'ran')".to_string()),
            ..BrowserConfig::default()
        },
        ..CrawlConfig::default()
    };
    let engine = create_engine(Some(config)).unwrap();

    let result = interact(
        &engine,
        &mock.uri(),
        vec![
            PageAction::Click {
                selector: "#go".to_string(),
            },
            PageAction::TypeText {
                selector: "#name".to_string(),
                text: "kreuzcrawl".to_string(),
            },
            PageAction::Wait {
                milliseconds: None,
                selector: Some("#done".to_string()),
            },
            PageAction::Scroll {
                direction: ScrollDirection::Down,
                selector: None,
                amount: Some(100),
            },
            PageAction::ExecuteJs {
                script: "document.querySelector('#name').value".to_string(),
            },
            PageAction::ExecuteJs {
                script: "throw new Error('boom')".to_string(),
            },
            PageAction::Wait {
                milliseconds: Some(1_000),
                selector: Some("##".to_string()),
            },
            PageAction::Screenshot { full_page: Some(false) },
            PageAction::Scrape,
        ],
    )
    .await
    .unwrap();

    assert_eq!(result.action_results.len(), 9);
    assert!(
        result.action_results[..5].iter().all(|action| action.success),
        "non-screenshot actions should succeed: {:?}",
        result.action_results
    );
    assert!(!result.action_results[5].success);
    assert!(
        result.action_results[5]
            .error
            .as_deref()
            .is_some_and(|error| !error.is_empty())
    );
    assert!(!result.action_results[6].success);
    assert!(
        result.action_results[6]
            .error
            .as_deref()
            .is_some_and(|error| error.contains("selector syntax error")),
        "invalid selector should surface an evaluation error: {:?}",
        result.action_results[6]
    );
    assert!(!result.action_results[7].success);
    assert!(
        result.action_results[7]
            .error
            .as_deref()
            .is_some_and(|error| error.contains("BrowserBackend::Chromiumoxide"))
    );
    assert!(result.action_results[8].success);
    assert!(result.final_html.contains("clicked"));
    assert!(result.final_html.contains("id=\"done\""));
    assert!(result.final_html.contains("data-eval-script=\"ran\""));
    assert!(result.screenshot.is_none());

    let typed_value = result.action_results[4]
        .data
        .as_ref()
        .and_then(serde_json::Value::as_str)
        .unwrap_or_default();
    assert_eq!(typed_value, "kreuzcrawl");

    let scrape_data = result.action_results[8]
        .data
        .as_ref()
        .and_then(|data| data.get("html"))
        .and_then(serde_json::Value::as_str)
        .unwrap_or_default();
    assert!(scrape_data.contains("clicked"));
}

#[test]
fn validation_rejects_empty_wait_and_scroll_selectors() {
    let wait = validate_actions(&[PageAction::Wait {
        milliseconds: None,
        selector: Some(String::new()),
    }]);
    assert!(matches!(wait, Err(CrawlError::InvalidConfig(message)) if message.contains("wait selector")));

    let scroll = validate_actions(&[PageAction::Scroll {
        direction: ScrollDirection::Down,
        selector: Some(String::new()),
        amount: None,
    }]);
    assert!(matches!(scroll, Err(CrawlError::InvalidConfig(message)) if message.contains("scroll selector")));
}

#[test]
fn validation_rejects_i64_min_scroll_amount() {
    let result = validate_actions(&[PageAction::Scroll {
        direction: ScrollDirection::Down,
        selector: None,
        amount: Some(i64::MIN),
    }]);

    assert!(matches!(result, Err(CrawlError::InvalidConfig(message)) if message.contains("scroll amount")));
}

#[cfg(not(feature = "browser-chromiumoxide"))]
#[tokio::test]
async fn no_chromiumoxide_backend_interact_returns_unsupported() {
    let engine = create_engine(None).unwrap();

    let result = interact(&engine, "https://example.com", vec![PageAction::Scrape]).await;

    match result {
        Err(CrawlError::Unsupported(message)) => {
            assert!(message.contains("browser-chromiumoxide"));
        }
        other => panic!("expected Unsupported, got {other:?}"),
    }
}

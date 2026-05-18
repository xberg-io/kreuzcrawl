#![allow(clippy::unwrap_used, clippy::panic)]

#[cfg(feature = "browser-chromiumoxide")]
use std::time::Duration;

#[cfg(feature = "browser-chromiumoxide")]
use kreuzcrawl::ScrollDirection;
#[cfg(any(feature = "browser-chromiumoxide", feature = "browser-native"))]
use kreuzcrawl::{BrowserBackend, BrowserConfig, BrowserMode, CrawlConfig};
use kreuzcrawl::{CrawlError, PageAction, create_engine, interact};

#[cfg(feature = "browser-chromiumoxide")]
use wiremock::matchers::{method, path};
#[cfg(feature = "browser-chromiumoxide")]
use wiremock::{Mock, MockServer, ResponseTemplate};

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
async fn native_interact_returns_unsupported() {
    let config = CrawlConfig {
        browser: BrowserConfig {
            backend: BrowserBackend::Native,
            mode: BrowserMode::Always,
            ..BrowserConfig::default()
        },
        ..CrawlConfig::default()
    };
    let engine = create_engine(Some(config)).unwrap();

    let result = interact(&engine, "https://example.com", vec![PageAction::Scrape]).await;

    match result {
        Err(CrawlError::Unsupported(message)) => {
            assert!(message.contains("BrowserBackend::Native"));
            assert!(message.contains("BrowserBackend::Chromiumoxide"));
        }
        other => panic!("expected Unsupported, got {other:?}"),
    }
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

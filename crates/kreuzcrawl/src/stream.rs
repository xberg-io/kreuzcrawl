//! Streaming crawl operation that emits events as pages are crawled.

use tokio_stream::wrappers::ReceiverStream;

use crate::crawl;
use crate::types::{CrawlConfig, CrawlEvent};

/// Crawl a website and return a stream of events.
///
/// Each page crawled emits a [`CrawlEvent::Page`] event as it is processed
/// (truly streaming). When the crawl is complete, a [`CrawlEvent::Complete`]
/// event is emitted.
pub fn crawl_stream(url: &str, config: &CrawlConfig) -> ReceiverStream<CrawlEvent> {
    let url = url.to_owned();
    let config = config.clone();

    let (tx, rx) = tokio::sync::mpsc::channel(64);

    tokio::spawn(async move {
        match crawl::crawl_with_sender(&url, &config, Some(tx.clone())).await {
            Ok(result) => {
                let pages_crawled = result.pages.len();
                let _ = tx.send(CrawlEvent::Complete { pages_crawled }).await;
            }
            Err(_) => {
                let _ = tx.send(CrawlEvent::Complete { pages_crawled: 0 }).await;
            }
        }
    });

    ReceiverStream::new(rx)
}

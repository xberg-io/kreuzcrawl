//! Kreuzcrawl-facing adapter for the native browser backend.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use crate::{BrowserContext, Page, PageError, WaitUntil};

#[derive(Debug, Clone)]
pub struct NativeBrowserConfig {
    pub user_agent: Option<String>,
    pub timeout: Duration,
    pub wait_until: NativeBrowserWait,
    pub extra_headers: HashMap<String, String>,
    pub respect_robots_txt: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NativeBrowserWait {
    Load,
    NetworkIdle,
}

#[derive(Debug, Clone)]
pub struct RenderedPage {
    pub final_url: String,
    pub status: Option<u16>,
    pub html: String,
    pub headers: HashMap<String, String>,
}

pub async fn render_url(url: &str, config: &NativeBrowserConfig) -> Result<RenderedPage, PageError> {
    let mut context =
        BrowserContext::with_full_options("kreuzcrawl".to_string(), None, false, config.user_agent.clone());
    context.obey_robots = config.respect_robots_txt;
    let context = Arc::new(context);
    context
        .http_client
        .set_extra_headers(config.extra_headers.clone())
        .await;
    render_with_context(url, config, context).await
}

async fn render_with_context(
    url: &str,
    config: &NativeBrowserConfig,
    context: Arc<BrowserContext>,
) -> Result<RenderedPage, PageError> {
    let mut page = Page::new("page-1".to_string(), context);
    let wait_until = match config.wait_until {
        NativeBrowserWait::Load => WaitUntil::Load,
        NativeBrowserWait::NetworkIdle => WaitUntil::NetworkIdle0,
    };

    tokio::time::timeout(config.timeout, page.navigate_with_wait(url, wait_until))
        .await
        .map_err(|_| PageError::NetworkError(format!("browser timed out after {:?}", config.timeout)))??;

    let final_url = page.url_string();
    let status = page
        .network_events
        .iter()
        .rev()
        .find(|event| event.resource_type == "Document")
        .map(|event| event.status);
    let headers = page
        .network_events
        .iter()
        .rev()
        .find(|event| event.resource_type == "Document")
        .map(|event| (*event.response_headers).clone())
        .unwrap_or_default();

    let html = rendered_html(&page)
        .ok_or_else(|| PageError::ParseError(format!("no rendered DOM available for {final_url}")))?;

    Ok(RenderedPage {
        final_url,
        status,
        html,
        headers,
    })
}

fn rendered_html(page: &Page) -> Option<String> {
    page.with_dom(|dom| {
        if let Some(root) = dom.query_selector("html").ok().flatten() {
            dom.outer_html(root)
        } else {
            dom.outer_html(dom.document())
        }
    })
}

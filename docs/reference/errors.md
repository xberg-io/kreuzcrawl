---
title: "Error Reference"
---

## Error Reference

All error types thrown by the library across all languages.

### CrawlError

Errors that can occur during crawling, scraping, or mapping operations.

| Variant | Message | Description |
|---------|---------|-------------|
| `NotFound` | not_found: {0} | The requested page was not found (HTTP 404). |
| `Unauthorized` | unauthorized: {0} | The request was unauthorized (HTTP 401). |
| `Forbidden` | forbidden: {0} | The request was forbidden (HTTP 403). |
| `WafBlocked` | forbidden: waf/blocked: {0} | The request was blocked by a WAF or bot protection (HTTP 403 with WAF indicators). |
| `Timeout` | timeout: {0} | The request timed out. |
| `RateLimited` | rate_limited: {0} | The request was rate-limited (HTTP 429). |
| `ServerError` | server_error: {0} | A server error occurred (HTTP 5xx). |
| `BadGateway` | bad_gateway: {0} | A bad gateway error occurred (HTTP 502). |
| `Gone` | gone: {0} | The resource is permanently gone (HTTP 410). |
| `Connection` | connection: {0} | A connection error occurred. |
| `Dns` | dns: {0} | A DNS resolution error occurred. |
| `Ssl` | ssl: {0} | An SSL/TLS error occurred. |
| `DataLoss` | data_loss: {0} | Data was lost or truncated during transfer. |
| `BrowserError` | browser: {0} | The browser failed to launch, connect, or navigate. |
| `BrowserTimeout` | browser_timeout: {0} | The browser page load or rendering timed out. |
| `InvalidConfig` | invalid_config: {0} | The provided configuration is invalid. |
| `Other` | other: {0} | An unclassified error occurred. |

---

# MCP Server

Kreuzcrawl exposes its crawling capabilities as an
[MCP (Model Context Protocol)](https://modelcontextprotocol.io/) server, allowing AI
assistants to scrape, crawl, and map websites through tool calls. The MCP feature is
gated behind `mcp`.

## Starting the MCP server

### CLI

```bash
kreuzcrawl mcp
```

The server starts on **stdio transport** -- it reads JSON-RPC messages from stdin and
writes responses to stdout. Diagnostic messages go to stderr.

### Programmatic

```rust
use kreuzcrawl::mcp::start_mcp_server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    start_mcp_server().await?;
    Ok(())
}
```

With custom configuration:

```rust
use kreuzcrawl::mcp::start_mcp_server_with_config;
use kreuzcrawl::CrawlConfig;

let config = CrawlConfig {
    respect_robots_txt: true,
    stay_on_domain: true,
    ..Default::default()
};

start_mcp_server_with_config(config).await?;
```

## Available tools

The server registers 10 tools. Seven are fully implemented; three are registered as stubs
for future feature-gated functionality.

### Implemented tools

#### `scrape`

Scrape a single URL and extract content as markdown or JSON.

| Parameter     | Type    | Required | Description                                           |
| ------------- | ------- | -------- | ----------------------------------------------------- |
| `url`         | string  | yes      | URL to scrape (http/https only).                      |
| `format`      | string  | no       | `"markdown"` (default) or `"json"`.                   |
| `use_browser` | boolean | no       | Force browser rendering (requires `browser` feature). |

Example:

```json
{
  "url": "https://example.com",
  "format": "markdown"
}
```

#### `crawl`

Crawl a website following links up to a configured depth.

| Parameter        | Type    | Required | Description                           |
| ---------------- | ------- | -------- | ------------------------------------- |
| `url`            | string  | yes      | Starting URL.                         |
| `max_depth`      | integer | no       | Maximum link depth (max 100).         |
| `max_pages`      | integer | no       | Maximum pages to crawl (1--100,000).  |
| `format`         | string  | no       | `"markdown"` (default) or `"json"`.   |
| `stay_on_domain` | boolean | no       | Restrict crawling to the same domain. |

Example:

```json
{
  "url": "https://docs.rs/tokio",
  "max_depth": 2,
  "max_pages": 50,
  "stay_on_domain": true
}
```

#### `map`

Discover all pages on a website via links and sitemaps.

| Parameter            | Type    | Required | Description                               |
| -------------------- | ------- | -------- | ----------------------------------------- |
| `url`                | string  | yes      | Website URL.                              |
| `limit`              | integer | no       | Maximum URLs to return.                   |
| `search`             | string  | no       | Case-insensitive substring filter.        |
| `respect_robots_txt` | boolean | no       | Whether to respect robots.txt directives. |

Example:

```json
{
  "url": "https://example.com",
  "limit": 200,
  "search": "api"
}
```

#### `batch_scrape`

Scrape multiple URLs concurrently.

| Parameter     | Type             | Required | Description                         |
| ------------- | ---------------- | -------- | ----------------------------------- |
| `urls`        | array of strings | yes      | URLs to scrape (must not be empty). |
| `format`      | string           | no       | `"markdown"` (default) or `"json"`. |
| `concurrency` | integer          | no       | Maximum concurrent requests.        |

Example:

```json
{
  "urls": ["https://example.com", "https://example.org"],
  "format": "json",
  "concurrency": 5
}
```

#### `download`

Download a document from a URL and return metadata.

| Parameter  | Type    | Required | Description                     |
| ---------- | ------- | -------- | ------------------------------- |
| `url`      | string  | yes      | Document URL.                   |
| `max_size` | integer | no       | Maximum document size in bytes. |

Returns JSON with `url`, `mime_type`, `size`, `filename`, and `content_hash` for
documents, or page metadata if the URL returns HTML.

#### `interact`

Execute browser actions on a page and return per-action results plus the final DOM.

| Parameter | Type             | Required | Description                                  |
| --------- | ---------------- | -------- | -------------------------------------------- |
| `url`     | string           | yes      | URL to navigate to before executing actions  |
| `actions` | array of objects | yes      | Page actions such as click, wait, and scrape |

#### `get_version`

Return the kreuzcrawl library version. Takes no parameters.

### Stub tools (not yet implemented)

These tools are registered so that clients can discover them, but return placeholder
messages until their backing features are implemented:

| Tool           | Required feature | Description                               |
| -------------- | ---------------- | ----------------------------------------- |
| `screenshot`   | `browser`        | Capture a screenshot of a URL.            |
| `research`     | `ai`             | AI-driven research across multiple pages. |
| `crawl_status` | (job registry)   | Check the status of a crawl job.          |

## Integration with AI assistants

### Claude Desktop / Claude Code

Add to your MCP configuration (`claude_desktop_config.json` or `.mcp.json`):

```json
{
  "mcpServers": {
    "kreuzcrawl": {
      "command": "kreuzcrawl",
      "args": ["mcp"]
    }
  }
}
```

### Cursor

In Cursor settings, add an MCP server:

- **Name:** kreuzcrawl
- **Command:** `kreuzcrawl mcp`
- **Transport:** stdio

### Windsurf

Add to your Windsurf MCP configuration:

```json
{
  "mcpServers": {
    "kreuzcrawl": {
      "command": "kreuzcrawl",
      "args": ["mcp"]
    }
  }
}
```

### Docker

```json
{
  "mcpServers": {
    "kreuzcrawl": {
      "command": "docker",
      "args": ["run", "-i", "--rm", "kreuzcrawl:latest", "mcp"]
    }
  }
}
```

## Transport

The MCP server uses **stdio transport** exclusively. It reads newline-delimited JSON-RPC
messages from stdin and writes responses to stdout. This is the standard transport for
local MCP servers.

The server announces itself with:

- **Name:** `kreuzcrawl-mcp`
- **Version:** The crate version from `Cargo.toml`
- **Title:** Kreuzcrawl Web Crawling MCP Server
- **Capabilities:** Tools

The server instructions tell clients:

> Scrape, crawl, and map websites. Use 'scrape' for single pages, 'crawl' for following
> links across a site, 'map' for discovering all URLs, and 'batch_scrape' for processing
> multiple URLs concurrently. Use format: 'json' for structured output or 'markdown'
> (default) for human-readable content.

## Output formats

All content-returning tools support two output formats:

- **markdown** (default) -- Human-readable output with page titles, metadata summary,
  and markdown-converted content. Ideal for AI assistant consumption.
- **json** -- Pretty-printed JSON with all fields from the `ScrapeResult` or `CrawlResult`
  structs. Use when you need structured data for programmatic processing.

## Error handling

Tool errors are mapped to MCP error responses:

| Crawl error                          | MCP error                                        |
| ------------------------------------ | ------------------------------------------------ |
| Invalid URL                          | `invalid_params`                                 |
| Invalid config (bad max_depth, etc.) | `invalid_params`                                 |
| WAF blocked                          | `internal_error` (Blocked by WAF/bot protection) |
| Timeout                              | `internal_error` (Request timed out)             |
| Network error                        | `internal_error`                                 |

All errors include descriptive messages to help the AI assistant understand what went wrong.

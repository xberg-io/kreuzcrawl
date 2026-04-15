#[cfg(feature = "api")]
use std::sync::Arc;
use std::time::Duration;

use clap::{Parser, Subcommand};
use kreuzcrawl::{CrawlConfig, CrawlEngine, PerDomainThrottle, ProxyConfig};

#[derive(Parser)]
#[command(name = "kreuzcrawl", about = "High-performance web crawler and scraper", version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Scrape a single URL and extract metadata
    Scrape {
        /// URL to scrape
        url: String,
        /// Output format: json or markdown
        #[arg(long, default_value = "json")]
        format: String,
        /// Proxy URL
        #[arg(long)]
        proxy: Option<String>,
        /// Custom user agent
        #[arg(long)]
        user_agent: Option<String>,
        /// Request timeout in milliseconds
        #[arg(long, default_value = "30000")]
        timeout: u64,
        /// Respect robots.txt
        #[arg(long)]
        respect_robots_txt: bool,
    },
    /// Crawl a website following links
    Crawl {
        /// Seed URL(s) to crawl
        #[arg(required = true)]
        urls: Vec<String>,
        /// Maximum crawl depth
        #[arg(long, short = 'd', default_value = "2")]
        depth: usize,
        /// Maximum pages to crawl
        #[arg(long, short = 'n')]
        max_pages: Option<usize>,
        /// Maximum concurrent requests
        #[arg(long, short = 'c', default_value = "10")]
        concurrent: usize,
        /// Rate limit delay in milliseconds
        #[arg(long, default_value = "200")]
        rate_limit: u64,
        /// Output format: json or markdown
        #[arg(long, default_value = "json")]
        format: String,
        /// Proxy URL
        #[arg(long)]
        proxy: Option<String>,
        /// Custom user agent
        #[arg(long)]
        user_agent: Option<String>,
        /// Request timeout in milliseconds
        #[arg(long, default_value = "30000")]
        timeout: u64,
        /// Respect robots.txt
        #[arg(long)]
        respect_robots_txt: bool,
        /// Stay on the same domain
        #[arg(long)]
        stay_on_domain: bool,
    },
    /// Discover all URLs on a website via sitemaps and link extraction
    Map {
        /// URL to map
        url: String,
        /// Maximum URLs to return
        #[arg(long)]
        limit: Option<usize>,
        /// Filter URLs by substring
        #[arg(long)]
        search: Option<String>,
        /// Respect robots.txt
        #[arg(long)]
        respect_robots_txt: bool,
    },
    /// Start the REST API server
    #[cfg(feature = "api")]
    Serve {
        /// Host address to bind to
        #[arg(long, default_value = "0.0.0.0")]
        host: String,
        /// Port to listen on
        #[arg(long, default_value = "3000")]
        port: u16,
    },
    /// Start the MCP server (stdio transport)
    #[cfg(feature = "mcp")]
    Mcp {},
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Scrape {
            url,
            format,
            proxy,
            user_agent,
            timeout,
            respect_robots_txt,
        } => {
            let config = CrawlConfig {
                user_agent,
                request_timeout: Duration::from_millis(timeout),
                respect_robots_txt,
                proxy: proxy.map(|url| ProxyConfig {
                    url,
                    username: None,
                    password: None,
                }),
                ..Default::default()
            };
            let engine = CrawlEngine::builder().config(config).build().unwrap();
            match engine.scrape(&url).await {
                Ok(result) => {
                    if format == "markdown" {
                        if let Some(ref md) = result.markdown {
                            println!("{}", md.content);
                        } else {
                            eprintln!("No markdown content available");
                        }
                    } else {
                        println!("{}", serde_json::to_string_pretty(&result).unwrap());
                    }
                }
                Err(e) => {
                    eprintln!("Error: {e}");
                    std::process::exit(1);
                }
            }
        }
        Commands::Crawl {
            urls,
            depth,
            max_pages,
            concurrent,
            rate_limit,
            format,
            proxy,
            user_agent,
            timeout,
            respect_robots_txt,
            stay_on_domain,
        } => {
            let config = CrawlConfig {
                max_depth: Some(depth),
                max_pages,
                max_concurrent: Some(concurrent),
                user_agent,
                request_timeout: Duration::from_millis(timeout),
                respect_robots_txt,
                stay_on_domain,
                proxy: proxy.map(|url| ProxyConfig {
                    url,
                    username: None,
                    password: None,
                }),
                ..Default::default()
            };
            let engine = CrawlEngine::builder()
                .config(config)
                .rate_limiter(PerDomainThrottle::new(Duration::from_millis(rate_limit)))
                .build()
                .unwrap();

            if urls.len() == 1 {
                match engine.crawl(&urls[0]).await {
                    Ok(result) => {
                        if format == "markdown" {
                            for page in &result.pages {
                                if let Some(ref md) = page.markdown {
                                    println!("---\nURL: {}\n---\n{}\n", page.url, md.content);
                                }
                            }
                        } else {
                            println!("{}", serde_json::to_string_pretty(&result).unwrap());
                        }
                    }
                    Err(e) => {
                        eprintln!("Error: {e}");
                        std::process::exit(1);
                    }
                }
            } else {
                let url_refs: Vec<&str> = urls.iter().map(|s| s.as_str()).collect();
                let results = engine.batch_crawl(&url_refs).await;
                if format == "markdown" {
                    for (seed_url, result) in &results {
                        match result {
                            Ok(r) => {
                                for page in &r.pages {
                                    if let Some(ref md) = page.markdown {
                                        println!("---\nSeed: {seed_url}\nURL: {}\n---\n{}\n", page.url, md.content);
                                    }
                                }
                            }
                            Err(e) => eprintln!("Error crawling {seed_url}: {e}"),
                        }
                    }
                } else {
                    println!(
                        "{}",
                        serde_json::to_string_pretty(
                            &results
                                .iter()
                                .map(|(url, r)| {
                                    serde_json::json!({
                                        "seed_url": url,
                                        "result": match r {
                                            Ok(r) => serde_json::to_value(r).unwrap_or_default(),
                                            Err(e) => serde_json::json!({"error": e.to_string()}),
                                        }
                                    })
                                })
                                .collect::<Vec<_>>()
                        )
                        .unwrap()
                    );
                }
            }
        }
        Commands::Map {
            url,
            limit,
            search,
            respect_robots_txt,
        } => {
            let config = CrawlConfig {
                respect_robots_txt,
                map_limit: limit,
                map_search: search,
                ..Default::default()
            };
            let engine = CrawlEngine::builder().config(config).build().unwrap();
            match engine.map(&url).await {
                Ok(result) => {
                    for url_entry in &result.urls {
                        println!("{}", url_entry.url);
                    }
                }
                Err(e) => {
                    eprintln!("Error: {e}");
                    std::process::exit(1);
                }
            }
        }
        #[cfg(feature = "api")]
        Commands::Serve { host, port } => {
            let engine = CrawlEngine::builder().build().unwrap();
            eprintln!("Starting REST API server on {host}:{port}");
            if let Err(e) = kreuzcrawl::api::serve(&host, port, Arc::new(engine)).await {
                eprintln!("Server error: {e}");
                std::process::exit(1);
            }
        }
        #[cfg(feature = "mcp")]
        Commands::Mcp {} => {
            eprintln!("Starting MCP server (stdio transport)");
            if let Err(e) = kreuzcrawl::mcp::start_mcp_server().await {
                eprintln!("MCP server error: {e}");
                std::process::exit(1);
            }
        }
    }
}

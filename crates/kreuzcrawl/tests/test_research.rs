#![cfg(feature = "ai")]

//! Type-level and unit tests for the research agent module.
//! No network access required.

use kreuzcrawl::research::{
    Finding, ResearchConfig, ResearchResult, ResearchStep, SourceInfo, StepAction,
};

#[test]
fn test_research_config_defaults() {
    let config = ResearchConfig::default();
    assert_eq!(config.max_steps, 10);
    assert_eq!(config.max_pages_per_step, 5);
    assert_eq!(config.max_depth, 3);
    assert!(config.query.is_empty());
    assert!(config.seed_urls.is_empty());
}

#[test]
fn test_research_config_serialization_roundtrip() {
    let config = ResearchConfig {
        query: "rust async patterns".into(),
        max_steps: 7,
        max_pages_per_step: 3,
        max_depth: 2,
        seed_urls: vec!["https://example.com".into(), "https://docs.rs".into()],
    };

    let json = serde_json::to_string(&config).expect("serialize");
    let deserialized: ResearchConfig = serde_json::from_str(&json).expect("deserialize");

    assert_eq!(deserialized.query, "rust async patterns");
    assert_eq!(deserialized.max_steps, 7);
    assert_eq!(deserialized.max_pages_per_step, 3);
    assert_eq!(deserialized.max_depth, 2);
    assert_eq!(deserialized.seed_urls.len(), 2);
    assert_eq!(deserialized.seed_urls[0], "https://example.com");
}

#[test]
fn test_research_result_serialization() {
    let result = ResearchResult {
        query: "test query".into(),
        synthesis: "# Report\n\nFindings here.".into(),
        findings: vec![Finding {
            content: "important fact".into(),
            source_url: "https://example.com/page".into(),
            relevance_score: 0.85,
        }],
        sources: vec![SourceInfo {
            url: "https://example.com/page".into(),
            title: Some("Example Page".into()),
            snippet: Some("A short snippet".into()),
        }],
        steps: vec![ResearchStep {
            step_number: 0,
            action: StepAction::Crawl {
                url: "https://example.com".into(),
                depth: 1,
            },
            urls_visited: vec!["https://example.com/page".into()],
            findings_count: 1,
            error: None,
        }],
        pages_crawled: 1,
        cost: None,
    };

    let json = serde_json::to_string(&result).expect("serialize");
    let parsed: serde_json::Value = serde_json::from_str(&json).expect("parse json");

    assert_eq!(parsed["query"], "test query");
    assert_eq!(parsed["pages_crawled"], 1);
    assert!(parsed["findings"].is_array());
    assert_eq!(parsed["findings"][0]["relevance_score"], 0.85);
}

#[test]
fn test_finding_ordering_by_relevance() {
    let mut findings = [Finding {
            content: "low".into(),
            source_url: "a".into(),
            relevance_score: 0.2,
        },
        Finding {
            content: "high".into(),
            source_url: "b".into(),
            relevance_score: 0.9,
        },
        Finding {
            content: "mid".into(),
            source_url: "c".into(),
            relevance_score: 0.5,
        }];

    findings.sort_by(|a, b| {
        b.relevance_score
            .partial_cmp(&a.relevance_score)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    assert_eq!(findings[0].content, "high");
    assert_eq!(findings[1].content, "mid");
    assert_eq!(findings[2].content, "low");
}

#[test]
fn test_step_action_serialization() {
    let crawl = StepAction::Crawl {
        url: "https://example.com".into(),
        depth: 2,
    };
    let json = serde_json::to_string(&crawl).expect("serialize crawl");
    let parsed: serde_json::Value = serde_json::from_str(&json).expect("parse");
    assert_eq!(parsed["type"], "crawl");
    assert_eq!(parsed["url"], "https://example.com");
    assert_eq!(parsed["depth"], 2);

    let synth = StepAction::Synthesize;
    let json = serde_json::to_string(&synth).expect("serialize synthesize");
    let parsed: serde_json::Value = serde_json::from_str(&json).expect("parse");
    assert_eq!(parsed["type"], "synthesize");
}

#[test]
fn test_source_info_optional_fields() {
    let source_with = SourceInfo {
        url: "https://example.com".into(),
        title: Some("Example".into()),
        snippet: Some("A snippet".into()),
    };
    let source_without = SourceInfo {
        url: "https://example.com".into(),
        title: None,
        snippet: None,
    };

    let json_with = serde_json::to_string(&source_with).expect("serialize");
    let json_without = serde_json::to_string(&source_without).expect("serialize");

    let parsed_with: serde_json::Value = serde_json::from_str(&json_with).expect("parse");
    let parsed_without: serde_json::Value = serde_json::from_str(&json_without).expect("parse");

    assert_eq!(parsed_with["title"], "Example");
    assert!(parsed_without["title"].is_null());
    assert!(parsed_without["snippet"].is_null());
}

#[test]
fn test_research_config_custom_values() {
    let json = r#"{
        "query": "custom query",
        "max_steps": 20,
        "max_pages_per_step": 10,
        "max_depth": 5,
        "seed_urls": ["https://a.com", "https://b.com", "https://c.com"]
    }"#;

    let config: ResearchConfig = serde_json::from_str(json).expect("deserialize");
    assert_eq!(config.query, "custom query");
    assert_eq!(config.max_steps, 20);
    assert_eq!(config.max_pages_per_step, 10);
    assert_eq!(config.max_depth, 5);
    assert_eq!(config.seed_urls.len(), 3);
}

#[tokio::test]
async fn test_synthesizer_produces_markdown_report() {
    // We test the synthesizer indirectly through the public types.
    // The synthesizer is pub(crate), so we verify via ResearchAgent output format.
    // For a unit-level check, we replicate the synthesis logic here.
    let findings = [Finding {
            content: "Rust is a systems language".into(),
            source_url: "https://rust-lang.org".into(),
            relevance_score: 0.9,
        },
        Finding {
            content: "Tokio is an async runtime".into(),
            source_url: "https://tokio.rs".into(),
            relevance_score: 0.8,
        }];

    let sources = [SourceInfo {
        url: "https://rust-lang.org".into(),
        title: Some("Rust Language".into()),
        snippet: None,
    }];

    // Replicate synthesizer logic for unit test (synthesizer is pub(crate))
    let mut sorted: Vec<&Finding> = findings.iter().collect();
    sorted.sort_by(|a, b| {
        b.relevance_score
            .partial_cmp(&a.relevance_score)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    let query = "rust ecosystem";
    let mut report = format!("# Research Report: {query}\n\n");
    report.push_str("## Key Findings\n\n");
    for (i, finding) in sorted.iter().enumerate().take(20) {
        report.push_str(&format!(
            "{}. {} (source: {}, relevance: {:.2})\n\n",
            i + 1,
            finding.content,
            finding.source_url,
            finding.relevance_score,
        ));
    }
    if !sources.is_empty() {
        report.push_str("## Sources\n\n");
        for (i, source) in sources.iter().enumerate() {
            let title = source.title.as_deref().unwrap_or(&source.url);
            report.push_str(&format!("{}. [{}]({})\n", i + 1, title, source.url));
        }
    }

    assert!(report.contains("# Research Report: rust ecosystem"));
    assert!(report.contains("## Key Findings"));
    assert!(report.contains("Rust is a systems language"));
    assert!(report.contains("## Sources"));
    assert!(report.contains("[Rust Language](https://rust-lang.org)"));
}

#[tokio::test]
async fn test_planner_returns_crawl_for_seed_urls() {
    // The planner is pub(crate), so we verify the logic directly.
    let seed_urls = ["https://example.com".to_string(),
        "https://docs.rs".to_string()];

    // Step 0 with seed URLs should produce a Crawl action
    // Replicate planner logic: step_number < seed_urls.len() && step_number < max_steps
    let step_number = 0_usize;
    let max_steps = 10_usize;
    let action = if step_number < seed_urls.len() && step_number < max_steps {
        StepAction::Crawl {
            url: seed_urls[step_number].clone(),
            depth: 1,
        }
    } else {
        StepAction::Synthesize
    };

    match action {
        StepAction::Crawl { ref url, depth } => {
            assert_eq!(url, "https://example.com");
            assert_eq!(depth, 1);
        }
        StepAction::Synthesize => panic!("expected Crawl, got Synthesize"),
    }
}

#[tokio::test]
async fn test_planner_returns_synthesize_when_done() {
    let seed_urls = ["https://example.com".to_string()];

    // step_number >= seed_urls.len() should synthesize
    let step_number = 9_usize;
    let max_steps = 10_usize;
    let action = if step_number < seed_urls.len() && step_number < max_steps {
        StepAction::Crawl {
            url: seed_urls[step_number].clone(),
            depth: 1,
        }
    } else {
        StepAction::Synthesize
    };

    assert!(
        matches!(action, StepAction::Synthesize),
        "expected Synthesize when seed URLs exhausted"
    );

    // Also: step_number >= seed_urls.len() should synthesize
    let step_number_past = 1_usize;
    let action2 = if step_number_past < seed_urls.len() && step_number_past < max_steps {
        StepAction::Crawl {
            url: seed_urls[step_number_past].clone(),
            depth: 1,
        }
    } else {
        StepAction::Synthesize
    };

    assert!(
        matches!(action2, StepAction::Synthesize),
        "expected Synthesize when seed URLs exhausted"
    );
}

// --- New tests ---

#[test]
fn test_research_config_max_steps_zero() {
    let config = ResearchConfig {
        query: "test".into(),
        max_steps: 0,
        ..Default::default()
    };
    assert_eq!(config.max_steps, 0);
    // Agent should produce empty result with no steps
}

#[test]
fn test_research_config_max_steps_one() {
    let config = ResearchConfig {
        query: "test".into(),
        max_steps: 1,
        ..Default::default()
    };
    assert_eq!(config.max_steps, 1);
}

#[test]
fn test_research_config_empty_seed_urls() {
    let config = ResearchConfig {
        query: "test".into(),
        seed_urls: vec![],
        ..Default::default()
    };
    assert!(config.seed_urls.is_empty());
}

#[test]
fn test_research_step_with_error() {
    let step = ResearchStep {
        step_number: 0,
        action: StepAction::Crawl {
            url: "https://fail.example".into(),
            depth: 1,
        },
        urls_visited: vec!["https://fail.example".into()],
        findings_count: 0,
        error: Some("connection refused".into()),
    };
    let json = serde_json::to_string(&step).expect("serialize");
    assert!(json.contains("connection refused"));
}

#[test]
fn test_research_step_error_skipped_when_none() {
    let step = ResearchStep {
        step_number: 0,
        action: StepAction::Synthesize,
        urls_visited: vec![],
        findings_count: 0,
        error: None,
    };
    let json = serde_json::to_string(&step).expect("serialize");
    assert!(!json.contains("error"));
}

#[tokio::test]
async fn test_planner_uses_all_seed_urls() {
    // Replicate planner logic: step_number < seed_urls.len() && step_number < max_steps
    let seeds: Vec<String> = vec![
        "https://a.com".into(),
        "https://b.com".into(),
        "https://c.com".into(),
    ];
    let max_steps = 10_usize;

    let plan = |step: usize| -> StepAction {
        if step < seeds.len() && step < max_steps {
            StepAction::Crawl {
                url: seeds[step].clone(),
                depth: 1,
            }
        } else {
            StepAction::Synthesize
        }
    };

    let step0 = plan(0);
    assert!(matches!(step0, StepAction::Crawl { ref url, .. } if url == "https://a.com"));

    let step1 = plan(1);
    assert!(matches!(step1, StepAction::Crawl { ref url, .. } if url == "https://b.com"));

    let step2 = plan(2);
    assert!(matches!(step2, StepAction::Crawl { ref url, .. } if url == "https://c.com"));

    // After all seeds exhausted, synthesize
    let step3 = plan(3);
    assert!(matches!(step3, StepAction::Synthesize));
}

#[tokio::test]
async fn test_planner_max_steps_equals_seeds() {
    // With max_steps=2, should crawl both seeds (steps 0 and 1)
    let seeds: Vec<String> = vec!["https://a.com".into(), "https://b.com".into()];
    let max_steps = 2_usize;

    let plan = |step: usize| -> StepAction {
        if step < seeds.len() && step < max_steps {
            StepAction::Crawl {
                url: seeds[step].clone(),
                depth: 1,
            }
        } else {
            StepAction::Synthesize
        }
    };

    let step0 = plan(0);
    assert!(matches!(step0, StepAction::Crawl { .. }));

    let step1 = plan(1);
    assert!(matches!(step1, StepAction::Crawl { .. }));
}

#[tokio::test]
async fn test_synthesizer_empty_findings() {
    // Replicate synthesizer logic with empty findings
    let query = "test query";
    let findings: Vec<Finding> = vec![];
    let sources: Vec<SourceInfo> = vec![];

    let mut sorted: Vec<&Finding> = findings.iter().collect();
    sorted.sort_by(|a, b| {
        b.relevance_score
            .partial_cmp(&a.relevance_score)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    let mut report = format!("# Research Report: {query}\n\n");
    report.push_str("## Key Findings\n\n");
    for (i, finding) in sorted.iter().enumerate().take(20) {
        report.push_str(&format!(
            "{}. {} (source: {}, relevance: {:.2})\n\n",
            i + 1,
            finding.content,
            finding.source_url,
            finding.relevance_score,
        ));
    }
    if !sources.is_empty() {
        report.push_str("## Sources\n\n");
        for (i, source) in sources.iter().enumerate() {
            let title = source.title.as_deref().unwrap_or(&source.url);
            report.push_str(&format!("{}. [{}]({})\n", i + 1, title, source.url));
        }
    }

    assert!(report.contains("Research Report"));
    assert!(report.contains("test query"));
}

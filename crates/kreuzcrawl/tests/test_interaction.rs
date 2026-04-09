//! Integration tests for the page interaction module — serialization, deserialization, and validation.

#![cfg(feature = "interact")]

use kreuzcrawl::interact::{PageAction, ScrollDirection, validate_actions};

// ── Deserialization ──────────────────────────────────────────────────────

#[test]
fn test_click_action_deserialization() {
    let json = r##"{"type":"click","selector":"#btn"}"##;
    let action: PageAction = serde_json::from_str(json).expect("deserialize click");
    assert_eq!(
        action,
        PageAction::Click {
            selector: "#btn".to_owned()
        }
    );
}

#[test]
fn test_type_action_deserialization() {
    let json = r#"{"type":"type","selector":"input.name","text":"hello"}"#;
    let action: PageAction = serde_json::from_str(json).expect("deserialize type");
    assert_eq!(
        action,
        PageAction::Type {
            selector: "input.name".to_owned(),
            text: "hello".to_owned(),
        }
    );
}

#[test]
fn test_scroll_action_deserialization() {
    let json = r#"{"type":"scroll","direction":"down"}"#;
    let action: PageAction =
        serde_json::from_str(json).expect("deserialize scroll without selector");
    assert_eq!(
        action,
        PageAction::Scroll {
            direction: ScrollDirection::Down,
            selector: None,
            amount: None,
        }
    );

    let json_with_selector =
        r#"{"type":"scroll","direction":"up","selector":".container","amount":500}"#;
    let action2: PageAction =
        serde_json::from_str(json_with_selector).expect("deserialize scroll with selector");
    assert_eq!(
        action2,
        PageAction::Scroll {
            direction: ScrollDirection::Up,
            selector: Some(".container".to_owned()),
            amount: Some(500),
        }
    );
}

#[test]
fn test_wait_action_deserialization() {
    let json = r#"{"type":"wait","milliseconds":2000}"#;
    let action: PageAction = serde_json::from_str(json).expect("deserialize wait");
    assert_eq!(
        action,
        PageAction::Wait {
            milliseconds: Some(2000),
            selector: None,
        }
    );
}

#[test]
fn test_execute_js_deserialization() {
    let json = r#"{"type":"executeJs","script":"return 42;"}"#;
    let action: PageAction = serde_json::from_str(json).expect("deserialize executeJs");
    assert_eq!(
        action,
        PageAction::ExecuteJs {
            script: "return 42;".to_owned()
        }
    );
}

#[test]
fn test_screenshot_action_deserialization() {
    let json = r#"{"type":"screenshot","fullPage":true}"#;
    let action: PageAction = serde_json::from_str(json).expect("deserialize screenshot");
    assert_eq!(
        action,
        PageAction::Screenshot {
            full_page: Some(true)
        }
    );
}

#[test]
fn test_scrape_action_deserialization() {
    let json = r#"{"type":"scrape"}"#;
    let action: PageAction = serde_json::from_str(json).expect("deserialize scrape");
    assert_eq!(action, PageAction::Scrape {});
}

#[test]
fn test_scroll_direction_variants() {
    let up: ScrollDirection = serde_json::from_str(r#""up""#).expect("deserialize up");
    assert_eq!(up, ScrollDirection::Up);

    let down: ScrollDirection = serde_json::from_str(r#""down""#).expect("deserialize down");
    assert_eq!(down, ScrollDirection::Down);
}

#[test]
fn test_action_roundtrip_serialization() {
    let actions = vec![
        PageAction::Click {
            selector: "#submit".to_owned(),
        },
        PageAction::Type {
            selector: "input".to_owned(),
            text: "world".to_owned(),
        },
        PageAction::Press {
            key: "Enter".to_owned(),
        },
        PageAction::Scroll {
            direction: ScrollDirection::Down,
            selector: None,
            amount: Some(300),
        },
        PageAction::Wait {
            milliseconds: Some(1000),
            selector: None,
        },
        PageAction::Screenshot {
            full_page: Some(false),
        },
        PageAction::ExecuteJs {
            script: "document.title".to_owned(),
        },
        PageAction::Scrape {},
    ];

    for action in &actions {
        let json = serde_json::to_string(action).expect("serialize");
        let roundtripped: PageAction = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(*action, roundtripped, "roundtrip failed for {json}");
    }
}

// ── Validation ───────────────────────────────────────────────────────────

#[test]
fn test_validation_max_actions_exceeded() {
    let actions: Vec<PageAction> = (0..101).map(|_| PageAction::Scrape {}).collect();
    let err = validate_actions(&actions).unwrap_err();
    let msg = err.to_string();
    assert!(
        msg.contains("too many actions"),
        "expected 'too many actions' in: {msg}"
    );
}

#[test]
fn test_validation_max_wait_exceeded() {
    let actions = vec![
        PageAction::Wait {
            milliseconds: Some(200_000),
            selector: None,
        },
        PageAction::Wait {
            milliseconds: Some(200_000),
            selector: None,
        },
    ];
    let err = validate_actions(&actions).unwrap_err();
    let msg = err.to_string();
    assert!(
        msg.contains("total wait time"),
        "expected 'total wait time' in: {msg}"
    );
}

#[test]
fn test_validation_empty_selector_rejected() {
    let actions = vec![PageAction::Click {
        selector: String::new(),
    }];
    let err = validate_actions(&actions).unwrap_err();
    let msg = err.to_string();
    assert!(
        msg.contains("click selector must not be empty"),
        "expected empty selector error in: {msg}"
    );
}

#[test]
fn test_validation_empty_script_rejected() {
    let actions = vec![PageAction::ExecuteJs {
        script: String::new(),
    }];
    let err = validate_actions(&actions).unwrap_err();
    let msg = err.to_string();
    assert!(
        msg.contains("executeJs script must not be empty"),
        "expected empty script error in: {msg}"
    );
}

#[test]
fn test_validation_empty_key_rejected() {
    let actions = vec![PageAction::Press { key: String::new() }];
    let err = validate_actions(&actions).unwrap_err();
    let msg = err.to_string();
    assert!(
        msg.contains("press key must not be empty"),
        "expected empty key error in: {msg}"
    );
}

#[test]
fn test_validation_valid_actions_pass() {
    let actions = vec![
        PageAction::Click {
            selector: "#btn".to_owned(),
        },
        PageAction::Type {
            selector: "input".to_owned(),
            text: "test".to_owned(),
        },
        PageAction::Press {
            key: "Enter".to_owned(),
        },
        PageAction::Scroll {
            direction: ScrollDirection::Up,
            selector: None,
            amount: None,
        },
        PageAction::Wait {
            milliseconds: Some(500),
            selector: None,
        },
        PageAction::Screenshot {
            full_page: Some(true),
        },
        PageAction::ExecuteJs {
            script: "return 1;".to_owned(),
        },
        PageAction::Scrape {},
    ];
    validate_actions(&actions).expect("valid actions should pass");
}

// ── Deserialization edge cases ──────────────────────────────────────────

#[test]
fn test_missing_required_selector_in_click() {
    let json = r#"{"type":"click"}"#;
    assert!(serde_json::from_str::<PageAction>(json).is_err());
}

#[test]
fn test_unknown_fields_rejected() {
    let json = r##"{"type":"click","selector":"#btn","extra":"field"}"##;
    assert!(serde_json::from_str::<PageAction>(json).is_err());
}

#[test]
fn test_missing_type_field_rejected() {
    let json = r##"{"selector":"#btn"}"##;
    assert!(serde_json::from_str::<PageAction>(json).is_err());
}

#[test]
fn test_unknown_action_type_rejected() {
    let json = r##"{"type":"hover","selector":"#btn"}"##;
    assert!(serde_json::from_str::<PageAction>(json).is_err());
}

#[test]
fn test_array_of_actions_deserialization() {
    let json = r##"[
        {"type":"click","selector":"#btn"},
        {"type":"type","selector":"input","text":"hello"},
        {"type":"press","key":"Enter"}
    ]"##;
    let actions: Vec<PageAction> = serde_json::from_str(json).unwrap();
    assert_eq!(actions.len(), 3);
}

// ── Bounds validation ───────────────────────────────────────────────────

#[test]
fn test_validation_selector_too_long() {
    let long = "a".repeat(5000);
    let actions = vec![PageAction::Click { selector: long }];
    assert!(validate_actions(&actions).is_err());
}

#[test]
fn test_validation_script_too_long() {
    let long = "x".repeat(2_000_000);
    let actions = vec![PageAction::ExecuteJs { script: long }];
    assert!(validate_actions(&actions).is_err());
}

#[test]
fn test_validation_text_too_long() {
    let long = "y".repeat(2_000_000);
    let actions = vec![PageAction::Type {
        selector: "#input".into(),
        text: long,
    }];
    assert!(validate_actions(&actions).is_err());
}

#[test]
fn test_validation_single_wait_too_long() {
    let actions = vec![PageAction::Wait {
        milliseconds: Some(400_000),
        selector: None,
    }];
    assert!(validate_actions(&actions).is_err());
}

#[test]
fn test_validation_scroll_amount_too_large() {
    let actions = vec![PageAction::Scroll {
        direction: ScrollDirection::Down,
        selector: None,
        amount: Some(200_000),
    }];
    assert!(validate_actions(&actions).is_err());
}

#[test]
fn test_validation_negative_scroll_allowed_within_bounds() {
    let actions = vec![PageAction::Scroll {
        direction: ScrollDirection::Up,
        selector: None,
        amount: Some(-500),
    }];
    assert!(validate_actions(&actions).is_ok());
}

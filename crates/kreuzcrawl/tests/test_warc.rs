#![cfg(feature = "warc")]

use std::collections::HashSet;

use chrono::Utc;
use kreuzcrawl::warc::WarcWriter;
use tempfile::tempdir;

/// Helper: create a writer inside a temp directory and return (writer, file_path).
fn setup_writer() -> (WarcWriter, std::path::PathBuf) {
    let dir = tempdir().expect("create temp dir");
    let path = dir.keep().join("test.warc");
    let writer = WarcWriter::new(&path).expect("create writer");
    (writer, path)
}

fn sample_headers() -> Vec<(&'static str, &'static str)> {
    vec![("Content-Type", "text/html"), ("Server", "nginx")]
}

#[test]
fn test_warc_writer_creates_valid_file() {
    let (mut writer, path) = setup_writer();

    writer
        .write_warcinfo("kreuzcrawl/0.1.0", "localhost")
        .expect("write warcinfo");
    writer
        .write_response(
            "https://example.com",
            200,
            &sample_headers(),
            b"<html>hello</html>",
            Utc::now(),
        )
        .expect("write response");
    writer.finish().expect("finish");

    let content = std::fs::read_to_string(&path).expect("read file");

    // File must contain the WARC version header at least twice (warcinfo + response).
    assert!(
        content.matches("WARC/1.1\r\n").count() >= 2,
        "expected at least 2 WARC/1.1 version lines"
    );
    // warcinfo record present
    assert!(content.contains("WARC-Type: warcinfo"));
    assert!(content.contains("software: kreuzcrawl/0.1.0"));
    assert!(content.contains("hostname: localhost"));
    // response record present
    assert!(content.contains("WARC-Type: response"));
    assert!(content.contains("WARC-Target-URI: https://example.com"));
    assert!(content.contains("HTTP/1.1 200 OK"));
    assert!(content.contains("<html>hello</html>"));
}

#[test]
fn test_warc_multiple_records() {
    let (mut writer, path) = setup_writer();

    writer
        .write_warcinfo("kreuzcrawl/0.1.0", "localhost")
        .expect("write warcinfo");

    for i in 0..3 {
        writer
            .write_response(
                &format!("https://example.com/page/{i}"),
                200,
                &sample_headers(),
                format!("body-{i}").as_bytes(),
                Utc::now(),
            )
            .expect("write response");
    }

    writer.finish().expect("finish");

    let content = std::fs::read_to_string(&path).expect("read file");

    // 1 warcinfo + 3 response = 4 total records
    assert_eq!(
        content.matches("WARC/1.1\r\n").count(),
        4,
        "expected 4 WARC records"
    );

    // Each record is terminated by \r\n\r\n after the payload.
    // With 4 records we expect at least 4 double-CRLF terminators.
    assert!(
        content.matches("\r\n\r\n").count() >= 4,
        "expected record separators"
    );

    for i in 0..3 {
        assert!(
            content.contains(&format!("WARC-Target-URI: https://example.com/page/{i}")),
            "missing target URI for page {i}"
        );
        assert!(
            content.contains(&format!("body-{i}")),
            "missing body for page {i}"
        );
    }
}

#[test]
fn test_warc_record_ids_are_unique() {
    let (mut writer, _path) = setup_writer();

    writer
        .write_warcinfo("kreuzcrawl/0.1.0", "localhost")
        .expect("write warcinfo");

    let id1 = writer
        .write_response(
            "https://example.com/a",
            200,
            &sample_headers(),
            b"a",
            Utc::now(),
        )
        .expect("write response 1");

    let id2 = writer
        .write_response(
            "https://example.com/b",
            200,
            &sample_headers(),
            b"b",
            Utc::now(),
        )
        .expect("write response 2");

    writer.finish().expect("finish");

    let mut ids = HashSet::new();
    ids.insert(id1.clone());
    ids.insert(id2.clone());

    assert_eq!(ids.len(), 2, "record IDs must be distinct: {id1} vs {id2}");

    // Verify URN format
    for id in &ids {
        assert!(
            id.starts_with("<urn:uuid:") && id.ends_with('>'),
            "record ID must be <urn:uuid:...> format, got: {id}"
        );
    }
}

#[test]
fn test_warc_binary_body_preserved() {
    let (mut writer, path) = setup_writer();

    // Binary payload with non-UTF8 bytes.
    let binary_body: Vec<u8> = (0..=255).collect();

    writer
        .write_response(
            "https://example.com/bin",
            200,
            &[("Content-Type", "application/octet-stream")],
            &binary_body,
            Utc::now(),
        )
        .expect("write response");
    writer.finish().expect("finish");

    let file_bytes = std::fs::read(&path).expect("read file bytes");

    // The binary body must appear verbatim somewhere in the file.
    assert!(
        file_bytes
            .windows(binary_body.len())
            .any(|w| w == binary_body.as_slice()),
        "binary body not found verbatim in WARC file"
    );
}

#[test]
fn test_warc_content_length_correct() {
    let (mut writer, path) = setup_writer();

    let body = b"Hello, WARC world!";
    let headers = vec![("Content-Type", "text/plain")];

    writer
        .write_response("https://example.com", 200, &headers, body, Utc::now())
        .expect("write response");
    writer.finish().expect("finish");

    let content = std::fs::read_to_string(&path).expect("read file");

    // Extract the Content-Length value from the WARC headers.
    let content_length_line = content
        .lines()
        .find(|line| line.starts_with("Content-Length:"))
        .expect("Content-Length header must be present");

    let claimed_length: usize = content_length_line
        .split(':')
        .nth(1)
        .expect("value after colon")
        .trim()
        .trim_end_matches('\r')
        .parse()
        .expect("numeric Content-Length");

    // Compute the expected HTTP block size:
    // "HTTP/1.1 200 OK\r\n" + "Content-Type: text/plain\r\n" + "\r\n" + body
    let status_line = "HTTP/1.1 200 OK\r\n";
    let header_line = "Content-Type: text/plain\r\n";
    let separator = "\r\n";
    let expected_length = status_line.len() + header_line.len() + separator.len() + body.len();

    assert_eq!(
        claimed_length, expected_length,
        "Content-Length {claimed_length} does not match actual HTTP block size {expected_length}"
    );
}

#[test]
fn test_warc_empty_body() {
    let (mut writer, path) = setup_writer();

    let headers = vec![("Content-Type", "text/html")];

    writer
        .write_response("https://example.com", 200, &headers, b"", Utc::now())
        .expect("write response");
    writer.finish().expect("finish");

    let content = std::fs::read_to_string(&path).expect("read file");

    // Extract Content-Length from the WARC response record.
    let content_length_line = content
        .lines()
        .find(|line| line.starts_with("Content-Length:"))
        .expect("Content-Length header must be present");

    let claimed_length: usize = content_length_line
        .split(':')
        .nth(1)
        .expect("value after colon")
        .trim()
        .trim_end_matches('\r')
        .parse()
        .expect("numeric Content-Length");

    // Expected: status line + header + separator, no body bytes.
    let status_line = "HTTP/1.1 200 OK\r\n";
    let header_line = "Content-Type: text/html\r\n";
    let separator = "\r\n";
    let expected_length = status_line.len() + header_line.len() + separator.len();

    assert_eq!(
        claimed_length, expected_length,
        "Content-Length {claimed_length} does not match expected {expected_length} for empty body"
    );
}

#[test]
fn test_warc_unicode_url() {
    let (mut writer, path) = setup_writer();

    let url = "https://example.com/données/résumé";
    writer
        .write_response(url, 200, &sample_headers(), b"unicode page", Utc::now())
        .expect("write response");
    writer.finish().expect("finish");

    let content = std::fs::read_to_string(&path).expect("read file");
    assert!(
        content.contains(&format!("WARC-Target-URI: {url}")),
        "unicode URL must be preserved verbatim in WARC-Target-URI"
    );
    assert!(content.contains("unicode page"));
}

#[test]
fn test_warc_header_crlf_injection_rejected() {
    let (mut writer, _path) = setup_writer();

    // Header value containing CRLF should be rejected.
    let headers = vec![("X-Evil", "value\r\nInjected-Header: malicious")];
    let result = writer.write_response("https://example.com", 200, &headers, b"body", Utc::now());

    assert!(
        result.is_err(),
        "CRLF injection in header value must be rejected"
    );

    // Header name containing newline should also be rejected.
    let (mut writer2, _path2) = setup_writer();
    let headers2 = vec![("X-Evil\nName", "value")];
    let result2 =
        writer2.write_response("https://example.com", 200, &headers2, b"body", Utc::now());

    assert!(result2.is_err(), "newline in header name must be rejected");
}

#[test]
fn test_warc_header_with_special_chars() {
    let (mut writer, path) = setup_writer();

    // Headers with colons, equals signs, and semicolons in values are valid.
    let headers = vec![
        ("Content-Type", "text/html; charset=utf-8"),
        ("Set-Cookie", "name=value; path=/; HttpOnly"),
        ("X-Custom", "key=value:extra"),
    ];

    writer
        .write_response("https://example.com", 200, &headers, b"ok", Utc::now())
        .expect("write response with special chars in headers");
    writer.finish().expect("finish");

    let content = std::fs::read_to_string(&path).expect("read file");
    assert!(content.contains("Content-Type: text/html; charset=utf-8"));
    assert!(content.contains("Set-Cookie: name=value; path=/; HttpOnly"));
    assert!(content.contains("X-Custom: key=value:extra"));
}

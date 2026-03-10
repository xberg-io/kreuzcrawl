//! Content type detection (HTML, binary, PDF).

/// Binary file extensions used to detect non-HTML content.
static BINARY_EXTENSIONS: &[&str] = &[
    ".jpg", ".jpeg", ".png", ".gif", ".bmp", ".webp", ".svg", ".ico", ".tiff", ".mp4", ".avi",
    ".mov", ".wmv", ".flv", ".mkv", ".webm", ".mp3", ".wav", ".ogg", ".flac", ".aac", ".wma",
    ".exe", ".dll", ".so", ".bin",
];

/// Check whether content appears to be HTML based on Content-Type header or body content.
pub(crate) fn is_html_content(content_type: &str, body: &str) -> bool {
    if content_type.contains("html") {
        return true;
    }
    let trimmed = body.trim_start();
    if !trimmed.starts_with('<') {
        return false;
    }
    let lower = trimmed.to_lowercase();
    // Reject XML/SVG that isn't HTML
    if lower.starts_with("<?xml") && !lower.contains("<html") {
        return false;
    }
    // Accept common HTML markers
    lower.starts_with("<!doctype")
        || lower.starts_with("<html")
        || lower.starts_with("<head")
        || lower.starts_with("<body")
        || lower.starts_with("<div")
        || lower.starts_with("<p")
        || lower.starts_with("<h1")
        || lower.starts_with("<script")
        || lower.starts_with("<meta")
        || lower.starts_with("<link")
        || lower.starts_with("<!")
}

/// Check whether a Content-Type header indicates binary content.
pub(crate) fn is_binary_content_type(ct: &str) -> bool {
    let lower = ct.to_lowercase();
    lower.starts_with("image/")
        || lower.starts_with("video/")
        || lower.starts_with("audio/")
        || lower.starts_with("application/octet-stream")
        || lower.starts_with("application/zip")
        || lower.starts_with("application/pdf")
}

/// Check whether a URL has a binary file extension.
pub(crate) fn is_binary_url(url: &str) -> bool {
    let lower = url.to_lowercase();
    let path = lower.split('?').next().unwrap_or(&lower);
    let path = path.split('#').next().unwrap_or(path);
    BINARY_EXTENSIONS.iter().any(|ext| path.ends_with(ext))
}

/// Check whether content is a PDF based on Content-Type or body magic bytes.
pub(crate) fn is_pdf_content(ct: &str, body: &str) -> bool {
    ct.to_lowercase().contains("application/pdf") || body.starts_with("%PDF")
}

/// Check whether a URL has a `.pdf` extension.
pub(crate) fn is_pdf_url(url: &str) -> bool {
    let lower = url.to_lowercase();
    let path = lower.split('?').next().unwrap_or(&lower);
    let path = path.split('#').next().unwrap_or(path);
    path.ends_with(".pdf")
}

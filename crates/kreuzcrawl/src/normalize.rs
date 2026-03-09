//! URL normalization and resolution utilities.

use url::Url;

/// Remove trailing slashes (except root) and collapse double slashes in a URL path.
fn clean_url_path(u: &mut Url) {
    let path = u.path().to_owned();
    if path.len() > 1 && path.ends_with('/') {
        u.set_path(&path[..path.len() - 1]);
    }
    let path = u.path().to_owned();
    if path.contains("//") {
        u.set_path(&path.replace("//", "/"));
    }
}

/// Normalize a URL by removing fragments, sorting query parameters,
/// removing trailing slashes (except root), and fixing double slashes in the path.
pub(crate) fn normalize_url(raw: &str) -> String {
    if let Ok(mut u) = Url::parse(raw) {
        u.set_fragment(None);
        // Sort query params
        let pairs: Vec<(String, String)> = u
            .query_pairs()
            .map(|(k, v)| (k.into_owned(), v.into_owned()))
            .collect();
        if !pairs.is_empty() {
            let mut sorted = pairs;
            sorted.sort();
            let query_str: String = sorted
                .iter()
                .map(|(k, v)| format!("{k}={v}"))
                .collect::<Vec<_>>()
                .join("&");
            u.set_query(Some(&query_str));
        }
        clean_url_path(&mut u);
        u.to_string()
    } else {
        raw.to_owned()
    }
}

/// Normalize a URL for deduplication during crawling.
///
/// Strips query parameters and fragments, removes trailing slashes (except root),
/// and fixes double slashes in the path.
pub(crate) fn normalize_url_for_dedup(raw: &str) -> String {
    if let Ok(mut u) = Url::parse(raw) {
        u.set_fragment(None);
        u.set_query(None);
        clean_url_path(&mut u);
        u.to_string()
    } else {
        raw.to_owned()
    }
}

/// Resolve a redirect target against a base URL.
///
/// If the target is already absolute, returns it as-is. Otherwise, resolves
/// it relative to the base URL.
pub(crate) fn resolve_redirect(base_url: &str, target: &str) -> String {
    if target.starts_with("http://") || target.starts_with("https://") {
        return target.to_owned();
    }
    if let Ok(base) = Url::parse(base_url)
        && let Ok(resolved) = base.join(target)
    {
        return resolved.to_string();
    }
    target.to_owned()
}

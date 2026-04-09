//! Integration tests for `BrowserProfile` — filesystem-backed persistent browser profiles.

#![cfg(feature = "browser")]

use kreuzcrawl::browser_profile::BrowserProfile;

#[test]
fn test_profile_path_deterministic() {
    let a = BrowserProfile::new("deterministic-test").unwrap();
    let b = BrowserProfile::new("deterministic-test").unwrap();
    assert_eq!(a.user_data_dir, b.user_data_dir);
    assert_eq!(&*a.name, &*b.name);
}

#[test]
fn test_profile_name_validation_rejects_slashes() {
    assert!(BrowserProfile::new("a/b").is_err());
    assert!(BrowserProfile::new("a\\b").is_err());
}

#[test]
fn test_profile_name_validation_rejects_special_chars() {
    assert!(BrowserProfile::new("bad name").is_err());
    assert!(BrowserProfile::new("foo@bar").is_err());
    assert!(BrowserProfile::new("test!").is_err());
}

#[test]
fn test_profile_name_validation_rejects_empty() {
    assert!(BrowserProfile::new("").is_err());
}

#[test]
fn test_profile_name_rejects_unicode() {
    // Homograph / normalization attacks
    assert!(BrowserProfile::new("caf\u{00e9}").is_err());
    assert!(BrowserProfile::new("\u{0430}dmin").is_err()); // Cyrillic 'a'
    assert!(BrowserProfile::new("profile\u{200b}name").is_err()); // zero-width space
    assert!(BrowserProfile::new("\u{ff41}\u{ff42}\u{ff43}").is_err()); // fullwidth 'abc'
}

#[test]
fn test_profile_exists_false_initially() {
    let profile = BrowserProfile::new("nonexistent-profile-xyz").unwrap();
    assert!(!profile.exists());
}

#[test]
fn test_profile_create_and_delete() {
    let dir = tempfile::tempdir().unwrap();
    let profile = BrowserProfile {
        name: Box::from("ephemeral"),
        user_data_dir: dir.path().join("ephemeral"),
    };

    assert!(!profile.exists());

    profile.create().unwrap();
    assert!(profile.exists());

    profile.delete().unwrap();
    assert!(!profile.exists());
}

#[test]
fn test_profile_list_all() {
    let dir = tempfile::tempdir().unwrap();
    let base = dir.path();

    std::fs::create_dir_all(base.join("alpha")).unwrap();
    std::fs::create_dir_all(base.join("beta")).unwrap();
    // A plain file should not appear in the listing.
    std::fs::write(base.join("not-a-dir.txt"), b"ignored").unwrap();

    let names = kreuzcrawl::browser_profile::list_profiles_in(base).unwrap();
    assert_eq!(names.len(), 2);
    assert_eq!(&*names[0], "alpha");
    assert_eq!(&*names[1], "beta");
}

#[test]
fn test_chrome_args_contains_user_data_dir() {
    let profile = BrowserProfile::new("chrome-args-test").unwrap();
    let args = profile.chrome_args();
    assert_eq!(args.len(), 1);
    assert!(args[0].starts_with("--user-data-dir="));
    assert!(args[0].contains("chrome-args-test"));
}

#[cfg(unix)]
#[test]
fn test_profile_delete_symlink_rejected() {
    let dir = tempfile::tempdir().unwrap();
    let real_dir = dir.path().join("real");
    std::fs::create_dir(&real_dir).unwrap();

    let symlink_path = dir.path().join("linked");
    std::os::unix::fs::symlink(&real_dir, &symlink_path).unwrap();

    let profile = BrowserProfile {
        name: Box::from("linked"),
        user_data_dir: symlink_path,
    };

    let err = profile.delete().unwrap_err();
    assert!(
        err.to_string().contains("symlink"),
        "error should mention symlink: {err}"
    );
    // The real directory must still exist.
    assert!(real_dir.is_dir());
}

#[cfg(unix)]
#[test]
fn test_profile_permissions_on_unix() {
    use std::os::unix::fs::PermissionsExt;

    let dir = tempfile::tempdir().unwrap();
    let profile = BrowserProfile {
        name: Box::from("perms-test"),
        user_data_dir: dir.path().join("perms-test"),
    };

    profile.create().unwrap();

    let metadata = std::fs::metadata(&profile.user_data_dir).unwrap();
    let mode = metadata.permissions().mode() & 0o777;
    assert_eq!(
        mode, 0o700,
        "profile directory should have 0o700 permissions, got {mode:#o}"
    );
}

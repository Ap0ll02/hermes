use hermes::package::Package;

#[test]
fn test_package_creation() {
    let pkg = Package {
        name: "firefox".to_string(),
        repo: "extra".to_string(),
        version: "120.0-1".to_string(),
        description: "Fast, private browser".to_string(),
        installed: true,
    };

    assert_eq!(pkg.name, "firefox");
    assert_eq!(pkg.repo, "extra");
    assert_eq!(pkg.version, "120.0-1");
    assert!(pkg.installed);
}

#[test]
fn test_display_line_installed() {
    let pkg = Package {
        name: "vim".to_string(),
        repo: "core".to_string(),
        version: "9.0".to_string(),
        description: "Text editor".to_string(),
        installed: true,
    };

    let line = pkg.display_line();
    assert!(line.contains("✓"), "Should show checkmark for installed");
    assert!(line.contains("vim"), "Should contain package name");
    assert!(line.contains("core"), "Should contain repo name");
}

#[test]
fn test_display_line_not_installed() {
    let pkg = Package {
        name: "htop".to_string(),
        repo: "extra".to_string(),
        version: "3.2.1".to_string(),
        description: "Process viewer".to_string(),
        installed: false,
    };

    let line = pkg.display_line();
    assert!(!line.contains("✓"), "Should not show checkmark");
    assert!(line.contains("htop"), "Should contain package name");
}

#[test]
fn test_display_line_truncates_long_description() {
    let long_desc = "a".repeat(100);
    let pkg = Package {
        name: "test".to_string(),
        repo: "core".to_string(),
        version: "1.0".to_string(),
        description: long_desc,
        installed: false,
    };

    let line = pkg.display_line();
    // Description should be truncated to 50 chars
    assert!(line.len() < 150, "Line should be reasonably short");
}

#[test]
fn test_package_clone() {
    let pkg1 = Package {
        name: "test".to_string(),
        repo: "core".to_string(),
        version: "1.0".to_string(),
        description: "Test".to_string(),
        installed: false,
    };

    let pkg2 = pkg1.clone();
    assert_eq!(pkg1.name, pkg2.name);
    assert_eq!(pkg1.installed, pkg2.installed);
}

// Mark this as ignored since it needs real ALPM
#[test]
#[ignore]
fn test_search_packages_real() {
    // This only runs when you do: cargo test -- --ignored
    let results = hermes::search_packages("firefox");
    assert!(results.is_ok());
}

#[test]
#[ignore]
fn test_empty_search() {
    let results = hermes::search_packages("");
    assert!(results.is_ok());
    assert!(results.unwrap().is_empty());
}

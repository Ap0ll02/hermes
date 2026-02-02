use hermes::package::Package;

#[test]
fn test_package_display_line() {
    let pkg = Package {
        name: "firefox".to_string(),
        repo: "extra".to_string(),
        version: "120.0.1".to_string(),
        description: "A web browser".to_string(),
        installed: true,
    };

    let line = pkg.display_line();
    assert!(line.contains("firefox"));
    assert!(line.contains("✓")); // Installed marker
}

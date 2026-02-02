use hermes::ui::{App, InputMode, ConfirmAction};
use hermes::package::Package;

#[test]
fn test_app_initialization() {
    let app = App::new();
    
    assert_eq!(app.selected, 0);
    assert!(app.packages.is_empty());
    assert_eq!(app.search_query, "");
    assert!(!app.show_help);
    assert!(app.confirm_msg.is_none());
    assert!(app.status_msg.is_none());
}

#[test]
fn test_app_mode_default() {
    let app = App::new();
    assert!(matches!(app.mode, InputMode::Search));
}

#[test]
fn test_navigation_next() {
    let mut app = App::new();
    
    // Add test packages
    app.packages = vec![
        Package {
            name: "pkg1".to_string(),
            repo: "core".to_string(),
            version: "1.0".to_string(),
            description: "First".to_string(),
            installed: false,
        },
        Package {
            name: "pkg2".to_string(),
            repo: "core".to_string(),
            version: "1.0".to_string(),
            description: "Second".to_string(),
            installed: false,
        },
        Package {
            name: "pkg3".to_string(),
            repo: "core".to_string(),
            version: "1.0".to_string(),
            description: "Third".to_string(),
            installed: false,
        },
    ];
    
    assert_eq!(app.selected, 0);
    
    app.next();
    assert_eq!(app.selected, 1);
    
    app.next();
    assert_eq!(app.selected, 2);
    
    app.next(); // Should wrap to 0
    assert_eq!(app.selected, 0);
}

#[test]
fn test_navigation_previous() {
    let mut app = App::new();
    
    app.packages = vec![
        Package {
            name: "pkg1".to_string(),
            repo: "core".to_string(),
            version: "1.0".to_string(),
            description: "First".to_string(),
            installed: false,
        },
        Package {
            name: "pkg2".to_string(),
            repo: "core".to_string(),
            version: "1.0".to_string(),
            description: "Second".to_string(),
            installed: false,
        },
    ];
    
    assert_eq!(app.selected, 0);
    
    app.previous(); // Should wrap to last
    assert_eq!(app.selected, 1);
    
    app.previous();
    assert_eq!(app.selected, 0);
}

#[test]
fn test_navigation_empty_packages() {
    let mut app = App::new();
    
    // Should not panic with empty package list
    app.next();
    assert_eq!(app.selected, 0);
    
    app.previous();
    assert_eq!(app.selected, 0);
}

#[test]
fn test_navigation_single_package() {
    let mut app = App::new();
    
    app.packages = vec![
        Package {
            name: "solo".to_string(),
            repo: "core".to_string(),
            version: "1.0".to_string(),
            description: "Only one".to_string(),
            installed: false,
        },
    ];
    
    app.next();
    assert_eq!(app.selected, 0); // Should stay at 0
    
    app.previous();
    assert_eq!(app.selected, 0); // Should stay at 0
}

#[test]
fn test_confirm_action_install() {
    let action = ConfirmAction::Install("firefox".to_string());
    
    match action {
        ConfirmAction::Install(pkg) => assert_eq!(pkg, "firefox"),
        _ => panic!("Wrong variant"),
    }
}

#[test]
fn test_confirm_action_remove() {
    let action = ConfirmAction::Remove("vim".to_string());
    
    match action {
        ConfirmAction::Remove(pkg) => assert_eq!(pkg, "vim"),
        _ => panic!("Wrong variant"),
    }
}

#[test]
fn test_search_query_modification() {
    let mut app = App::new();
    
    app.search_query = "fire".to_string();
    assert_eq!(app.search_query, "fire");
    
    app.search_query.push('f');
    assert_eq!(app.search_query, "firef");
    
    app.search_query.pop();
    assert_eq!(app.search_query, "fire");
}

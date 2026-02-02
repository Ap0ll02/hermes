use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Package {
    pub name: String,
    pub repo: String,
    pub version: String,
    pub description: String,
    pub installed: bool,
}

impl Package {
    pub fn display_line(&self) -> String {
        let status = if self.installed { "✓" } else { " " };
        format!(
            "{} {:<8} {:<30} {}",
            status,
            self.repo,
            self.name,
            self.description.chars().take(50).collect::<String>()
        )
    }
}

pub fn search_packages(query: &str) -> Result<Vec<Package>, String> {
    use alpm::{Alpm, SigLevel};

    if query.is_empty() {
        return Ok(Vec::new());
    }

    // eprintln!("=== Searching for: '{}' ===", query);

    // Try with explicit config
    let alpm = Alpm::new("/", "/var/lib/pacman")
        .map_err(|e| format!("Failed to initialize ALPM: {}", e))?;

    // Manually register sync databases
    // let repos = vec!["core", "extra", "community", "multilib"];
    let sync_dir = "/var/lib/pacman/sync";
    let mut repos = Vec::new();

    if let Ok(entries) = fs::read_dir(sync_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(name) = path.file_name()
                && let Some(name_str) = name.to_str()
                    && let Some(repo) = repo_from_filename(name_str) {
                        repos.push(repo);
                    }
        }
    }
    // Fallback if no repos found
    if repos.is_empty() {
        repos = vec![
            "core".to_string(),
            "extra".to_string(),
            "community".to_string(),
            "multilib".to_string(),
        ];
    }

    for repo_name in repos {
        let _ = alpm.register_syncdb(repo_name, SigLevel::USE_DEFAULT);
    }

    let mut results = Vec::new();
    let local_db = alpm.localdb();

    // Search in sync databases
    for db in alpm.syncdbs() {
        let db_name = db.name();

        for pkg in db.pkgs() {
            let name = pkg.name();
            let desc = pkg.desc().unwrap_or("");

            if match_query(name, desc, query) {
                let installed = local_db.pkg(name).is_ok();

                results.push(Package {
                    name: name.to_string(),
                    repo: db_name.to_string(),
                    version: pkg.version().to_string(),
                    description: desc.to_string(),
                    installed,
                });
            }
        }
    }
    Ok(results)
}

fn match_query(name: &str, desc: &str, query: &str) -> bool {
    if query.is_empty() {
        return false;
    }

    let q = query.to_lowercase();
    name.to_lowercase().contains(&q) || desc.to_lowercase().contains(&q)
}

fn repo_from_filename(filename: &str) -> Option<String> {
    if filename.ends_with(".db") && !filename.ends_with(".db.sig") {
        Some(filename.trim_end_matches(".db").to_string())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repo_from_filename_db() {
        let repo = repo_from_filename("extra.db");
        assert_eq!(repo, Some("extra".to_string()));
    }

    #[test]
    fn test_repo_from_filename_db_sig() {
        let repo = repo_from_filename("extra.db.sig");
        assert_eq!(repo, None);
    }

    #[test]
    fn test_repo_from_filename_non_db() {
        let repo = repo_from_filename("extra.txt");
        assert_eq!(repo, None);
    }

    #[test]
    fn matches_query_matches_name() {
        assert!(match_query("vim", "text editor", "vi"));
    }

    #[test]
    fn matches_query_matches_desc() {
        assert!(match_query("helix", "rust text editor", "editor"));
    }

    #[test]
    fn matches_name_caps_sens() {
        assert!(match_query("NANO", "small editor", "nan"));
    }

    #[test]
    fn match_false_query_doesnt_match() {
        assert!(!match_query("helix", "rust text editor", "vim"));
    }

    #[test]
    fn match_empty_query_doesnt_match() {
        assert!(!match_query("helix", "rust text editor", ""));
    }

    #[test]
    fn test_package_creation() {
        let pkg = Package {
            name: "test".to_string(),
            repo: "core".to_string(),
            version: "1.0.0".to_string(),
            description: "Test package".to_string(),
            installed: false,
        };

        assert_eq!(pkg.name, "test");
        assert!(!pkg.installed);
    }

    #[test]
    fn test_display_line_format() {
        let pkg = Package {
            name: "vim".to_string(),
            repo: "extra".to_string(),
            version: "9.0".to_string(),
            description: "Text editor".to_string(),
            installed: false,
        };

        let line = pkg.display_line();
        assert!(line.contains("vim"));
        assert!(line.contains("extra"));
    }

    #[test]
    fn test_truncate_display_line() {
        let pkg = Package {
            name: "Helix".to_string(),
            repo: "extra".to_string(),
            version: "95.9".to_string(),
            description: "Mushroom editor".to_string(),
            installed: false,
        };

        let line = pkg.display_line();
        let after_status = &line[2..];
        let repo_col = &after_status[..8];
        assert_eq!(repo_col, "extra   ")
    }

    #[test]
    fn test_desc_truncate() {
        let pkg = Package {
            name: "Helix".to_string(),
            repo: "extra".to_string(),
            version: "95.9".to_string(),
            description: "This is a really long description in rust and I need to ensure only the first 50 characters are taken this is so great haha".to_string(),
            installed: false,
        };

        let longest_desc = "This is a really long description in rust and I need to ensure only the first 50 characters are taken this is so great haha";
        let long_desc = longest_desc.chars().take(50).collect::<String>();
        let line = pkg.display_line();
        assert!(line.contains(&long_desc));
        assert!(!line.contains(longest_desc));
    }

    #[test]
    fn display_line_shows_installed_and_truncates_description() {
        let pkg = Package {
            name: "vim".to_string(),
            repo: "extra".to_string(),
            version: "9.0".to_string(),
            description: "a".repeat(100), // force truncation
            installed: true,
        };

        let line = pkg.display_line();

        // Installed marker
        assert!(line.starts_with("✓"));

        // Description should be truncated to 50 chars
        let desc_part = line.split_whitespace().last().unwrap();
        assert!(desc_part.len() <= 50);
    }

    #[test]
    fn display_line_not_installed_has_blank_marker() {
        let pkg = Package {
            name: "nano".to_string(),
            repo: "core".to_string(),
            version: "7.0".to_string(),
            description: "Editor".to_string(),
            installed: false,
        };

        let line = pkg.display_line();

        assert!(line.starts_with(" "));
        assert!(!line.starts_with("✓"));
    }

    #[test]
    fn test_package_struct_fields() {
        let pkg = Package {
            name: "test".to_string(),
            repo: "core".to_string(),
            version: "1.0.0".to_string(),
            description: "A test package".to_string(),
            installed: true,
        };

        assert_eq!(pkg.name, "test");
        assert_eq!(pkg.repo, "core");
        assert_eq!(pkg.version, "1.0.0");
        assert_eq!(pkg.description, "A test package");
        assert!(pkg.installed);
    }

    #[test]
    fn test_package_not_installed_default() {
        let pkg = Package {
            name: "test".to_string(),
            repo: "core".to_string(),
            version: "1.0.0".to_string(),
            description: "Test".to_string(),
            installed: false,
        };

        assert!(!pkg.installed);
    }
}

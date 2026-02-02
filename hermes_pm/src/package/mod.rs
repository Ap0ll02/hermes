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
            if let Some(name) = path.file_name() {
                // eprintln!("Found DB: {}", name.to_str().unwrap());
                let name_str = name.to_str();
                if name_str.unwrap().ends_with(".db") && !name_str.unwrap().ends_with(".db.sig") {
                    let repo_entry = name_str.unwrap().trim_end_matches(".db").to_string();
                    repos.push(repo_entry);
                }
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

            if name.to_lowercase().contains(&query.to_lowercase())
                || desc.to_lowercase().contains(&query.to_lowercase())
            {
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

    // eprintln!("=== Total results: {} ===\n", results.len());
    Ok(results)
}

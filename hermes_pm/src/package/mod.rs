use serde::{Deserialize, Serialize};

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
        .map_err(|e| {
            // eprintln!("❌ ALPM init error: {}", e);
            format!("Failed to initialize ALPM: {}", e)
        })?;
    
    // eprintln!("✓ ALPM initialized");
    
    // Manually register sync databases
    let repos = vec!["core", "extra", "community", "multilib"];
    
    for repo_name in repos {
        match alpm.register_syncdb(repo_name, SigLevel::USE_DEFAULT) {
            Ok(_db) => {
                // eprintln!("  ✓ Registered: {} ({} packages)", repo_name, db.pkgs().into_iter().count());
            }
            Err(_e) => {
                // eprintln!("  ❌ Failed to register {}: {}", repo_name, e);
            }
        }
    }
    
    let mut results = Vec::new();
    let local_db = alpm.localdb();
    
    // Search in sync databases
    for db in alpm.syncdbs() {
        let db_name = db.name();
        
        for pkg in db.pkgs() {
            let name = pkg.name();
            let desc = pkg.desc().unwrap_or("");
            
            if name.to_lowercase().contains(&query.to_lowercase()) ||
               desc.to_lowercase().contains(&query.to_lowercase()) {
                
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

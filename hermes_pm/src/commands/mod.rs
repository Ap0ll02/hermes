use std::process::Command;

pub fn install_package(package_name: &str) -> Result<String, String> {
    let status = Command::new("pacman")
        .args(["-S", package_name])
        .status()
        .map_err(|e| format!("Error Installing: {e}"))?;

    if status.success() {
        Ok(format!("Successfully Installed {package_name}"))
    } else {
        Err(format!("Failed to install {}", package_name))
    }
}

pub fn remove_package(package_name: &str) -> Result<String, String> {
    let output = Command::new("pacman")
        .args(["-R", package_name])
        .output()
        .map_err(|e| format!("Error removing: {e}"))?;

    if output.status.success() {
        Ok(format!("Successfully Removed {package_name}"))
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(format!("Failed to remove {}: {}", package_name, stderr))
    }
}

pub fn update_packages() -> Result<String, String> {
    let output = Command::new("pacman")
        .args(["-Syu"])
        .output()
        .map_err(|e| format!("Error Updating: {e}"))?;

    if output.status.success() {
        Ok(format!("Successfully Updated"))
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(format!("Failed to update packages: {}", stderr))
    }
}

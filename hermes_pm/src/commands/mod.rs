use std::process::Command;

pub fn install_package(package_name: &str) -> Result<String, String> {
    let status = Command::new("pacman")
        .args(["-S", package_name])
        .status()
        .map_err(|e| format!("Error Installing: {e}"))?;
    status_to_message(
        status.success(),
        format!("Successfully installed {package_name}"),
        format!("Failed to install {package_name}"),
    )
}

pub fn remove_package(package_name: &str) -> Result<String, String> {
    let status = Command::new("pacman")
        .args(["-R", package_name])
        .status()
        .map_err(|e| format!("Error removing: {e}"))?;

    status_to_message(
        status.success(),
        format!("Successfully removed {package_name}"),
        format!("Failed to remove {package_name}"),
    )
}

pub fn update_packages() -> Result<String, String> {
    let status = Command::new("pacman")
        .args(["-Syu"])
        .status()
        .map_err(|e| format!("Error Updating: {e}"))?;

    status_to_message(
        status.success(),
        "Successfully Updated Packages".to_string(),
        "Failed to update packages".to_string(),
    )
}

pub fn downgrade_package(package_name: &str) -> Result<String, String> {
    let status = Command::new("downgrade")
        .args([package_name])
        .status()
        .map_err(|e| format!("Error removing: {e}"))?;

    status_to_message(
        status.success(),
        format!("Successfully Downgraded {package_name}"),
        format!("Failed to downgrade {package_name}"),
    )
}

fn status_to_message(success: bool, suc_msg: String, fail_msg: String) -> Result<String, String> {
    if success { Ok(suc_msg) } else { Err(fail_msg) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn status_msg_returns_ok() {
        let result = status_to_message(true, "Success".to_string(), "Failure".to_string());

        assert_eq!(result, Ok("Success".to_string()));
    }
    #[test]
    fn status_msg_returns_err() {
        let result = status_to_message(false, "Success".to_string(), "Failure".to_string());

        assert_eq!(result, Err("Failure".to_string()));
    }
}

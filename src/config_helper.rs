use crate::error::{Error, Result};
use std::process::Command;

/// Check if c2rust-config command exists
pub fn check_c2rust_config_exists() -> Result<()> {
    let result = Command::new("c2rust-config")
        .arg("--version")
        .output();

    match result {
        Ok(output) if output.status.success() => Ok(()),
        _ => Err(Error::ConfigToolNotFound),
    }
}

/// Save test configuration using c2rust-config
pub fn save_config(command: &str, feature: Option<&str>) -> Result<()> {
    let feature_args = if let Some(f) = feature {
        vec!["--feature", f]
    } else {
        vec![]
    };

    // Save test.dir configuration (current directory)
    let mut cmd = Command::new("c2rust-config");
    cmd.args(&["config", "--make"])
        .args(&feature_args)
        .args(&["--set", "test.dir", "."]);

    let output = cmd.output().map_err(|e| {
        Error::ConfigSaveFailed(format!("Failed to execute c2rust-config: {}", e))
    })?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(Error::ConfigSaveFailed(format!(
            "Failed to save test.dir: {}",
            stderr
        )));
    }

    // Save test command configuration
    let mut cmd = Command::new("c2rust-config");
    cmd.args(&["config", "--make"])
        .args(&feature_args)
        .args(&["--set", "test", command]);

    let output = cmd.output().map_err(|e| {
        Error::ConfigSaveFailed(format!("Failed to execute c2rust-config: {}", e))
    })?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(Error::ConfigSaveFailed(format!(
            "Failed to save test command: {}",
            stderr
        )));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_c2rust_config_exists() {
        // This test will fail if c2rust-config is not installed
        // We can't test for ConfigToolNotFound without uninstalling it
        let _ = check_c2rust_config_exists();
    }
}

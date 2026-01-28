use crate::error::{Error, Result};
use std::path::Path;
use std::process::{Command, Stdio};

/// Execute a command in the specified directory with real-time output
pub fn execute_command(dir: &Path, command: &[String]) -> Result<()> {
    if command.is_empty() {
        return Err(Error::CommandExecutionFailed(
            "No command provided".to_string(),
            None,
        ));
    }

    let program = &command[0];
    let args = &command[1..];

    // Print the command being executed
    println!("Executing command: {} {}", program, args.join(" "));
    println!("In directory: {}", dir.display());
    println!();

    // Spawn the command with inherited stdout/stderr for real-time output
    let mut child = Command::new(program)
        .args(args)
        .current_dir(dir)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .map_err(|e| {
            Error::CommandExecutionFailed(
                format!(
                    "Failed to execute command '{} {}': {}",
                    program,
                    args.join(" "),
                    e
                ),
                None,
            )
        })?;

    // Wait for the command to complete
    let status = child.wait().map_err(|e| {
        Error::CommandExecutionFailed(
            format!(
                "Failed to wait for command '{} {}': {}",
                program,
                args.join(" "),
                e
            ),
            None,
        )
    })?;

    // Print exit status
    println!();
    if let Some(code) = status.code() {
        println!("Exit code: {}", code);
    } else {
        println!("Process terminated by signal");
    }
    println!();

    if !status.success() {
        let error_msg = if let Some(code) = status.code() {
            format!(
                "Command '{} {}' failed with exit code {}",
                program,
                args.join(" "),
                code,
            )
        } else {
            format!(
                "Command '{} {}' was terminated by signal",
                program,
                args.join(" "),
            )
        };
        
        return Err(Error::CommandExecutionFailed(error_msg, status.code()));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute_command_empty() {
        let result = execute_command(Path::new("."), &[]);
        assert!(result.is_err());
    }

    #[test]
    fn test_execute_command_basic() {
        // Test with a simple command that should succeed
        let result = execute_command(Path::new("."), &["echo".to_string(), "test".to_string()]);
        assert!(result.is_ok());
    }
}

mod config_helper;
mod error;
mod executor;
mod git_helper;

use clap::{Args, Parser, Subcommand};
use error::Result;
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[command(name = "c2rust-test")]
#[command(about = "C project test execution tool for c2rust")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Execute test command
    Test(CommandArgs),
}

#[derive(Args)]
struct CommandArgs {
    /// Optional feature name (default: "default")
    #[arg(long)]
    feature: Option<String>,

    /// Test command to execute - use after '--' separator
    /// Example: c2rust-test test -- make test
    #[arg(trailing_var_arg = true, allow_hyphen_values = true, required = true, value_name = "TEST_CMD")]
    test_cmd: Vec<String>,
}

fn run(args: CommandArgs) -> Result<()> {
    // 1. Check if c2rust-config exists
    config_helper::check_c2rust_config_exists()?;

    // 2. Get feature name (default to "default")
    let feature = args.feature.as_deref().unwrap_or("default");

    // 3. Get the current working directory (where the command is executed)
    let current_dir = std::env::current_dir()
        .map_err(|e| error::Error::IoError(e))?;
    
    // 4. Find the project root (where .c2rust will be created)
    // Start from current directory and search upward for .c2rust or use current as root
    let project_root = find_project_root(&current_dir)?;
    
    // 5. Calculate the test directory relative to project root
    // Note: If current_dir is not a descendant of project_root (which shouldn't happen
    // based on find_project_root logic), we fall back to "." as a safe default.
    let test_dir_relative = current_dir.strip_prefix(&project_root)
        .map(|p| {
            if p.as_os_str().is_empty() {
                ".".to_string()
            } else {
                p.display().to_string()
            }
        })
        .unwrap_or_else(|_| {
            eprintln!("Warning: current directory is not under project root, using '.' as test directory");
            ".".to_string()
        });

    println!("=== c2rust-test ===");
    println!("Project root: {}", project_root.display());
    println!("Test directory (relative): {}", test_dir_relative);
    println!("Feature: {}", feature);
    println!("Command: {}", args.test_cmd.join(" "));
    println!();
    
    // 6. Execute the test command in the current directory
    executor::execute_command(&current_dir, &args.test_cmd)?;

    println!("Test command executed successfully.");
    
    // 7. Save configuration using c2rust-config
    let command_str = args.test_cmd.join(" ");
    config_helper::save_config(&test_dir_relative, &command_str, Some(feature), &project_root)?;
    println!("✓ Configuration saved.");

    // Auto-commit changes in .c2rust directory if any
    git_helper::auto_commit_if_modified(&project_root)?;

    Ok(())
}

/// Find the project root directory.
/// First checks C2RUST_PROJECT_ROOT environment variable.
/// If not set, searches for .c2rust directory upward from start_dir.
/// If not found, returns the start_dir as root.
fn find_project_root(start_dir: &Path) -> Result<PathBuf> {
    // Check if C2RUST_PROJECT_ROOT environment variable is set
    // If set, it IS the project root (set by upstream tools), so use it directly
    // No validation is performed - the upstream tool is trusted to provide a valid path
    if let Ok(project_root) = std::env::var("C2RUST_PROJECT_ROOT") {
        return Ok(PathBuf::from(project_root));
    }
    
    // If not set, search for .c2rust directory
    let mut current = start_dir;
    loop {
        let c2rust_dir = current.join(".c2rust");
        if c2rust_dir.exists() && c2rust_dir.is_dir() {
            return Ok(current.to_path_buf());
        }
        match current.parent() {
            Some(parent) => current = parent,
            None => return Ok(start_dir.to_path_buf()),
        }
    }
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Test(args) => run(args),
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(e.exit_code());
    }
}

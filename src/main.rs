mod error;
mod executor;

use clap::{Args, Parser, Subcommand};
use error::Result;

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
    /// Test command to execute - use after '--' separator
    /// Example: c2rust-test test -- make test
    #[arg(trailing_var_arg = true, allow_hyphen_values = true, required = true, value_name = "TEST_CMD")]
    test_cmd: Vec<String>,
}

fn run(args: CommandArgs) -> Result<()> {
    // Get the current working directory (where the command is executed)
    let current_dir = std::env::current_dir()
        .map_err(|e| error::Error::IoError(e))?;
    
    let test_dir = current_dir.to_str()
        .ok_or_else(|| error::Error::IoError(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Current directory path contains invalid UTF-8"
        )))?;
    
    // Execute the test command in the current directory
    executor::execute_command(test_dir, &args.test_cmd)?;

    println!("Test command executed successfully.");
    Ok(())
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

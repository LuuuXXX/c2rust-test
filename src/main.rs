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
    /// Directory to execute test command (required)
    #[arg(long, required = true)]
    dir: String,

    /// Test command to execute (e.g., "make test") (required)
    #[arg(last = true, required = true)]
    command: Vec<String>,
}

fn run(args: CommandArgs) -> Result<()> {
    // Validate that command is not empty
    if args.command.is_empty() {
        return Err(error::Error::MissingParameter(
            "Command not specified. Provide command arguments after '--'".to_string(),
        ));
    }

    // Execute the test command
    executor::execute_command(&args.dir, &args.command)?;

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
        std::process::exit(1);
    }
}

mod config_helper;
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
    /// Execute test command and save configuration
    Test(CommandArgs),
}

#[derive(Args)]
struct CommandArgs {
    /// Feature name (default: "default")
    #[arg(long)]
    feature: Option<String>,

    /// Test command to execute (e.g., "make test")
    #[arg(last = true, required = true)]
    command: Vec<String>,
}

fn run(args: CommandArgs) -> Result<()> {
    // 1. Check if c2rust-config exists
    config_helper::check_c2rust_config_exists()?;

    // 2. Execute the test command
    executor::execute_command(&args.command)?;

    // 3. Save configuration using c2rust-config
    let command_str = args.command.join(" ");
    config_helper::save_config(&command_str, args.feature.as_deref())?;

    println!("Test command executed successfully and configuration saved.");
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

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
    #[arg(long = "test.dir", required = true)]
    test_dir: String,

    /// Test command to execute (required, can be multiple arguments)
    #[arg(long = "test.cmd", required = true, num_args = 1.., allow_hyphen_values = true)]
    test_cmd: Vec<String>,
}

fn run(args: CommandArgs) -> Result<()> {
    // Execute the test command
    executor::execute_command(&args.test_dir, &args.test_cmd)?;

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

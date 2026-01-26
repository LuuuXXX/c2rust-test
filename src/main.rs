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
    /// Directory to execute test command
    #[arg(long)]
    dir: Option<String>,

    /// Feature name (default: "default")
    #[arg(long)]
    feature: Option<String>,

    /// Test command to execute (e.g., "make test")
    #[arg(last = true)]
    command: Vec<String>,
}

fn run(args: CommandArgs) -> Result<()> {
    // 1. Check if c2rust-config exists
    config_helper::check_c2rust_config_exists()?;

    // 2. Read configuration from file
    let config = config_helper::read_config(args.feature.as_deref())?;

    // 3. Determine final values (CLI overrides config)
    let dir = args.dir.or(config.dir).ok_or_else(|| {
        error::Error::MissingParameter(
            "Directory not specified. Use --dir or set test.dir in config".to_string(),
        )
    })?;

    // NOTE: `command` is populated by clap with an empty Vec when no arguments are
    // provided after `--` (due to `#[arg(last = true)]`). We intentionally treat
    // an empty Vec here as "no CLI command provided" and fall back to config.
    let command = if !args.command.is_empty() {
        args.command
    } else if let Some(cmd_str) = config.command {
        // Parse command string into Vec<String>
        // Note: This uses simple whitespace splitting and doesn't handle quoted arguments.
        // For commands with quoted arguments, specify them directly on the CLI.
        let parsed_command: Vec<String> = cmd_str
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        if parsed_command.is_empty() {
            return Err(error::Error::MissingParameter(
                "Command in config is empty or whitespace-only. Provide command arguments after '--' or set a non-empty 'test.cmd' value in the config file.".to_string(),
            ));
        }

        parsed_command
    } else {
        return Err(error::Error::MissingParameter(
            "Command not specified. Provide command arguments after '--' or set 'test.cmd' (command) in config file".to_string(),
        ));
    };

    // 4. Execute the test command
    executor::execute_command(&dir, &command)?;

    // 5. Save configuration using c2rust-config
    let command_str = command.join(" ");
    config_helper::save_config(&dir, &command_str, args.feature.as_deref())?;

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

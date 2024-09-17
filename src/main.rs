mod cli;
mod commit;
mod config;

use structopt::StructOpt;
use std::process;

fn main() {
    let cli = cli::Cli::from_args();

    match cli.command {
        cli::Command::Init => {
            let prefixes = cli::prompt_prefixes();
            if let Err(e) = config::save_prefixes(&prefixes) {
                eprintln!("Failed to save prefixes: {}", e);
                process::exit(1);
            }
            println!("Prefixes saved successfully!");
        }
        cli::Command::Commit => {
            let config = config::load_config().unwrap_or_default();
            
            let commit_type = cli::prompt_commit_type(&config.prefixes);
            let description = cli::prompt_description();
            
            let commit_message = commit::generate_commit_message(commit_type, description);
            
            println!("\nGenerated commit message:");
            println!("{}", commit_message);
        }
    }
}
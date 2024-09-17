mod cli;
mod commit;
mod config;

use structopt::StructOpt;
use std::process;
use git2::{Repository, Signature};

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
        cli::Command::Commit { create_commit } => {
            let config = config::load_config().unwrap_or_default();
            
            let commit_type = cli::prompt_commit_type(&config.prefixes);
            let description = cli::prompt_description();
            
            let commit_message = commit::generate_commit_message(commit_type, description);
            
            if create_commit {
                match create_git_commit(&commit_message) {
                    Ok(()) => println!("Commit created successfully: {}", commit_message),
                    Err(e) => eprintln!("Failed to create commit: {}", e),
                }
            } else {
                println!("Generated commit message: {}", commit_message);
                println!("To create a Git commit with this message, run this commmand with the --create-commit flag:");
            }
        }
        cli::Command::Help => {
            println!("Usage: commitier <command>");
            println!("Commands:");
            println!("  init - Initialize the commitier configuration");
            println!("  commit - Generate a commit message");
            println!("  help - Show this help message");
            println!("  check-commits - Check the last n commits with --count n");
        }
        cli::Command::CheckCommits { count } => {
            match check_commits(count) {
                Ok(()) => (),
                Err(e) => eprintln!("Failed to check commits: {}", e),
            }
        }
    }
}

fn create_git_commit(message: &str) -> Result<(), git2::Error> {
    let repo = Repository::open_from_env()?;
    let config = repo.config()?;

    let name = config.get_string("user.name").unwrap_or_else(|_| "Unknown".to_string());
    let email = config.get_string("user.email").unwrap_or_else(|_| "unknown@example.com".to_string());

    let signature = Signature::now(&name, &email)?;
    
    let mut index = repo.index()?;
    let oid = index.write_tree()?;
    let tree = repo.find_tree(oid)?;
    
    let parent_commit = repo.head()?.peel_to_commit()?;
    
    repo.commit(Some("HEAD"), &signature, &signature, message, &tree, &[&parent_commit])?;
    println!("Commit created with signature: {} <{}>", name, email);

    
    Ok(())
}

fn check_commits(count: u32) -> Result<(), git2::Error> {
    let repo = Repository::open_from_env()?;
    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;
    
    println!("Last {} commits:", count);
    for (index, oid) in revwalk.take(count as usize).enumerate() {
        let oid = oid?;
        let commit = repo.find_commit(oid)?;
        println!("{}. {} - {}", 
            index + 1, 
            commit.id().to_string().get(..7).unwrap_or(""),
            commit.summary().unwrap_or("No summary")
        );
    }
    
    Ok(())
}
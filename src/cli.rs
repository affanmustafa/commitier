use structopt::StructOpt;
use std::io::{self, Write};

#[derive(Debug, StructOpt)]
pub struct Cli {
    #[structopt(subcommand)]
    pub command: Command,
}

#[derive(Debug, StructOpt)]
pub enum Command {
    Init,
    Commit {
        #[structopt(long, help = "Create a Git commit")]
        create_commit: bool,
    },
}

pub fn prompt_prefixes() -> Vec<String> {
    let mut prefixes = Vec::new();
    println!("Enter prefixes (type 'commitier-end' when done):");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();
        if input == "commitier-end" {
            break;
        }
        prefixes.push(input.to_string());
    }

    prefixes
}

pub fn prompt_commit_type(prefixes: &[String]) -> String {
    println!("Select the type of change you're committing:");
    for (i, p) in prefixes.iter().enumerate() {
        println!("{}. {}", i + 1, p);
    }

    loop {
        print!("Enter the number of your selection: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if let Ok(num) = input.trim().parse::<usize>() {
            if num > 0 && num <= prefixes.len() {
                return prefixes[num - 1].clone();
            }
        }

        println!("Invalid selection. Please try again.");
    }
}

pub fn prompt_description() -> String {
    print!("Enter a short description: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input.trim().to_string()
}
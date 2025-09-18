use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {


    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand,Debug)]
enum Commands {
    AddMode,
    QuizMode
}

fn main() {
    let cli = Cli::parse();
 
  
    match &cli.command {
        Some(Commands::AddMode) => {
            
                println!("Not printing testing lists...");
                    println!("Add");

            
        }
        Some(Commands::QuizMode) => {
            
                println!("Not printing testing lists...");
            
        }
        None => {

        }
    }

    // Continued program logic goes here...
}
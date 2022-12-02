mod days;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    DayOne {
        #[arg(required = true)]
        path: PathBuf,
    },
}

fn main() {
    let args = Args::parse();

    match &args.command {
        Commands::DayOne { path } => {
            println!("{}", days::day_one::run(path));
        }
    }
}

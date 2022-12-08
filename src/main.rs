mod days;
pub mod utils;
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
        #[arg(short, long)]
        bonus: bool,
    },
    DayTwo {
        #[arg(required = true)]
        path: PathBuf,
        #[arg(short, long)]
        bonus: bool,
    },
    DayThree {
        #[arg(required = true)]
        path: PathBuf,
        #[arg(short, long)]
        bonus: bool,
    },
    DayFour {
        #[arg(required = true)]
        path: PathBuf,
        #[arg(short, long)]
        bonus: bool,
    },
    DayFive {
        #[arg(required = true)]
        path: PathBuf,
        #[arg(short, long)]
        bonus: bool,
    },

}

fn main() {
    let args = Args::parse();

    match &args.command {
        Commands::DayOne { path, bonus } => {
            println!("{}", days::day_one::run(path, bonus.to_owned()));
        }
        Commands::DayTwo { path, bonus } => {
            println!("{}", days::day_two::run(path, bonus.to_owned()))
        }
        Commands::DayThree { path, bonus } => {
            println!("{}", days::day_three::run(path, bonus.to_owned()))
        }
        Commands::DayFour { path, bonus } => println!("{}", days::day_four::run(path, bonus.to_owned())),
        Commands::DayFive { path, bonus } => println!("{}", days::day_five::run(path, bonus.to_owned()))
    }
}

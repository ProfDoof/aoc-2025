use std::path::PathBuf;

use clap::{Parser, Subcommand};

/// The commandline tool to run all of my advent of code solutions
#[derive(Parser, Clone, Debug)]
struct Cli {
    #[arg(short, long, default_value = "inputs")]
    inputs: PathBuf,
    
    #[command(subcommand)]
    day:Day
}

/// The days of advent of code
#[derive(Subcommand, Clone, Debug)]
enum Day {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Eleven,
    Twelve,
    Thirteen,
    Fourteen,
    Fifteen,
    Sixteen,
    Seventeen,
    Eighteen,
    Nineteen,
    Twenty,
    TwentyOne,
    TwentyTwo,
    TwentyThree,
    TwentyFour,
}

fn main() {
    let args = Cli::parse();
    println!("{:?}", args);
}

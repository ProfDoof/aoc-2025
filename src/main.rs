mod days;

use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, Subcommand};

/// The commandline tool to run all of my advent of code solutions
#[derive(Parser, Clone, Debug)]
struct Cli {
    #[arg(short, long, default_value = "inputs")]
    inputs: PathBuf,
    
    #[command(subcommand)]
    day:AdventDay
}

/// The days of advent of code
#[derive(Subcommand, Clone, Debug)]
enum AdventDay {
    #[command(name = "1")]
    One,
    #[command(name = "2")]
    Two,
    #[command(name = "3")]
    Three,
    #[command(name = "4")]
    Four,
    #[command(name = "5")]
    Five,
    #[command(name = "6")]
    Six,
    #[command(name = "7")]
    Seven,
    #[command(name = "8")]
    Eight,
    #[command(name = "9")]
    Nine,
    #[command(name = "10")]
    Ten,
    #[command(name = "11")]
    Eleven,
    #[command(name = "12")]
    Twelve,
    #[command(name = "13")]
    Thirteen,
    #[command(name = "14")]
    Fourteen,
    #[command(name = "15")]
    Fifteen,
    #[command(name = "16")]
    Sixteen,
    #[command(name = "17")]
    Seventeen,
    #[command(name = "18")]
    Eighteen,
    #[command(name = "19")]
    Nineteen,
    #[command(name = "20")]
    Twenty,
    #[command(name = "21")]
    TwentyOne,
    #[command(name = "22")]
    TwentyTwo,
    #[command(name = "23")]
    TwentyThree,
    #[command(name = "24")]
    TwentyFour,
}

impl AdventDay{
    fn day(&self) -> Box<dyn crate::days::Day> {
        match self {
            AdventDay::One => Box::new(crate::days::one::DayOne),
            AdventDay::Two => Box::new(crate::days::two::DayTwo),
            AdventDay::Three => Box::new(crate::days::three::DayThree),
            AdventDay::Four => todo!(),
            AdventDay::Five => todo!(),
            AdventDay::Six => todo!(),
            AdventDay::Seven => todo!(),
            AdventDay::Eight => todo!(),
            AdventDay::Nine => todo!(),
            AdventDay::Ten => todo!(),
            AdventDay::Eleven => todo!(),
            AdventDay::Twelve => todo!(),
            AdventDay::Thirteen => todo!(),
            AdventDay::Fourteen => todo!(),
            AdventDay::Fifteen => todo!(),
            AdventDay::Sixteen => todo!(),
            AdventDay::Seventeen => todo!(),
            AdventDay::Eighteen => todo!(),
            AdventDay::Nineteen => todo!(),
            AdventDay::Twenty => todo!(),
            AdventDay::TwentyOne => todo!(),
            AdventDay::TwentyTwo => todo!(),
            AdventDay::TwentyThree => todo!(),
            AdventDay::TwentyFour => todo!(),
        }
    }
}

fn main() -> Result<()>{
    let args = Cli::parse();
    let day = args.day.day();
    day.run(args.inputs)
}

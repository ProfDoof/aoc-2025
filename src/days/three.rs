use anyhow::Context;
use nom::{bytes::complete::tag, character::complete::{anychar, u32}, combinator::map, multi::{many0, many_till}, sequence::{delimited, preceded, separated_pair, terminated}, IResult};

pub (crate)struct DayThree;

fn parse_mul(input: &str) -> IResult<&str, u32> {
    let (remaining, (a, b)) = preceded(tag("mul"), delimited(tag("("), separated_pair(u32, tag(","), u32), tag(")")))(input)?;
    Ok((remaining, a * b))
}

fn parse_day_one(input: &str) -> IResult<&str, Vec<u32>> {
    terminated(many0(map(many_till(anychar, parse_mul), |(_, multiplied)| multiplied)), many0(anychar))(input)
}

impl super::Day for DayThree {
    fn run(&self, inputs: std::path::PathBuf) -> anyhow::Result<()> {
        let input_path = inputs.join("day3.input");
        let input = std::fs::read_to_string(input_path)?;
        let (_, day_one_nums) = parse_day_one(&input).expect("Failed to parse day one");
        println!("Answer 1: {}", day_one_nums.iter().sum::<u32>());
        Ok(())
    }
}
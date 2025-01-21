

use nom::{
    character::complete::{line_ending, multispace0, u32}, combinator::{eof, opt}, multi::{many0, separated_list1}, sequence::terminated, IResult
};

pub(crate) struct DayTwo;

fn parse_line(input: &str) -> IResult<&str, Vec<u32>> {
    terminated(separated_list1(multispace0, u32), opt(line_ending))(input)
}

fn parse_lists(input: &str) -> IResult<&str, Vec<Vec<u32>>> {
    terminated(many0(parse_line), eof)(input)
}

enum PartOneState<'a> {
    Start,
    First(&'a u32),
    Increasing(&'a u32),
    Decreasing(&'a u32),
}

impl super::Day for DayTwo {
    fn run(&self, inputs: std::path::PathBuf) -> anyhow::Result<()> {
        let input_path = inputs.join("day2.input");
        let input = std::fs::read_to_string(input_path)?;
        // println!("input: {}", input);
        let (_, reports) = parse_lists(&input).unwrap();
        let answer_one = reports.iter().map(|report| {
            report.iter().try_fold(PartOneState::Start, |state, num| {
                match state {
                    PartOneState::Start => Ok(PartOneState::First(num)),
                    PartOneState::First(prev) => {
                        if num > prev {
                            Ok(PartOneState::Increasing(num))
                        } else if num < prev {
                            Ok(PartOneState::Decreasing(num))
                        } else {
                            Err(anyhow::anyhow!("Invalid input"))
                        }
                    },
                    PartOneState::Increasing(prev) => {
                        if num >= prev  && num - prev <= 3 {
                            Ok(PartOneState::Increasing(num))
                        } else {
                            Err(anyhow::anyhow!("Invalid input"))
                        }
                    },
                    PartOneState::Decreasing(prev) => {
                        if num <= prev  && prev - num <= 3 {
                            Ok(PartOneState::Decreasing(num))
                        } else {
                            Err(anyhow::anyhow!("Invalid input"))
                        }
                    },
                }
            }).is_ok()
        }).filter(|val| *val).count();
        println!("Answer 1: {}", answer_one);
        Ok(())
    }
}
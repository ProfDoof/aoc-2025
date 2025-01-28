use itertools::Itertools;
use nom::{
    character::complete::{line_ending, multispace0, space1, u32}, combinator::eof, error::VerboseError, multi::{many0, separated_list1}, sequence::{pair, terminated}, Finish, IResult
};

pub(crate) struct DayTwo;

fn parse_line<'a, E: nom::error::ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Vec<u32>, E> {
    terminated(separated_list1(space1, u32), line_ending)(input)
}

fn parse_lists<'a, E: nom::error::ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Vec<Vec<u32>>, E> {
    terminated(many0(parse_line), pair(multispace0, eof))(input)
}

enum PartOneState<'a> {
    Start,
    First(&'a u32),
    Increasing(&'a u32),
    Decreasing(&'a u32),
}

fn is_safe_report<'a>(state: PartOneState<'a>, num: &'a u32) -> anyhow::Result<PartOneState<'a>> {
    match state {
        PartOneState::Start => Ok(PartOneState::First(num)),
        PartOneState::First(prev) => {
            if num > prev   && num - prev <= 3 {
                Ok(PartOneState::Increasing(num))
            } else if num < prev   && prev - num <= 3 {
                Ok(PartOneState::Decreasing(num))
            } else {
                Err(anyhow::anyhow!("Invalid input"))
            }
        },
        PartOneState::Increasing(prev) => {
            if num > prev  && num - prev <= 3 {
                Ok(PartOneState::Increasing(num))
            } else {
                Err(anyhow::anyhow!("Invalid input"))
            }
        },
        PartOneState::Decreasing(prev) => {
            if num < prev  && prev - num <= 3 {
                Ok(PartOneState::Decreasing(num))
            } else {
                Err(anyhow::anyhow!("Invalid input"))
            }
        },
    }
}

enum PartTwoState<'a> {
    Increasing(&'a u32, usize),
    Decreasing(&'a u32, usize),
    Fail,
}

impl super::Day for DayTwo {
    fn run(&self, inputs: std::path::PathBuf) -> anyhow::Result<()> {
        let input_path = inputs.join("day2.input");
        let input = std::fs::read_to_string(input_path)?;
        // println!("input: {}", input);
        let reports = match parse_lists::<VerboseError<_>>(&input).finish() {
            Ok((_, reports)) => reports,
            Err(err) => {
                eprintln!("Error parsing input: {}", nom::error::convert_error(input.as_str(), err));
                return Ok(());
            }
        };
        let answer_one = reports.iter().filter(|report| {
            report.iter().try_fold(PartOneState::Start, is_safe_report).is_ok()
        }).count();
        println!("Answer 1: {}", answer_one);
        let answer_two = reports.iter().filter(|report| {
            report.iter().try_fold(PartOneState::Start, is_safe_report).is_ok() || report.iter().combinations(report.len() - 1).any(|comb| {
                comb.into_iter().try_fold(PartOneState::Start, is_safe_report).is_ok()
            })
            // let mut increasing = vec![report[0]];
            // let mut decreasing = vec![report[0]];
            // for x in report.iter().skip(1) {
            //     if increasing.last().unwrap() < x {
            //         increasing.push(*x);
            //     } else { 
            //         let idx = increasing.partition_point(|y| y < x);
            //         increasing[idx] = *x;
            //     }

            //     if decreasing.last().unwrap() > x {
            //         decreasing.push(*x);
            //     } else {
            //         let idx = decreasing.partition_point(|y| y > x);
            //         decreasing[idx] = *x;
            //     }
            // }
            // match report.iter().skip(1).try_fold (if increasing.len() >= report.len() - 1 {
            //     PartTwoState::Increasing(report.first().unwrap(), 0)
            // } else if decreasing.len() >= report.len() - 1 {
            //     PartTwoState::Decreasing(report.first().unwrap(), 0)
            // }  else { PartTwoState::Fail }, |state, num| {
            //     match state {
            //         PartTwoState::Increasing(prev, count) => {
            //             if num - prev <= 3 && num - prev >= 1 {
            //                 Some(PartTwoState::Increasing(num, count))
            //             } else {
            //                 Some(PartTwoState::Increasing(prev, count + 1))
            //             }
            //         },
            //         PartTwoState::Decreasing(prev, count) => {
            //             if prev - num <= 3 && prev - num >= 1 {
            //                 Some(PartTwoState::Decreasing(num, count))
            //             } else {
            //                 Some(PartTwoState::Decreasing(prev, count + 1))
            //             }
            //         }
            //         PartTwoState::Fail => None,
            //     }
            // }) {
            //     Some(PartTwoState::Increasing(_, count) | PartTwoState::Decreasing(_, count)) => {
            //         if count > 1 {println!("report ({}): {:?} {}", count, report, if increasing.len() >= report.len() - 1 {
            //             "+"
            //         } else if decreasing.len() >= report.len() - 1 {
            //             "-"
            //         }  else { "err" });}
            //         count <= 1},
            //     Some(PartTwoState::Fail) | None => false,
            // }
        }).count();
        println!("Answer 2: {}", answer_two);
        Ok(())
    }
}
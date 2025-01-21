use itertools::Itertools;
use nom::combinator::{eof, opt};
use nom::multi::many0;
use nom::sequence::terminated;
use nom::{sequence::separated_pair, IResult};
use nom::character::complete::{line_ending, multispace0, u32};

pub(crate) struct DayOne;

fn parse_line(input: &str) -> IResult<&str, (u32, u32)> {
    terminated(separated_pair(u32, multispace0, u32), opt(line_ending))(input)
}

fn parse_lists(input: &str) -> IResult<&str, Vec<(u32, u32)>> {
    terminated(many0(parse_line), eof)(input)
}

impl super::Day for DayOne {
    fn run(&self, inputs: std::path::PathBuf) -> anyhow::Result<()> {
        let input_path = inputs.join("day1.input");
        let input = std::fs::read_to_string(input_path)?;
        // println!("input: {}", input);
        let (_, lists) = parse_lists(&input).unwrap();
        let (mut left,mut right): (Vec<_>, Vec<_>) = lists.iter().cloned().unzip();
        // println!("left: {:?}", left);
        // println!("right: {:?}", right);
        left.sort();
        right.sort();
        let answer_one = left.iter().zip(right.iter()).map(|(l,r)| l.abs_diff(*r)).sum::<u32>();
        println!("Answer 1: {}", answer_one);
        let left_counts = left.iter().counts();
        let right_counts = right.iter().counts();
        let answer_two = left_counts.iter().map(|(k,v)| {
            let right_count = right_counts.get(k).unwrap_or(&0);
            usize::try_from(**k).unwrap() * v * right_count
        }).sum::<usize>();
        println!("Answer 2: {}", answer_two);
        Ok(())
    }
}
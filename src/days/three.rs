use chumsky::{error::Simple, extra, prelude::{any, choice, empty, end, just, Recursive}, recursive::Indirect, text::{self, Char}, IterParser, Parser};
use nom::{bytes::complete::tag, character::complete::{anychar, u32}, combinator::map, multi::{many0, many_till}, sequence::{delimited, preceded, separated_pair, terminated}, IResult};

pub (crate)struct DayThree;

fn parse_mul(input: &str) -> IResult<&str, u32> {
    let (remaining, (a, b)) = preceded(tag("mul"), delimited(tag("("), separated_pair(u32, tag(","), u32), tag(")")))(input)?;
    Ok((remaining, a * b))
}

fn parse_day_one(input: &str) -> IResult<&str, Vec<u32>> {
    terminated(many0(map(many_till(anychar, parse_mul), |(_, multiplied)| multiplied)), many0(anychar))(input)
}

#[derive(Debug, Clone)]
enum Expression {
    Mul(u32),
    Do(Vec<Expression>),
    DoNot(Vec<Expression>),
}

fn parser<'a>(handle_conditional: bool) -> impl Parser<'a, &'a str, Expression> {
    let num = text::int(10).from_str::<u32>().unwrapped();
    let multiplied = just("mul")
        .ignore_then(
            num.then_ignore(just(","))
                .then(num)
                .delimited_by(just("("), just(")"))
        )
        .map(|(a, b)| Expression::Mul(a * b)).boxed();

    let mut do_not: Recursive<Indirect<&'a str, Expression, extra::Default>> = Recursive::declare();
    let mut r#do: Recursive<Indirect<&'a str, Expression, extra::Default>> = Recursive::declare();

    do_not.define(choice((
        just("do()")
            .ignore_then(r#do.clone()).map(|do_expression| Expression::DoNot(vec![do_expression])),
        multiplied.clone().then(do_not.clone()).map(|(mul, exprs)| {
            match exprs {
                Expression::DoNot(mut vec) => {
                    vec.push(mul);
                    Expression::DoNot(vec)
                },
                Expression::Mul(_) | Expression::Do(_) => unreachable!("reached unreachable expression: {:?}", exprs),
            }
        }),
        end().to(Expression::DoNot(vec![])),
        any().ignore_then(do_not.clone())
    )));

    r#do.define(choice((
        just("don't()")
            .ignore_then(do_not.clone()).map(|do_not_expression| Expression::Do(vec![do_not_expression])),
        multiplied.clone()
            .then(r#do.clone())
            .map(|(mul, exprs)| {
                match exprs {
                    Expression::DoNot(_) | Expression::Mul(_) => unreachable!("reached unreachable expression: {:?}", exprs),
                    Expression::Do(mut vec) => {
                        vec.push(mul);
                        Expression::Do(vec)
                    },
                }
            }),
        end().to(Expression::Do(vec![])),
        any().ignore_then(r#do.clone())
    )));

    if handle_conditional {
        r#do.boxed()
    } else {
        any()
            .and_is(multiplied.clone().not())
            .repeated()
            .ignore_then(multiplied)
            .repeated()
            .collect()
            .map(|mults| Expression::Do(mults))
            .then_ignore(any().repeated()).boxed()
    }
}

fn eval(expr: Expression) -> u32 {
    match expr {
        Expression::Mul(mul) => mul,
        Expression::Do(exprs) => exprs.into_iter().map(eval).sum(),
        Expression::DoNot(exprs) => exprs.into_iter().filter(|expr| matches!(expr, Expression::Do(_))).map(eval).sum(),
    }
}

impl super::Day for DayThree {
    fn run(&self, inputs: std::path::PathBuf) -> anyhow::Result<()> {
        let input_path = inputs.join("day3.input");
        let input = std::fs::read_to_string(input_path)?;
        let day_one_parser = parser(false);
        let day_one_parse = day_one_parser.parse(&input);
        let day_one_answer = day_one_parse.into_result().map(eval).map_err(|e| anyhow::anyhow!("{:?}", e))?;
        println!("Answer 1: {}", day_one_answer);
        let day_two_parser = parser(true);
        let day_two_parse = day_two_parser.parse(&input);
        let day_two_answer = day_two_parse.into_result().map(eval).map_err(|e| anyhow::anyhow!("{:?}", e))?;
        println!("Answer 2: {}", day_two_answer);
        Ok(())
    }
}
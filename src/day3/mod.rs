use crate::prelude::*;

use std::str::FromStr;

use nom::{
    bytes::complete::{tag, take},
    character::complete::char,
    combinator::{map_res, not, peek},
    branch::alt,
    error::Error,
    multi::{many1, many0},
    sequence::{delimited, pair, preceded, separated_pair, terminated},
    Finish, IResult,
};

impl AoC for Day {
    fn run(input: &str) -> anyhow::Result<AoCResult> {
        let parsed: Day = input.parse()?;

        Ok(AoCResult {
            part_a: Some(parsed.part_a()),
            part_b: Some(parsed.part_b())
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Day {
    instr: Vec<Instruction>,
}

#[derive(Debug, PartialEq, Eq)]
enum Instruction {
    Mul { left: usize, right: usize },
    Do,
    Dont
}

impl Day {
    fn part_a(&self) -> usize {
        self.mul(false)
    }

    fn part_b(&self) -> usize {
        self.mul(true)
    }

    fn mul(&self, use_enable: bool) -> usize {
        let mut enable: bool = true;
        let mut acc = 0;
        for ins in &self.instr {
            match ins {
                Instruction::Mul{left, right} if enable => { acc += left * right; },
                Instruction::Do if use_enable => { enable = true; },
                Instruction::Dont if use_enable => { enable = false; },
                _ => {}
            }
        }
        acc
    }
}

impl FromStr for Day {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Day> {
        match parse_day(s).finish() {
            Ok(("", parsed)) => Ok(parsed),
            Ok((rest, parsed)) => Err(anyhow::anyhow!(
                "Successful parsed {:?}, but input was not fully consumed! ({:?})",
                parsed,
                rest
            )),
            Err(Error { input, code }) => Err(Error {
                input: input.to_string(),
                code,
            }
            .into()),
        }
    }
}

fn parse_mul(input: &str) -> IResult<&str, Instruction> {
    map_res(
        delimited(
            tag("mul("),
            separated_pair(parse_usize, char(','), parse_usize),
            char(')'),
        ),
        |(left, right)| -> anyhow::Result<Instruction> { Ok(Instruction::Mul { left, right }) },
    )(input)
}

fn parse_do(input: &str) -> IResult<&str, Instruction> {
    map_res(
        tag("do()"),
        |_| -> anyhow::Result<Instruction> { Ok(Instruction::Do) }
    )(input)
    }

fn parse_dont(input: &str) -> IResult<&str, Instruction> {
    map_res(
        tag("don't()"),
        |_| -> anyhow::Result<Instruction> { Ok(Instruction::Dont) }
    )(input)
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    alt((parse_mul, parse_do, parse_dont))(input)
}


fn parse_garbage(input: &str) -> IResult<&str, Vec<((), &str)>> {
    many0(pair(not(peek(parse_instruction)), take(1usize)))(input)
}

fn parse_day(input: &str) -> IResult<&str, Day> {
    map_res(
        terminated(
            many1(preceded(parse_garbage, parse_instruction)),
            parse_garbage
        ),
        |instr| -> anyhow::Result<Day> { Ok(Day { instr }) },
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    fn example() -> &'static str {
        "\
        xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))\n\
        "
    }

    #[fixture]
    fn example_parsed() -> Day {
        Day {
            instr: [
                Instruction::Mul { left: 2, right: 4 },
                Instruction::Mul { left: 5, right: 5 },
                Instruction::Mul { left: 11, right: 8 },
                Instruction::Mul { left: 8, right: 5 },
            ]
            .into(),
        }
    }

    #[rstest]
    fn parse_example_a(example: &'static str, example_parsed: Day) {
        let result: Day = example.parse().unwrap();
        assert_eq!(result, example_parsed)
    }

    #[rstest]
    fn test_part_a(example_parsed: Day) {
        assert_eq!(example_parsed.part_a(), 161)
    }

    #[fixture]
    fn example_b() -> &'static str {
        "\
        xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))\n\
        "
    }

    #[fixture]
    fn example_b_parsed() -> Day {
        Day {
            instr: [
                Instruction::Mul { left: 2, right: 4 },
                Instruction::Dont,
                Instruction::Mul { left: 5, right: 5 },
                Instruction::Mul { left: 11, right: 8 },
                Instruction::Do,
                Instruction::Mul { left: 8, right: 5 },
            ]
            .into(),
        }
    }

    #[rstest]
    fn test_part_b(example_b_parsed: Day) {
        assert_eq!(example_b_parsed.part_b(), 48)
    }
}

use crate::prelude::*;

use std::str::FromStr;

use crate::table::TableIdx;

use ndarray::prelude::*;
use ndarray_linalg::*;

use nom::{
    bytes::complete::tag,
    character::complete::newline,
    combinator::map_res,
    error::Error,
    multi::{count, separated_list1},
    sequence::{delimited, preceded, separated_pair, terminated, tuple},
    Finish, IResult,
};

impl AoC for Day {
    fn run(input: &str) -> anyhow::Result<AoCResult> {
        let parsed: Day = input.parse()?;

        Ok(AoCResult {
            part_a: Some(parsed.part_a()),
            part_b: None,
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Day {
    machines: Vec<Machine>,
}

impl Day {
    fn part_a(&self) -> usize {
        self.machines.iter().flat_map(|m| m.solve()).sum()
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

fn parse_day(input: &str) -> IResult<&str, Day> {
    map_res(
        terminated(separated_list1(count(newline, 2), parse_machine), newline),
        |machines| -> anyhow::Result<Day> { Ok(Day { machines }) },
    )(input)
}

const COST: TableIdx = TableIdx::new(3, 1);

const MAX_BUTTON_PRESSES : usize = 100;

#[derive(Debug, PartialEq, Eq)]
struct Machine {
    a: TableIdx,
    b: TableIdx,
    prize: TableIdx,
}

impl Machine {
    fn solve(&self) -> Option<usize> {
        let a: Array2<f64> = array![[self.a.x as f64, self.b.x as f64] , [self.a.y as f64, self.b.y as f64]];
        let b: Array1<f64> = array![self.prize.x as f64, self.prize.y as f64];
        let x: Array1<usize> = a.solve_into(b).ok()?.map(|x| x.round() as usize);
        let solution = TableIdx::new(x[0], x[1]);
        if solution.x > MAX_BUTTON_PRESSES || solution.y > MAX_BUTTON_PRESSES {
            return None;
        }
        if solution.x * self.a.x + solution.y * self.b.x != self.prize.x || solution.x * self.a.y + solution.y * self.b.y != self.prize.y {
            return None;
        }
        Some(x[0]* COST.x + x[1])
    }
}

fn parse_button(input: &str) -> IResult<&str, TableIdx> {
    map_res(
        separated_pair(
            preceded(tag("X+"), parse_usize),
            tag(", "),
            preceded(tag("Y+"), parse_usize),
        ),
        |(x, y)| -> anyhow::Result<TableIdx> { Ok(TableIdx::new(x, y)) },
    )(input)
}

fn parse_prize(input: &str) -> IResult<&str, TableIdx> {
    map_res(
        preceded(
            tag("Prize: "),
            separated_pair(
                preceded(tag("X="), parse_usize),
                tag(", "),
                preceded(tag("Y="), parse_usize),
            ),
        ),
        |(x, y)| -> anyhow::Result<TableIdx> { Ok(TableIdx::new(x, y)) },
    )(input)
}

fn parse_machine(input: &str) -> IResult<&str, Machine> {
    map_res(
        tuple((
            delimited(tag("Button A: "), parse_button, newline),
            delimited(tag("Button B: "), parse_button, newline),
            parse_prize,
        )),
        |(a, b, prize)| -> anyhow::Result<Machine> { Ok(Machine { a, b, prize }) },
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    use vek::Vec2;

    #[fixture]
    fn example() -> &'static str {
        "\
        Button A: X+94, Y+34\n\
        Button B: X+22, Y+67\n\
        Prize: X=8400, Y=5400\n\
        \n\
        Button A: X+26, Y+66\n\
        Button B: X+67, Y+21\n\
        Prize: X=12748, Y=12176\n\
        \n\
        Button A: X+17, Y+86\n\
        Button B: X+84, Y+37\n\
        Prize: X=7870, Y=6450\n\
        \n\
        Button A: X+69, Y+23\n\
        Button B: X+27, Y+71\n\
        Prize: X=18641, Y=10279\n\
        "
    }

    #[fixture]
    fn example_parsed() -> Day {
        Day {
            machines: [
                Machine {
                    a: Vec2 { x: 94, y: 34 },
                    b: Vec2 { x: 22, y: 67 },
                    prize: Vec2 { x: 8400, y: 5400 },
                },
                Machine {
                    a: Vec2 { x: 26, y: 66 },
                    b: Vec2 { x: 67, y: 21 },
                    prize: Vec2 { x: 12748, y: 12176 },
                },
                Machine {
                    a: Vec2 { x: 17, y: 86 },
                    b: Vec2 { x: 84, y: 37 },
                    prize: Vec2 { x: 7870, y: 6450 },
                },
                Machine {
                    a: Vec2 { x: 69, y: 23 },
                    b: Vec2 { x: 27, y: 71 },
                    prize: Vec2 { x: 18641, y: 10279 },
                },
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
        assert_eq!(example_parsed.part_a(), 480)
    }
}

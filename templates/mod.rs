use crate::prelude::*;

use std::str::FromStr;

use nom::{
    character::complete::{digit1, newline, space1},
    combinator::map_res,
    error::Error,
    multi::separated_list1,
    sequence::{separated_pair, terminated},
    Finish, IResult,
};

impl AoC for Day {
    fn run(input: &str) -> anyhow::Result<AoCResult> {
        let parsed: Day = input.parse()?;

        Ok(AoCResult {
            part_a : None,
            part_b : None
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Day {
}

impl Day {
}

impl FromStr for Day {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Day> {
        match parse_day(s).finish() {
            Ok(("", parsed)) => Ok(parsed),
            Ok((rest, parsed)) => Err(anyhow::anyhow!("Successful parsed {:?}, but input was not fully consumed! ({:?})", parsed, rest)),
            Err(Error { input, code }) => Err(Error {
                input: input.to_string(),
                code,
            }.into()),
        }
    }
}

fn parse_day(input: &str) -> IResult<&str, Day> {
    Ok(("", Day {}))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    fn example() -> &'static str {
        "\
        "
    }

    #[fixture]
    fn example_parsed() -> Day {
        Day {
        }
    }

    #[rstest]
    fn parse_example_a(example: &'static str, example_parsed: Day) {
        let result: Day = example.parse().unwrap();
        assert_eq!(result, example_parsed)
    }
}

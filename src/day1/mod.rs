use crate::prelude::*;

use std::str::FromStr;

use itertools::{sorted, Itertools};
use std::iter::zip;

use nom::{
    character::complete::{digit1, newline, space1},
    combinator::map_res,
    error::Error,
    multi::separated_list1,
    sequence::{separated_pair, terminated},
    Finish, IResult,
};

impl AoC for Day1 {
    fn run(input: &str) -> anyhow::Result<AoCResult> {
        let parsed: Day1 = input.parse()?;

        Ok(AoCResult {
            part_a : Some(parsed.total_distance()),
            part_b : Some(parsed.similarity_score())
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Day1 {
    left: Vec<usize>,
    right: Vec<usize>,
}

impl Day1 {
    fn total_distance(&self) -> usize {
        zip(sorted(self.left.iter()), sorted(self.right.iter()))
        .map(|(x, y)| {
                x.abs_diff(*y)
            }).sum()
    }

    fn similarity_score(&self) -> usize {
        let counts = self.right.iter().counts();
        self.left.iter().map(|x| {
            match counts.get(x) {
                Some(val) => x * *val,
                None => 0
            }
        }).sum()
    }
}

impl FromStr for Day1 {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Day1> {
        match parse_two_lists(s).finish() {
            Ok(("", parsed)) => Ok(parsed),
            Ok((rest, parsed)) => Err(anyhow::anyhow!("Successful parsed {:?}, but input was not fully consumed! ({:?})", parsed, rest)),
            Err(Error { input, code }) => Err(Error {
                input: input.to_string(),
                code,
            }.into()),
        }
    }
}

fn parseu32(input: &str) -> IResult<&str, usize> {
    map_res(digit1, str::parse)(input)
}

fn parse_two_lists(input: &str) -> IResult<&str, Day1> {
    map_res(
        terminated(separated_list1(newline, separated_pair(parseu32, space1, parseu32)), newline),
        |vec| -> anyhow::Result<Day1> {
            let (left, right) = vec.into_iter().unzip();
            Ok(Day1 { left, right })
        },
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    fn example() -> &'static str {
        "\
        3   4\n\
        4   3\n\
        2   5\n\
        1   3\n\
        3   9\n\
        3   3\n\
        "
    }

    #[fixture]
    fn example_parsed() -> Day1 {
        Day1 {
            left: [3, 4, 2, 1, 3, 3].into(),
            right: [4, 3, 5, 3, 9, 3].into(),
        }
    }

    #[rstest]
    fn parse_example_a(example: &'static str, example_parsed: Day1) {
        let result: Day1 = example.parse().unwrap();
        assert_eq!(result, example_parsed)
    }

    #[rstest]
    fn total_distance_a(example_parsed: Day1) {
        assert_eq!(example_parsed.total_distance(), 11)
    }

    #[rstest]
    fn test_similarity_score(example_parsed: Day1) {
        assert_eq!(example_parsed.similarity_score(), 31)
    }
}

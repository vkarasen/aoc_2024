use crate::prelude::*;

use std::str::FromStr;

use nom::{
    bytes::complete::tag,
    character::complete::newline,
    combinator::map_res,
    error::Error,
    multi::separated_list1,
    sequence::{separated_pair, terminated},
    Finish, IResult,
};

use itertools::{Itertools, repeat_n};

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
    entries: Vec<Entry>,
}

impl Day {
    fn part_a(&self) -> usize {
        self.entries.iter().filter_map(|e| {
            if e.evaluates() {
                Some(e.left)
            } else {
                None
            }
        }).sum()
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
        terminated(separated_list1(newline, parse_entry), newline),
        |entries| -> anyhow::Result<Day> { Ok(Day { entries }) },
    )(input)
}

#[derive(Debug, PartialEq, Eq)]
struct Entry {
    left: usize,
    right: Vec<usize>,
}

impl Entry {
    fn evaluates(&self) -> bool {
        let comblen = self.right.len() - 1;
        for ops in repeat_n(['*', '+'].iter(), comblen).multi_cartesian_product() {
            let mut acc = self.right[0];
            for (op, val) in ops.into_iter().zip(self.right.iter().skip(1)) {
                match op {
                    '+' => { acc += val; },
                    '*' => { acc *= val; },
                    _ => unreachable!()
                }
                if acc > self.left {
                    break;
                }
            }
            if acc == self.left {
                return true;
            }
        }
        false
    }
}

fn parse_entry(input: &str) -> IResult<&str, Entry> {
    map_res(
        separated_pair(
            parse_usize,
            tag(": "),
            separated_list1(tag(" "), parse_usize),
        ),
        |(left, right)| -> anyhow::Result<Entry> { Ok(Entry { left, right }) },
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    fn example() -> &'static str {
        "\
            190: 10 19\n\
            3267: 81 40 27\n\
            83: 17 5\n\
            156: 15 6\n\
            7290: 6 8 6 15\n\
            161011: 16 10 13\n\
            192: 17 8 14\n\
            21037: 9 7 18 13\n\
            292: 11 6 16 20\n\
        "
    }

    #[fixture]
    fn example_parsed() -> Day {
        Day {
            entries: [
                Entry {
                    left: 190,
                    right: [10, 19].into(),
                },
                Entry {
                    left: 3267,
                    right: [81, 40, 27].into(),
                },
                Entry {
                    left: 83,
                    right: [17, 5].into(),
                },
                Entry {
                    left: 156,
                    right: [15, 6].into(),
                },
                Entry {
                    left: 7290,
                    right: [6, 8, 6, 15].into(),
                },
                Entry {
                    left: 161011,
                    right: [16, 10, 13].into(),
                },
                Entry {
                    left: 192,
                    right: [17, 8, 14].into(),
                },
                Entry {
                    left: 21037,
                    right: [9, 7, 18, 13].into(),
                },
                Entry {
                    left: 292,
                    right: [11, 6, 16, 20].into(),
                },
            ].into(),
        }
    }

    #[rstest]
    fn parse_example_a(example: &'static str, example_parsed: Day) {
        let result: Day = example.parse().unwrap();
        assert_eq!(result, example_parsed)
    }

    #[rstest]
    fn test_part_a(example_parsed: Day) {
        assert_eq!(example_parsed.part_a(), 3749)
    }
}

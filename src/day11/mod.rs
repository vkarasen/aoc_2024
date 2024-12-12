use crate::prelude::*;

use std::str::FromStr;

use nom::{
    bytes::complete::tag,
    character::complete::{digit1, newline},
    combinator::map_res,
    error::Error,
    multi::separated_list1,
    sequence::terminated,
    Finish, IResult,
};

impl AoC for Day {
    fn run(input: &str) -> anyhow::Result<AoCResult> {
        let parsed: Day = input.parse()?;

        Ok(AoCResult {
            part_a: Some(parsed.part_a()),
            part_b: Some(parsed.part_b()),
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Day {
    stone_row: StoneRow,
}

impl Day {
    fn part_a(&self) -> usize {
        self.stone_row.blink_iter().nth(24).unwrap().lengths.len()
    }

    fn part_b(&self) -> usize {
        0
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
        terminated(separated_list1(tag(" "), digit1), newline),
        |v: Vec<&str>| -> anyhow::Result<Day> {
            let mut row = String::new();
            let mut lengths = Vec::new();

            for x in v.into_iter() {
                let trimmed = x.trim_start_matches('0');
                lengths.push(trimmed.len());
                row.push_str(trimmed);
            }

            Ok(Day {
                stone_row: StoneRow { row, lengths },
            })
        },
    )(input)
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
struct StoneRow {
    row: String,
    lengths: Vec<usize>,
}

impl StoneRow {
    fn blink(&self) -> Self {
        let mut retval = Self::default();
        let mut idx = 0;
        for len in self.lengths.iter() {
            if *len == 0 {
                retval.row.push('1');
                retval.lengths.push(1);
            } else if *len % 2 == 0 {
                retval.row.push_str(&self.row[idx..idx + len / 2]);
                idx += *len / 2;
                retval.lengths.push(*len / 2);

                let right = self.row[idx..idx + len / 2].trim_start_matches('0');
                retval.row.push_str(right);
                retval.lengths.push(right.len());
                idx += *len / 2;
            } else {
                let num = self.row[idx..idx + len].parse::<u64>().unwrap() * 2024;
                let numstr = format!("{}", num);
                retval.row.push_str(&numstr);
                retval.lengths.push(numstr.len());

                idx += *len;
            }
        }
        retval
    }

    fn blink_iter(&self) -> impl Iterator<Item = Self> {
        StoneRowIter { row: self.clone() }
    }
}

impl std::fmt::Display for StoneRow {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut ret = String::new();
        let mut idx = 0;
        for len in self.lengths.iter() {
            ret.push_str(&self.row[idx..idx+len]);
            ret.push(' ');
            idx += len;
        }
        write!(f, "{}", &ret)
    }

}

struct StoneRowIter {
    row: StoneRow,
}

impl std::iter::Iterator for StoneRowIter {
    type Item = StoneRow;

    fn next(&mut self) -> Option<Self::Item> {
        self.row = self.row.blink();
        //dbg!(&self.row);

        Some(self.row.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    fn example() -> &'static str {
        "\
        125 17\n\
        "
    }

    #[fixture]
    fn example_parsed() -> Day {
        Day {
            stone_row: StoneRow {
                row: "12517".into(),
                lengths: [3, 2].into(),
            },
        }
    }

    #[rstest]
    fn parse_example_a(example: &'static str, example_parsed: Day) {
        let result: Day = example.parse().unwrap();
        assert_eq!(result, example_parsed)
    }

    #[rstest]
    fn test_part_a(example_parsed: Day) {
        assert_eq!(example_parsed.part_a(), 55312)
    }
}

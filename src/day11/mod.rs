use crate::prelude::*;

use std::str::FromStr;

use std::collections::HashMap;

type StoneMap = HashMap<usize, usize>;

use nom::{
    bytes::complete::tag,
    character::complete::newline,
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
    stones: StoneMap
}

impl Day {
    fn part_a(&self) -> usize {
        self.blink().nth(24).unwrap()
    }

    fn part_b(&self) -> usize {
        self.blink().nth(74).unwrap()
    }

    fn blink(&self) -> StoneIter {
        StoneIter {
            stones : self.stones.clone()
        }
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
        terminated(separated_list1(tag(" "), parse_usize), newline),
        |v: Vec<usize>| -> anyhow::Result<Day> {

            let mut stones = HashMap::new();

            for stone in v.into_iter() {
                stones.entry(stone).and_modify(|s| *s += 1).or_insert(1);
            }

            Ok(Day {
                stones
            })
        },
    )(input)
}



struct StoneIter {
    stones: StoneMap
}

impl StoneIter {
    fn blink(&self) -> StoneMap {
        let mut retval = HashMap::new();
        for (stone, number) in self.stones.iter() {
            let mut interr = |stone: usize| {retval.entry(stone).and_modify(|s| *s += *number).or_insert(*number); };
            let stonestr = format!("{}", &stone);
            if *stone == 0 {
                interr(1);
            } else if stonestr.len() % 2 == 0 {
                interr(stonestr[..stonestr.len()/2].parse().unwrap());
                interr(stonestr[stonestr.len()/2..].parse().unwrap());
            } else {
                interr(stone*2024);
            }
        }
        retval

    }
}

impl std::iter::Iterator for StoneIter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.stones = self.blink();

        Some(self.stones.values().sum())
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
            stones : [(17, 1), (125, 1)].into()
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

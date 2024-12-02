use crate::prelude::*;

use std::str::FromStr;

use nom::{
    character::complete::{digit1, newline, space1},
    combinator::map_res,
    error::Error,
    multi::separated_list1,
    sequence::terminated,
    Finish, IResult,
};

use itertools::Itertools;

impl AoC for Day {
    fn run(input: &str) -> anyhow::Result<AoCResult> {
        let parsed: Day = input.parse()?;

        Ok(AoCResult {
            part_a: Some(parsed.part_a()),
            part_b: Some(parsed.part_b()),
        })
    }
}

#[derive(Debug, PartialEq, Eq, Default)]
pub struct Day {
    reports: Vec<Report>,
}

impl Day {
    fn safeties(&self) -> impl Iterator<Item = bool> + '_ {
        self.reports.iter().map(|x| x.is_safe())
    }

    fn dampened(&self) -> impl Iterator<Item = bool> + '_ {
        self.reports.iter().map(|x| x.is_safety_dampened())
    }

    fn part_a(&self) -> usize {
        self.safeties().filter(|x| *x).count()
    }

    fn part_b(&self) -> usize {
        self.dampened().filter(|x| *x).count()
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

fn parse_u32(input: &str) -> IResult<&str, u32> {
    map_res(digit1, str::parse)(input)
}

fn parse_level(input: &str) -> IResult<&str, Report> {
    map_res(
        separated_list1(space1, parse_u32),
        |lst| -> anyhow::Result<Report> { Ok(lst.into()) },
    )(input)
}

fn parse_day(input: &str) -> IResult<&str, Day> {
    map_res(
        terminated(separated_list1(newline, parse_level), newline),
        |reports| -> anyhow::Result<Day> { Ok(Day { reports }) },
    )(input)
}

#[derive(Debug, PartialEq, Eq, Default)]
struct Report(Vec<u32>);

impl From<Vec<u32>> for Report {
    fn from(vec: Vec<u32>) -> Self {
        Report(vec)
    }
}

impl Report {
    fn all_in_range(&self, range: std::ops::Range<i32>) -> bool {
        self.0
            .iter()
            .tuple_windows()
            .map(|(prev, next)| *next as i32 - *prev as i32)
            .all(|x| range.contains(&x))
    }

    fn is_safe(&self) -> bool {
        self.all_in_range(-3..0) || self.all_in_range(1..4)
    }

    fn safety_dampen(&self, idx: usize) -> Self {
        let arr = self.0.clone().into_iter().enumerate().filter_map(|(i, val)| {
            if i == idx {
                return None;
            }
            Some(val)
        }).collect::<Vec<u32>>();
        Report(arr)
    }

    fn is_safety_dampened(&self) -> bool {
        if self.is_safe() {
            return true;
        }

        for idx in 0..self.0.len() {
            if self.safety_dampen(idx).is_safe() {
                return true;
            }
        }

        false
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    fn example() -> &'static str {
        "\
        7 6 4 2 1\n\
        1 2 7 8 9\n\
        9 7 6 2 1\n\
        1 3 2 4 5\n\
        8 6 4 4 1\n\
        1 3 6 7 9\n\
        "
    }

    #[fixture]
    fn example_parsed() -> Day {
        Day {
            reports: [
                Vec::from([7, 6, 4, 2, 1]).into(),
                Vec::from([1, 2, 7, 8, 9]).into(),
                Vec::from([9, 7, 6, 2, 1]).into(),
                Vec::from([1, 3, 2, 4, 5]).into(),
                Vec::from([8, 6, 4, 4, 1]).into(),
                Vec::from([1, 3, 6, 7, 9]).into(),
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
    fn test_safeties(example_parsed: Day) {
        let safeties: Vec<bool> = example_parsed.safeties().collect();
        assert_eq!(
            safeties,
            Vec::from([true, false, false, false, false, true])
        )
    }

    #[rstest]
    fn test_part_a(example_parsed: Day) {
        assert_eq!(example_parsed.part_a(), 2)
    }

    #[rstest]
    fn test_part_b(example_parsed: Day) {
        assert_eq!(example_parsed.part_b(), 4)
    }

    #[rstest]
    fn test_dampened(example_parsed: Day) {
        let safeties: Vec<bool> = example_parsed.dampened().collect();
        assert_eq!(safeties, Vec::from([true, false, false, true, true, true]))
    }

    #[rstest]
    #[case(Report(Vec::from([25,26,29,30,32,35,37,35])), false)]
    #[case(Report(Vec::from([27,27,29,27,31])), false)]
    #[case(Report(Vec::from([1,5,7,12,12])), false)]
    #[case(Report(Vec::from([7,6,4,2,1])), true)]
    #[case(Report(Vec::from([1,2,7,8,9])), false)]
    #[case(Report(Vec::from([9,7,6,2,1])), false)]
    #[case(Report(Vec::from([1,3,2,4,5])), false)]
    #[case(Report(Vec::from([8,6,4,4,1])), false)]
    #[case(Report(Vec::from([1,3,6,7,9])), true)]
    fn test_report_safety(#[case] input: Report, #[case] expected: bool) {
        assert_eq!(input.is_safe(), expected)
    }
}

use crate::prelude::*;

use std::str::FromStr;

use nom::{
    bytes::complete::tag,
    character::complete::{digit1, newline, space1},
    combinator::map_res,
    error::Error,
    multi::separated_list1,
    sequence::{separated_pair, terminated},
    Finish, IResult,
};

use itertools::Itertools;

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
    rules: Vec<PageOrderRule>,
    updates: Vec<Update>,
}

fn in_correct_order(update: &Update, rules: &[PageOrderRule]) -> bool {
    ! update.0.iter().combinations(2).map(|x| {
        PageOrderRule {first: *x[1], second: *x[0]}
    }).any(|rule| rules.contains(&rule)) 
}

impl Day {
    fn correctly_ordered(&self) -> impl Iterator<Item = bool> + '_ {
        self.updates
            .iter()
            .map(|u| in_correct_order(u, &self.rules))
    }

    fn part_a(&self) -> usize {
        self.updates.iter().filter_map(|u| {
            if in_correct_order(u, &self.rules) {
                return Some(u.0[u.0.len()/2]);
            }
            None
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
        separated_pair(
            terminated(separated_list1(newline, parse_page_order_rule), newline),
            newline,
            terminated(separated_list1(newline, parse_update), newline),
        ),
        |(rules, updates)| -> anyhow::Result<Day> { Ok(Day { rules, updates }) },
    )(input)
}

#[derive(Debug, PartialEq, Eq)]
struct PageOrderRule {
    first: usize,
    second: usize,
}

fn parse_page_order_rule(input: &str) -> IResult<&str, PageOrderRule> {
    map_res(
        separated_pair(parse_usize, tag("|"), parse_usize),
        |(first, second)| -> anyhow::Result<PageOrderRule> { Ok(PageOrderRule { first, second }) },
    )(input)
}

#[derive(Debug, PartialEq, Eq)]
struct Update(Vec<usize>);

fn parse_update(input: &str) -> IResult<&str, Update> {
    map_res(
        separated_list1(tag(","), parse_usize),
        |v| -> anyhow::Result<Update> { Ok(Update(v)) },
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    fn example() -> &'static str {
        "\
        47|53\n\
        97|13\n\
        97|61\n\
        97|47\n\
        75|29\n\
        61|13\n\
        75|53\n\
        29|13\n\
        97|29\n\
        53|29\n\
        61|53\n\
        97|53\n\
        61|29\n\
        47|13\n\
        75|47\n\
        97|75\n\
        47|61\n\
        75|61\n\
        47|29\n\
        75|13\n\
        53|13\n\
        \n\
        75,47,61,53,29\n\
        97,61,53,29,13\n\
        75,29,13\n\
        75,97,47,61,53\n\
        61,13,29\n\
        97,13,75,29,47\n\
        "
    }

    #[fixture]
    fn example_parsed() -> Day {
        Day {
            rules: [
                PageOrderRule {
                    first: 47,
                    second: 53,
                },
                PageOrderRule {
                    first: 97,
                    second: 13,
                },
                PageOrderRule {
                    first: 97,
                    second: 61,
                },
                PageOrderRule {
                    first: 97,
                    second: 47,
                },
                PageOrderRule {
                    first: 75,
                    second: 29,
                },
                PageOrderRule {
                    first: 61,
                    second: 13,
                },
                PageOrderRule {
                    first: 75,
                    second: 53,
                },
                PageOrderRule {
                    first: 29,
                    second: 13,
                },
                PageOrderRule {
                    first: 97,
                    second: 29,
                },
                PageOrderRule {
                    first: 53,
                    second: 29,
                },
                PageOrderRule {
                    first: 61,
                    second: 53,
                },
                PageOrderRule {
                    first: 97,
                    second: 53,
                },
                PageOrderRule {
                    first: 61,
                    second: 29,
                },
                PageOrderRule {
                    first: 47,
                    second: 13,
                },
                PageOrderRule {
                    first: 75,
                    second: 47,
                },
                PageOrderRule {
                    first: 97,
                    second: 75,
                },
                PageOrderRule {
                    first: 47,
                    second: 61,
                },
                PageOrderRule {
                    first: 75,
                    second: 61,
                },
                PageOrderRule {
                    first: 47,
                    second: 29,
                },
                PageOrderRule {
                    first: 75,
                    second: 13,
                },
                PageOrderRule {
                    first: 53,
                    second: 13,
                },
            ]
            .into(),
            updates: [
                Update([75, 47, 61, 53, 29].into()),
                Update([97, 61, 53, 29, 13].into()),
                Update([75, 29, 13].into()),
                Update([75, 97, 47, 61, 53].into()),
                Update([61, 13, 29].into()),
                Update([97, 13, 75, 29, 47].into()),
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
    fn test_correct_orders(example_parsed: Day) {
        let orders: Vec<bool> = example_parsed.correctly_ordered().collect();
        assert_eq!(orders, [true, true, true, false, false, false])
    }

    #[rstest]
    fn test_part_a(example_parsed: Day) {
        assert_eq!(example_parsed.part_a(), 143)
    }
}

use crate::prelude::*;

use std::str::FromStr;

use crate::table::{TableDir, TableIdx};

use nom::{
    bytes::complete::tag,
    character::complete::newline,
    combinator::map_res,
    error::Error,
    multi::separated_list1,
    sequence::{preceded, separated_pair, terminated},
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
    robots: Vec<Robot>,
}

impl Day {
    fn bathroom(&self, width: usize, height: usize) -> Bathroom {
        Bathroom {
            robots: &self.robots,
            width,
            height,
        }
    }

    fn part_a(&self) -> usize {
        self.bathroom(101, 103).safety_factor(100)
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
        terminated(
            separated_list1(newline, separated_pair(parse_p, tag(" "), parse_v)),
            newline,
        ),
        |x| -> anyhow::Result<Day> {
            let robots = x.into_iter().map(|(p, v)| Robot { p, v }).collect();
            Ok(Day { robots })
        },
    )(input)
}

fn parse_p(input: &str) -> IResult<&str, TableIdx> {
    map_res(
        preceded(
            tag("p="),
            separated_pair(parse_usize, tag(","), parse_usize),
        ),
        |(x, y)| -> anyhow::Result<TableIdx> { Ok(TableIdx::new(x, y)) },
    )(input)
}

fn parse_v(input: &str) -> IResult<&str, TableDir> {
    map_res(
        preceded(
            tag("v="),
            separated_pair(parse_isize, tag(","), parse_isize),
        ),
        |(x, y)| -> anyhow::Result<TableDir> { Ok(TableDir::new(x, y)) },
    )(input)
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Robot {
    p: TableIdx,
    v: TableDir,
}

struct Bathroom<'a> {
    robots: &'a Vec<Robot>,
    width: usize,
    height: usize,
}

impl<'a> Bathroom<'a> {
    fn walk(&self, duration: usize) -> impl Iterator<Item = TableIdx> + '_ {
        self.robots
            .iter()
            .map(move |r| r.walk(self.width, self.height, duration))
    }

    fn quadrant(&self, pos: &TableIdx) -> Option<usize> {
        use std::cmp::Ordering::*;

        match (((self.width - 1) / 2).cmp(&pos.x), ((self.height - 1) / 2).cmp(&pos.y)) {
            (Less, Less) => Some(0),
            (Less, Greater) => Some(1),
            (Greater, Less) => Some(2),
            (Greater, Greater) => Some(3),
            (_, _) => None
        }
    }

    fn safety_factor(&self, duration: usize) -> usize {
        let mut quadrants = [0usize; 4];

        for rpos in self.walk(duration) {
            if let Some(idx) = self.quadrant(&rpos) {
                quadrants[idx] += 1;
            }
        }

        quadrants.iter().product()
    }
}

impl Robot {
    fn walk(&self, width: usize, height: usize, duration: usize) -> TableIdx {
        let beeg = self.p.as_::<i64>() + duration as i64 * self.v.as_::<i64>();
        TableIdx::new(beeg.x.rem_euclid(width.try_into().unwrap()) as usize, beeg.y.rem_euclid(height.try_into().unwrap()) as usize)

    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    fn example() -> &'static str {
        "\
            p=0,4 v=3,-3\n\
            p=6,3 v=-1,-3\n\
            p=10,3 v=-1,2\n\
            p=2,0 v=2,-1\n\
            p=0,0 v=1,3\n\
            p=3,0 v=-2,-2\n\
            p=7,6 v=-1,-3\n\
            p=3,0 v=-1,-2\n\
            p=9,3 v=2,3\n\
            p=7,3 v=-1,2\n\
            p=2,4 v=2,-3\n\
            p=9,5 v=-3,-3\n\
        "
    }

    #[fixture]
    fn example_parsed() -> Day {
        Day {
            robots: [
                Robot {
                    p: TableIdx::new(0, 4),
                    v: TableDir::new(3, -3),
                },
                Robot {
                    p: TableIdx { x: 6, y: 3 },
                    v: TableDir { x: -1, y: -3 },
                },
                Robot {
                    p: TableIdx { x: 10, y: 3 },
                    v: TableDir { x: -1, y: 2 },
                },
                Robot {
                    p: TableIdx { x: 2, y: 0 },
                    v: TableDir { x: 2, y: -1 },
                },
                Robot {
                    p: TableIdx { x: 0, y: 0 },
                    v: TableDir { x: 1, y: 3 },
                },
                Robot {
                    p: TableIdx { x: 3, y: 0 },
                    v: TableDir { x: -2, y: -2 },
                },
                Robot {
                    p: TableIdx { x: 7, y: 6 },
                    v: TableDir { x: -1, y: -3 },
                },
                Robot {
                    p: TableIdx { x: 3, y: 0 },
                    v: TableDir { x: -1, y: -2 },
                },
                Robot {
                    p: TableIdx { x: 9, y: 3 },
                    v: TableDir { x: 2, y: 3 },
                },
                Robot {
                    p: TableIdx { x: 7, y: 3 },
                    v: TableDir { x: -1, y: 2 },
                },
                Robot {
                    p: TableIdx { x: 2, y: 4 },
                    v: TableDir { x: 2, y: -3 },
                },
                Robot {
                    p: TableIdx { x: 9, y: 5 },
                    v: TableDir { x: -3, y: -3 },
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
    fn test_bathroom(example_parsed: Day) {
        assert_eq!(example_parsed.bathroom(11, 7).safety_factor(100), 12)
    }
}

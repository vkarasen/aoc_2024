use crate::prelude::*;

use std::str::FromStr;

use crate::table::{TableDir, TableIdx, PPCharTable, CharTable, into_shape};

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
            part_b: Some(parsed.part_b()),
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
            robots: self.robots.clone(),
            width,
            height,
        }
    }

    fn part_a(&self) -> usize {
        let mut bathroom = self.bathroom(101, 103);
        bathroom.walk(100);
        bathroom.quadrants().safety_factor()
    }

    fn part_b(&self) -> usize {
        let mut bathroom = self.bathroom(101, 103);
        let mut min_safety_factor = usize::MAX;
        let mut ret = 0;
        let mut tree = String::new();
        for cur in 1..bathroom.width*bathroom.height {
            bathroom.walk(1);
            let safety_factor = bathroom.quadrants().safety_factor();
            if safety_factor < min_safety_factor {
                min_safety_factor = safety_factor;
                ret = cur;
                tree = format!("{}", &bathroom);
            }
        }
        println!("{}", &tree);
        ret
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

struct Bathroom {
    robots: Vec<Robot>,
    width: usize,
    height: usize,
}

impl std::fmt::Display for Bathroom {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut table = CharTable::from_shape_simple_fn((self.height, self.width), || '.');

        for robot in &self.robots {
            let elem = table.get_mut(into_shape(robot.p)).unwrap();
            match elem {
                '.' => *elem = '1',
                ref val if val.is_ascii_hexdigit() => *elem = char::from_digit(val.to_digit(16).unwrap() + 1, 16).unwrap(),
                _ => unreachable!()
            }
        }

        let center = self.center();

        for y in 0..self.height {
            *table.get_mut(into_shape(TableIdx::new(center.x, y))).unwrap() = ' ';
        }

        for x in 0..self.width {
            *table.get_mut(into_shape(TableIdx::new(x, center.y))).unwrap() = ' ';
        }

        write!(f, "{:?}", PPCharTable::from(&table))
    }
}

impl Bathroom {
    fn walk(&mut self, duration: isize) {
        self.robots
            .iter_mut()
            .for_each(|r| r.walk(self.width, self.height, duration));
    }

    fn center(&self) -> TableIdx {
        TableIdx::new((self.width - 1) / 2, (self.height - 1) / 2)
    }

    fn quadrant_idx(&self, pos: &TableIdx) -> Option<usize> {
        use std::cmp::Ordering::*;

        let center = self.center();

        match (center.x.cmp(&pos.x), center.y.cmp(&pos.y)) {
            (Less, Less) => Some(3),
            (Greater, Less) => Some(2),
            (Less, Greater) => Some(1),
            (Greater, Greater) => Some(0),
            (_, _) => None
        }
    }

    fn quadrants(&self) -> Quadrants {
        let mut quadrants = Quadrants::default();

        for robot in &self.robots {
            if let Some(idx) = self.quadrant_idx(&robot.p) {
                quadrants.0[idx] += 1;
            }
        }

        quadrants
    }
}

impl Robot {
    fn walk(&mut self, width: usize, height: usize, duration: isize) {
        let beeg = self.p.as_::<i64>() + duration as i64 * self.v.as_::<i64>();
        self.p = TableIdx::new(beeg.x.rem_euclid(width.try_into().unwrap()) as usize, beeg.y.rem_euclid(height.try_into().unwrap()) as usize);

    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Default)]
struct Quadrants([usize; 4]);

impl Quadrants {
    fn safety_factor(&self) -> usize {
        self.0.iter().product()
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
        let mut bathroom = example_parsed.bathroom(11, 7);
        bathroom.walk(100);
        assert_eq!(bathroom.quadrants().safety_factor(), 12)

    }
}

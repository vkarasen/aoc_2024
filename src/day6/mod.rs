use crate::prelude::*;

use anyhow::anyhow;

use crate::table::{into_idx, into_shape, parse_char_table, shift, CharTable, TableDir, TableIdx};

use ndarray::Ix2;

use itertools::Itertools;

use std::str::FromStr;

use std::collections::HashSet;

impl AoC for Day {
    fn run(input: &str) -> anyhow::Result<AoCResult> {
        let parsed: Day = input.parse()?;

        Ok(AoCResult {
            part_a: Some(parsed.part_a()),
            part_b: Some(parsed.part_b()),
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Day {
    table: CharTable,
    start: TableIdx,
}

impl Day {
    fn drop_boulder(&self, pos: TableIdx) -> Option<Self> {
        let shape = into_shape(pos);
        if let Some('.') = self.table.get(shape) {
            let mut ret = self.clone();
            ret.table[shape] = '#';
            return Some(ret);
        }
        None
    }

    fn walk(&self) -> GuardPath {
        GuardPath {
            table: &self.table,
            guard: Guard {
                pos: self.start,
                dir: TableDir::new(0, -1),
            },
        }
    }

    fn is_stuck(&self) -> bool {
        let mut trace: HashSet<Guard> = HashSet::new();
        for guard in self.walk() {
            if ! trace.insert(guard) {
                return true;
            }
        }
        false
    }

    fn unique_guard_pos(&self) -> impl Iterator<Item = TableIdx> + '_ {
        self.walk().map(|x| x.pos).unique()
    }

    fn part_a(&self) -> usize {
        self.unique_guard_pos().count()
    }

    fn all_paradox_boulders(&self) -> impl Iterator<Item = TableIdx> + '_ {
        self.unique_guard_pos().filter(|dropped| {
            if let Some(day) = self.drop_boulder(*dropped) {
                if day.is_stuck() {
                    return true;
                }
            }
            false
        })
    }

    fn part_b(&self) -> usize {
        self.all_paradox_boulders().count()
    }
}

impl FromStr for Day {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Day> {
        let table = parse_char_table(s)?;

        let ((row, col), _) = table
            .indexed_iter()
            .find_or_last(|((_row, _col), c)| *c == &'^')
            .ok_or(anyhow!("couldn't find start position"))?;

        Ok(Day {
            table,
            start: into_idx(Ix2(row, col)),
        })
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
struct Guard {
    pos: TableIdx,
    dir: TableDir,
}

impl Guard {
    fn turned(&self) -> TableDir {
        self.dir.as_().rotated_z(std::f32::consts::PI * 0.5).as_()
    }

    fn turn(&mut self) {
        self.dir = self.turned();
    }

    fn step(&mut self) {
        self.pos = self.look()
    }

    fn look(&self) -> TableIdx {
        shift(self.pos, self.dir)
    }
}

struct GuardPath<'a> {
    table: &'a CharTable,
    guard: Guard,
}

impl GuardPath<'_> {
    fn get(&self, pos: TableIdx) -> Option<&char> {
        self.table.get(into_shape(pos))
    }

    fn get_guard_pos(&self) -> Option<&char> {
        self.get(self.guard.pos)
    }

    fn blocked(&self) -> bool {
        self.get(self.guard.look()) == Some(&'#')
    }
}

impl<'a> Iterator for GuardPath<'a> {
    type Item = Guard;

    fn next(&mut self) -> Option<Self::Item> {
        self.get_guard_pos()?;

        let retval = self.guard;

        if self.blocked() {
            self.guard.turn();
        }

        self.guard.step();

        Some(retval)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    use ndarray::arr2;

    #[fixture]
    fn example() -> &'static str {
        "\
            ....#.....\n\
            .........#\n\
            ..........\n\
            ..#.......\n\
            .......#..\n\
            ..........\n\
            .#..^.....\n\
            ........#.\n\
            #.........\n\
            ......#...\n\
        "
    }

    #[fixture]
    fn example_parsed() -> Day {
        Day {
            table: arr2(&[
                ['.', '.', '.', '.', '#', '.', '.', '.', '.', '.'],
                ['.', '.', '.', '.', '.', '.', '.', '.', '.', '#'],
                ['.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
                ['.', '.', '#', '.', '.', '.', '.', '.', '.', '.'],
                ['.', '.', '.', '.', '.', '.', '.', '#', '.', '.'],
                ['.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
                ['.', '#', '.', '.', '^', '.', '.', '.', '.', '.'],
                ['.', '.', '.', '.', '.', '.', '.', '.', '#', '.'],
                ['#', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
                ['.', '.', '.', '.', '.', '.', '#', '.', '.', '.'],
            ]),
            start: TableIdx::new(4, 6),
        }
    }

    #[rstest]
    fn parse_example_a(example: &'static str, example_parsed: Day) {
        let result: Day = example.parse().unwrap();
        assert_eq!(result, example_parsed)
    }

    #[rstest]
    fn trace_walk(example_parsed: Day) {
        let path: Vec<Guard> = example_parsed.walk().take(7).collect();
        assert_eq!(
            path,
            vec![
                Guard {
                    pos: TableIdx::new(4, 6),
                    dir: TableDir::new(0, -1)
                },
                Guard {
                    pos: TableIdx::new(4, 5),
                    dir: TableDir::new(0, -1)
                },
                Guard {
                    pos: TableIdx::new(4, 4),
                    dir: TableDir::new(0, -1)
                },
                Guard {
                    pos: TableIdx::new(4, 3),
                    dir: TableDir::new(0, -1)
                },
                Guard {
                    pos: TableIdx::new(4, 2),
                    dir: TableDir::new(0, -1)
                },
                Guard {
                    pos: TableIdx::new(4, 1),
                    dir: TableDir::new(0, -1)
                },
                Guard {
                    pos: TableIdx::new(5, 1),
                    dir: TableDir::new(1, 0)
                },
            ]
        )
    }

    #[rstest]
    fn test_part_a(example_parsed: Day) {
        assert_eq!(example_parsed.part_a(), 41)
    }

    #[rstest]
    fn test_all_obstacles(example_parsed: Day) {
        let obstacles: HashSet<TableIdx> = example_parsed.all_paradox_boulders().collect();
        assert_eq!(
            obstacles,
            [
                TableIdx::new(3, 6),
                TableIdx::new(6, 7),
                TableIdx::new(7, 7),
                TableIdx::new(1, 8),
                TableIdx::new(3, 8),
                TableIdx::new(7, 9)
            ]
            .into()
        )
    }

    #[rstest]
    fn test_part_b(example_parsed: Day) {
        assert_eq!(example_parsed.part_b(), 6)
    }
}

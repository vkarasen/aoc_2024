use crate::prelude::*;

use std::collections::HashSet;

use std::str::FromStr;

use rayon::prelude::*;

use crate::table::{
    cast_ray, from_pattern, into_idx, shift, into_shape, parse_char_table, CharTable, TableDir, TableIdx,
};

use ndarray::Array2;

type HeightMap = Array2<u8>;

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
    table: HeightMap
}

impl Day {
    fn get_all_trailheads(&self) -> impl ParallelIterator<Item = TableIdx> + '_ {
        self.table.indexed_iter().par_bridge().filter_map(|(pattern, val)| {
            if val == &0 {
                return Some(from_pattern(pattern));
            }
            None
        })
    }

    fn hike(&self, start: TableIdx) -> Hiker {
        Hiker {
            table: &self.table,
            stack: [(start, 0)].into(),
            visited: HashSet::new(),
        }
    }

    fn trailscore(&self, start: TableIdx) -> usize {
        self.hike(start).filter(|(_, height)| height == &9).count()
    }

    fn part_a(&self) -> usize {
        self.get_all_trailheads().map(|start| self.trailscore(start)).sum()
    }
}

impl FromStr for Day {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Day> {
        let table = (parse_char_table(s)?).map(|x| x.to_digit(10).unwrap() as u8);

        Ok(Day { table })
    }
}

struct Hiker<'a> {
    table: &'a HeightMap,
    stack: Vec<(TableIdx, u8)>,
    visited: HashSet<TableIdx>,
}

const ALL_CARD_DIRS: [TableDir; 4] = [
    TableDir::new(0, 1),
    TableDir::new(0, -1),
    TableDir::new(1, 0),
    TableDir::new(-1, 0),
];

impl<'a> Iterator for Hiker<'a> {
    type Item = (TableIdx, u8);

    fn next(&mut self) -> Option<Self::Item> {
        let (curpos, curheight) = self.stack.pop()?;

        self.visited.insert(curpos);

        for dir in ALL_CARD_DIRS {
            let candpos = shift(curpos, dir);
            if let Some(candheight) = self.table.get(into_shape(candpos)) {
                if ! self.visited.contains(&candpos) && *candheight == curheight + 1 {
                    self.stack.push((candpos, *candheight));
                }
            }
        }

        Some((curpos, curheight))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    use ndarray::arr2;

    #[fixture]
    fn small_example() -> &'static str {
        "\
        0123\n\
        1234\n\
        8765\n\
        9876\n\
        "
    }

    #[fixture]
    fn small_example_parsed() -> Day {
        Day {
            table: arr2(&[
                [0, 1, 2, 3],
                [1, 2, 3, 4],
                [8, 7, 6, 5],
                [9, 8, 7, 6],
            ]),
        }
    }

    #[rstest]
    fn parse_small_example_a(small_example: &'static str, small_example_parsed: Day) {
        let result: Day = small_example.parse().unwrap();
        assert_eq!(result, small_example_parsed)
    }

    #[rstest]
    fn test_small_example_a(small_example_parsed: Day) {
        assert_eq!(small_example_parsed.part_a(), 1);
    }
}

use crate::prelude::*;

use anyhow::anyhow;

use crate::table::{
    CharTable,
    parse_char_table,
    TableIdx, TableDir,
    cast_ray,
    into_idx,
    into_shape,
    shift
};

use ndarray::Ix2;

use itertools::Itertools;

use vek::Mat2;

use std::str::FromStr;

impl AoC for Day {
    fn run(input: &str) -> anyhow::Result<AoCResult> {
        let parsed: Day = input.parse()?;

        Ok(AoCResult {
            part_a : Some(parsed.part_a()),
            part_b : None
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Day {
    table: CharTable,
    start: TableIdx
}

impl Day {

    fn walk(&self) -> GuardPath {
        GuardPath {
            table: &self.table,
            pos: self.start,
            dir: TableDir::new(0, -1),
        }
    }

    fn part_a(&self) -> usize {
        self.walk().unique().count()
    }
}


impl FromStr for Day {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Day> {
        let table = parse_char_table(s)?;

        let ((row, col), _) = table.indexed_iter().find_or_last(|((_row, _col), c)|  *c == &'^').ok_or(anyhow!("couldn't find start position"))?;

        Ok(Day{ table, start: into_idx(Ix2(row, col))})
    }
}

struct GuardPath<'a> {
    table: &'a CharTable,
    pos: TableIdx,
    dir: TableDir,
}

impl <'a> Iterator for GuardPath<'a> {
    type Item = TableIdx;

    fn next(&mut self) -> Option<Self::Item> {

        self.table.get(into_shape(self.pos))?;

        let retval = self.pos;

        self.pos = shift(retval, self.dir);

        if let Some('#') = self.table.get(into_shape(self.pos)) {
            self.dir = self.dir.as_().rotated_z(std::f32::consts::PI*0.5).as_();
            self.pos = shift(retval, self.dir);
        }

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
                ['.', '.', '.', '.', '.', '.', '#', '.', '.', '.']
            ]),
            start: TableIdx::new(4, 6)
        }
    }

    #[rstest]
    fn parse_example_a(example: &'static str, example_parsed: Day) {
        let result: Day = example.parse().unwrap();
        assert_eq!(result, example_parsed)
    }

    #[rstest]
    fn trace_walk(example_parsed: Day) {
        let path : Vec<TableIdx> = example_parsed.walk().take(7).collect();
        assert_eq!(
            path,
            vec![
                TableIdx::new(4, 6),
                TableIdx::new(4, 5),
                TableIdx::new(4, 4),
                TableIdx::new(4, 3),
                TableIdx::new(4, 2),
                TableIdx::new(4, 1),
                TableIdx::new(5, 1),
            ])
    }

    #[rstest]
    fn test_part_a(example_parsed: Day) {
        assert_eq!(example_parsed.part_a(), 41)
    }
}

use crate::prelude::*;

use crate::table::{
    CharTable,
    parse_char_table,
    TableIdx, TableDir,
    cast_ray,
    into_idx
};

use ndarray::Ix2;

use std::str::FromStr;

use itertools::iproduct;


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
    table: CharTable
}

const XMAS: &str = "XMAS";
const ALL_DIRS: [TableDir; 8] = [
    TableDir::new(0, 1),
    TableDir::new(0, -1),
    TableDir::new(1, 1),
    TableDir::new(1, -1),
    TableDir::new(-1, 1),
    TableDir::new(-1, -1),
    TableDir::new(1, 0),
    TableDir::new(-1, 0),
];

impl Day {
    fn is_xmas(&self, origin: TableIdx, dir: TableDir) -> bool {
        //dbg!(&origin, &dir);
        let mut ray = cast_ray(&self.table, origin, dir).cloned().take(4);

        for c in XMAS.chars() {
            match ray.next() {
                None => { return false; },
                Some(v) if v != c => { return false; },
                _ => {}
            }

        }

        true
    }

    fn part_a(&self) -> usize {
        let (height, width) = self.table.dim();
        iproduct!(0..width, 0..height).flat_map(|(x, y)| {
            std::iter::repeat(into_idx(Ix2(x, y))).zip(ALL_DIRS.iter())
        }).map(|(origin, dir)| self.is_xmas(origin, *dir)).filter(|x| *x).count()
    }
}

impl FromStr for Day {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Day> {
        let table = parse_char_table(s)?;

        Ok(Day{ table})
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
        MMMSXXMASM\n\
        MSAMXMSMSA\n\
        AMXSXMAAMM\n\
        MSAMASMSMX\n\
        XMASAMXAMM\n\
        XXAMMXXAMA\n\
        SMSMSASXSS\n\
        SAXAMASAAA\n\
        MAMMMXMMMM\n\
        MXMXAXMASX\n\
        "
    }

    #[fixture]
    fn example_parsed() -> Day {
        Day {
            table: arr2(&[
                ['M', 'M', 'M', 'S', 'X', 'X', 'M', 'A', 'S', 'M'],
                ['M', 'S', 'A', 'M', 'X', 'M', 'S', 'M', 'S', 'A'],
                ['A', 'M', 'X', 'S', 'X', 'M', 'A', 'A', 'M', 'M'],
                ['M', 'S', 'A', 'M', 'A', 'S', 'M', 'S', 'M', 'X'],
                ['X', 'M', 'A', 'S', 'A', 'M', 'X', 'A', 'M', 'M'],
                ['X', 'X', 'A', 'M', 'M', 'X', 'X', 'A', 'M', 'A'],
                ['S', 'M', 'S', 'M', 'S', 'A', 'S', 'X', 'S', 'S'],
                ['S', 'A', 'X', 'A', 'M', 'A', 'S', 'A', 'A', 'A'],
                ['M', 'A', 'M', 'M', 'M', 'X', 'M', 'M', 'M', 'M'],
                ['M', 'X', 'M', 'X', 'A', 'X', 'M', 'A', 'S', 'X']
            ])
        }
    }

    #[rstest]
    fn parse_example_a(example: &'static str, example_parsed: Day) {
        let result: Day = example.parse().unwrap();
        assert_eq!(result, example_parsed)
    }

    #[rstest]
    #[case(TableIdx::new(4, 0), TableDir::new(1, 1))]
    #[case(TableIdx::new(5, 0), TableDir::new(1, 0))]
    #[case(TableIdx::new(9, 9), TableDir::new(0, -1))]
    #[case(TableIdx::new(9, 9), TableDir::new(-1, -1))]
    #[case(TableIdx::new(9, 3), TableDir::new(-1, 1))]
    fn test_xmas_ray(example_parsed: Day, #[case] origin: TableIdx, #[case] dir: TableDir) {
        assert!(example_parsed.is_xmas(origin, dir))
    }

    #[rstest]
    fn test_part_a(example_parsed: Day) {
        assert_eq!(example_parsed.part_a(), 18)
    }
}

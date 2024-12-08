use crate::prelude::*;

use std::str::FromStr;

use std::collections::HashMap;

use itertools::{Itertools, iproduct};

use crate::table::{
    into_idx, cast_ray, parse_char_table, CharTable, TableDir, TableIdx,
};

use ndarray::Ix2;

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
    table: CharTable,
    antennas: HashMap<char, Vec<TableIdx>>,
}

impl Day {
    fn is_antinode_for(&self, pos: TableIdx) -> impl Iterator<Item=char> + '_ {
        self.antennas.iter().filter_map(move |(a, v)| {
            for x in v.iter().combinations(2) {
                let mut dists : Vec<TableDir> = vec![x[0].as_() - pos.as_(), x[1].as_() - pos.as_()];
                dists.sort_by_key(|a| a.magnitude_squared());

                //dbg!(&dists);

                if  2*dists[0].as_() == dists[1].as_() {
                    //dbg!(a, &pos);
                    return Some(*a);
                }
            }
            None
        })
    }

    fn part_a(&self) -> usize {
        let (height, width) = self.table.dim();
        iproduct!(0..width, 0..height).filter(|(x, y)| {
            let idx = into_idx(Ix2(*x, *y));
            self.is_antinode_for(idx).next().is_some()
        }).count()
    }

    fn part_b(&self) -> usize {
        self.antennas.values().flat_map(|v| {
            v.iter().combinations(2).flat_map(|dvec| {
                let dir = dvec[1].as_() - dvec[0].as_();
                cast_ray(&self.table, *dvec[0], dir).chain(cast_ray(&self.table, *dvec[1], -dir)).map(|(pos, _v)| pos)
            })
        }).unique().count()
    }
}

impl FromStr for Day {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Day> {
        let table = parse_char_table(s)?;

        let mut antennas: HashMap<char, Vec<TableIdx>> = HashMap::new();

        for (shape, sym) in table.indexed_iter() {
            if *sym != '.' {
                let idx = into_idx(Ix2(shape.0, shape.1));
                if let Some(v) = antennas.get_mut(sym) {
                    v.push(idx);
                } else {
                    antennas.insert(*sym, vec![idx]);
                }
            }
        }

        Ok(Day { table, antennas })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    use std::collections::HashSet;

    use ndarray::arr2;

    #[fixture]
    fn example() -> &'static str {
        "\
            ............\n\
            ........0...\n\
            .....0......\n\
            .......0....\n\
            ....0.......\n\
            ......A.....\n\
            ............\n\
            ............\n\
            ........A...\n\
            .........A..\n\
            ............\n\
            ............\n\
        "
    }

    #[fixture]
    fn example_parsed() -> Day {
        Day {
            table: arr2(&[
                ['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
                ['.', '.', '.', '.', '.', '.', '.', '.', '0', '.', '.', '.'],
                ['.', '.', '.', '.', '.', '0', '.', '.', '.', '.', '.', '.'],
                ['.', '.', '.', '.', '.', '.', '.', '0', '.', '.', '.', '.'],
                ['.', '.', '.', '.', '0', '.', '.', '.', '.', '.', '.', '.'],
                ['.', '.', '.', '.', '.', '.', 'A', '.', '.', '.', '.', '.'],
                ['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
                ['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
                ['.', '.', '.', '.', '.', '.', '.', '.', 'A', '.', '.', '.'],
                ['.', '.', '.', '.', '.', '.', '.', '.', '.', 'A', '.', '.'],
                ['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
                ['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            ]),
            antennas: [
                (
                    '0',
                    [
                        TableIdx { x: 8, y: 1 },
                        TableIdx { x: 5, y: 2 },
                        TableIdx { x: 7, y: 3 },
                        TableIdx { x: 4, y: 4 },
                    ].into(),
                ),
                (
                    'A',
                    [
                        TableIdx { x: 6, y: 5 },
                        TableIdx { x: 8, y: 8 },
                        TableIdx { x: 9, y: 9 },
                    ].into(),
                ),
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
    #[case(TableIdx { x: 3, y: 1 }, vec!['A', '0'])]
    #[case(TableIdx { x: 6, y: 0 }, vec!['0'])]
    #[case(TableIdx { x: 0, y: 0 }, vec![])]
    fn test_antinodes(example_parsed: Day, #[case] pos: TableIdx, #[case] expected: Vec<char>) {
        let antinodes: HashSet<char> = example_parsed.is_antinode_for(pos).collect();
        assert_eq!(
            antinodes,
            expected.into_iter().collect()
        )
    }

    #[rstest]
    fn test_part_a(example_parsed: Day) {
        assert_eq!(example_parsed.part_a(), 14)
    }
    #[rstest]
    fn test_part_b(example_parsed: Day) {
        assert_eq!(example_parsed.part_b(), 34)
    }
}

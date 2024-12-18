use crate::prelude::*;

use std::str::FromStr;

use anyhow::anyhow;

use bimap::BiMap;

use crate::table::{
    from_pattern, into_shape, parse_char_table, shift, CharTable, PPCharTable, TableDir, TableIdx,
};

use nom::{bytes::complete::take_until, error::Error, Finish};

use lazy_static::lazy_static;

lazy_static! {
    static ref CHAR_MAP: BiMap<char, TableDir> = BiMap::from_iter(
        [
            ('<', TableDir::new(-1, 0)),
            ('^', TableDir::new(0, -1)),
            ('>', TableDir::new(1, 0)),
            ('v', TableDir::new(0, 1)),
        ]
        .into_iter()
    );
}

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
    moves: Vec<TableDir>,
}

fn gps_coordinates(table: &CharTable) -> impl Iterator<Item = (TableIdx, usize)> + '_ {
    table.indexed_iter().filter_map(|(pattern, c)| {
        if c == &'O' || c == &'[' {
            let pos = from_pattern(pattern);
            return Some((pos, pos.x + pos.y * 100));
        }
        None
    })
}

fn thicc_table(table: &CharTable) -> CharTable {
    let mut cells = Vec::new();
    for cell in table.iter() {
        let ex: &str = match *cell {
            '#' => "##",
            'O' => "[]",
            '.' => "..",
            '@' => "@.",
            _ => unreachable!(),
        };
        cells.extend(ex.chars());
    }
    let original_shape = table.dim();
    CharTable::from_shape_vec((original_shape.0, 2 * original_shape.1), cells).unwrap()
}

fn find_start_pos(table: &CharTable) -> TableIdx {
    for (pattern, c) in table.indexed_iter() {
        if c == &'@' {
            return from_pattern(pattern);
        }
    }
    panic!("couldn't find start position");
}

impl Day {
    fn part_a(&self) -> usize {
        let mut table = self.table.clone();
        let it = self.walk(&mut table);
        for _afterpos in it {
            //dbg!(&afterpos);
        }
        gps_coordinates(&table).map(|(_pos, gps)| gps).sum()
    }

    fn part_b(&self) -> usize {
        let mut table = thicc_table(&self.table);
        let it = self.thicc_walk(&mut table);
        for _afterpos in it {
            //dbg!(&afterpos);
        }
        gps_coordinates(&table).map(|(_pos, gps)| gps).sum()
    }

    fn walk<'a>(&'a self, table: &'a mut CharTable) -> Robot<'a> {
        let pos = find_start_pos(table);
        let last_board = table.clone();
        Robot {
            table,
            pos,
            dirs: self.moves.clone().into_iter().rev().collect(),
            shape: Vec::new(),
            visit: Vec::new(),
            last_board
        }
    }

    fn thicc_walk<'a>(&'a self, table: &'a mut CharTable) -> Robot<'a> {
        let pos = find_start_pos(table);
        let last_board = table.clone();
        Robot {
            table,
            pos,
            dirs: self.moves.clone().into_iter().rev().collect(),
            shape: Vec::new(),
            visit: Vec::new(),
            last_board
        }
    }
}

impl FromStr for Day {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Day> {
        let (input, tablestring) = take_until("\n\n")(s)
            .finish()
            .map_err(|Error { input, code }| anyhow!("input: {}, code: {:?}", input, code))
            .unwrap();
        let table = parse_char_table(tablestring)?;
        let moves = input
            .chars()
            .filter_map(|c| {
                let x = CHAR_MAP.get_by_left(&c)?;
                Some(*x)
            })
            .collect();
        Ok(Day { table, moves })
    }
}

#[derive(Debug, Copy, Clone)]
struct Elem {
    pos: TableIdx,
    shape: char,
}

#[derive(Debug)]
struct Robot<'a> {
    table: &'a mut CharTable,
    pos: TableIdx,
    dirs: Vec<TableDir>,
    shape: Vec<Elem>,
    visit: Vec<Elem>,
    last_board: CharTable,
}

impl<'a> Iterator for Robot<'a> {
    type Item = TableIdx;

    fn next(&mut self) -> Option<Self::Item> {
        let dir = self.dirs.pop()?;
        let dirchar = *CHAR_MAP.get_by_right(&dir).unwrap();
        self.shape.clear();
        self.visit.clear();
        //dbg!(&dirchar);

        self.visit.push(Elem {
            pos: self.pos,
            shape: '@',
        });
        let mut can_move = true;
        while let Some(cur) = self.visit.pop() {
            match (cur.shape, dirchar) {
                ('@' | 'O', _) | ('[' | ']', '<' | '>')  => {
                    self.shape.push(cur);
                    let look = shift(cur.pos, dir);
                    self.visit.push(Elem {
                        pos: look,
                        shape: *self.table.get(into_shape(look)).unwrap(),
                    });
                }
                ('#', _) => {
                    can_move = false;
                    break;
                }
                ('.', _) => {}
                ('[', _)  => {
                    self.shape.push(cur);
                    let look = shift(cur.pos, dir);
                    self.visit.push(Elem {
                        pos: look,
                        shape: *self.table.get(into_shape(look)).unwrap(),
                    });
                    let comrade = Elem { pos : shift(cur.pos, TableDir::new(1, 0)), shape : ']' };
                    self.shape.push(comrade);
                    let look = shift(comrade.pos, dir);
                    self.visit.push(Elem {
                        pos: look,
                        shape: *self.table.get(into_shape(look)).unwrap(),
                    });
                }
                (']', _)  => {
                    self.shape.push(cur);
                    let look = shift(cur.pos, dir);
                    self.visit.push(Elem {
                        pos: look,
                        shape: *self.table.get(into_shape(look)).unwrap(),
                    });
                    let comrade = Elem { pos : shift(cur.pos, TableDir::new(-1, 0)), shape : '[' };
                    self.shape.push(comrade);
                    let look = shift(comrade.pos, dir);
                    self.visit.push(Elem {
                        pos: look,
                        shape: *self.table.get(into_shape(look)).unwrap(),
                    });
                }
                _ => unreachable!(),
            }
        }
        let make_print = self.shape.len() > 5;
        if can_move {
            while let Some(cur) = self.shape.pop() {
                let look = shift(cur.pos, dir);
                *self.table.get_mut(into_shape(look)).unwrap() = cur.shape;
                *self.table.get_mut(into_shape(cur.pos)).unwrap() = '.';
                if self.shape.is_empty() {
                    self.pos = look;
                }
            }
        }
        let pptable = self.table.clone();
        if make_print {
            let pp: PPCharTable = (&pptable).into();
            let pp_last: PPCharTable = (&self.last_board).into();
            dbg!(&pp_last);
            dbg!(&dirchar, &pp);
        }
        self.last_board = pptable;
        Some(self.pos)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    fn example() -> &'static str {
        "\
            ##########\n\
            #..O..O.O#\n\
            #......O.#\n\
            #.OO..O.O#\n\
            #..O@..O.#\n\
            #O#..O...#\n\
            #O..O..O.#\n\
            #.OO.O.OO#\n\
            #....O...#\n\
            ##########\n\
            \n\
            <vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^\n\
            vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v\n\
            ><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<\n\
            <<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^\n\
            ^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><\n\
            ^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^\n\
            >^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^\n\
            <><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>\n\
            ^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>\n\
            v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^\n\
        "
    }

    #[fixture]
    fn small_example() -> &'static str {
        "\
            ########\n\
            #..O.O.#\n\
            ##@.O..#\n\
            #...O..#\n\
            #.#.O..#\n\
            #...O..#\n\
            #......#\n\
            ########\n\
            \n\
            <^^>>>vv<v>>v<<\n\
        "
    }

    #[fixture]
    fn tiny_example() -> &'static str {
        "\
            #######\n\
            #...#.#\n\
            #.....#\n\
            #..OO@#\n\
            #..O..#\n\
            #.....#\n\
            #######\n\
            \n\
            <vv<<^^<<^^\n\
        "
    }

    #[fixture]
    fn example_parsed(example: &'static str) -> Day {
        example.parse().unwrap()
    }

    #[fixture]
    fn small_example_parsed(small_example: &'static str) -> Day {
        small_example.parse().unwrap()
    }

    #[fixture]
    fn tiny_example_parsed(small_example: &'static str) -> Day {
        small_example.parse().unwrap()
    }

    #[rstest]
    #[case(example_parsed(example()), 10092)]
    #[case(small_example_parsed(small_example()), 2028)]
    fn test_part_a(#[case] test: Day, #[case] expected: usize) {
        assert_eq!(test.part_a(), expected)
    }

    #[rstest]
    #[case(example_parsed(example()), 9021)]
    #[case(tiny_example_parsed(tiny_example()), 618)]
    fn test_part_b(#[case] test: Day, #[case] expected: usize) {
        assert_eq!(test.part_b(), expected)
    }
}

use crate::prelude::*;

use std::str::FromStr;

use rayon::prelude::*;

use crate::table::{parse_char_table, shift, TableDir, TableIdx, from_pattern, into_shape};

use petgraph::{algo, prelude::*};

use itertools::iproduct;

use bimap::BiMap;

type TrailGraph = DiGraph<(), ()>;

type NodeMap = BiMap<NodeIndex, TableIdx>;

impl AoC for Day {
    fn run(input: &str) -> anyhow::Result<AoCResult> {
        let parsed: Day = input.parse()?;

        Ok(AoCResult {
            part_a: Some(parsed.part_a()),
            part_b: Some(parsed.part_b()),
        })
    }
}

#[derive(Debug)]
pub struct Day {
    trails: Vec<(TableIdx, TableIdx, usize)>
}

const ALL_CARD_DIRS: [TableDir; 4] = [
    TableDir::new(0, 1),
    TableDir::new(0, -1),
    TableDir::new(1, 0),
    TableDir::new(-1, 0),
];

impl Day {
    fn part_a(&self) -> usize {
        self.trails.len()
    }

    fn part_b(&self) -> usize {
        self.trails.iter().map(|x| x.2).sum()
    }
}

fn get_node_or_insert(pos: &TableIdx, graph: &mut TrailGraph, nodemap: &mut NodeMap) -> NodeIndex {
    if let Some(n) = nodemap.get_by_right(pos) {
        *n
    } else {
        let n = graph.add_node(());
        nodemap.insert(n, *pos);
        n
    }
}


impl FromStr for Day {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Day> {
        let table = (parse_char_table(s)?).map(|x| x.to_digit(10).unwrap() as u8);
        let mut nodemap = BiMap::new();
        let mut graph = TrailGraph::new();
        let mut trailheads = Vec::new();
        let mut peaks = Vec::new();

        for (x, height) in table.indexed_iter() {
            let curpos = from_pattern(x);
            let curnode = get_node_or_insert(&curpos, &mut graph, &mut nodemap);
            nodemap.insert(curnode, curpos);
            if height == &0 {
                trailheads.push(curnode);
            } else if height == &9 {
                peaks.push(curnode);
            }

            for dir in &ALL_CARD_DIRS {
                let neighborpos = shift(curpos, *dir);
                if let Some(neighborheight) = table.get(into_shape(neighborpos)) {
                    if height + 1 == *neighborheight {
                        let neighbornode = get_node_or_insert(&neighborpos, &mut graph, &mut nodemap);
                        graph.add_edge(curnode, neighbornode, ());
                    }
                }
            }
        }

        let trails = iproduct!(trailheads.iter(), peaks.iter()).par_bridge().filter_map(|(head, peak)| {

            let ways = algo::all_simple_paths::<Vec<_>,_>(&graph, *head, *peak, 1, None).collect::<Vec<_>>();

            if ways.is_empty() {
                None
            } else {
                Some((*nodemap.get_by_left(head).unwrap(), *nodemap.get_by_left(peak).unwrap(), ways.len()))
            }
        }).collect();

        Ok(Day {
            trails,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

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
    fn larger_example() -> &'static str {
        "\
        89010123\n\
        78121874\n\
        87430965\n\
        96549874\n\
        45678903\n\
        32019012\n\
        01329801\n\
        10456732\n\
        "
    }

    #[rstest]
    fn test_small_example_a(small_example: &'static str) {
        assert_eq!(small_example.parse::<Day>().unwrap().part_a(), 1);
    }

    #[rstest]
    fn test_larger_example_a(larger_example: &'static str) {
        assert_eq!(larger_example.parse::<Day>().unwrap().part_a(), 36);
    }

    #[rstest]
    fn test_larger_example_b(larger_example: &'static str) {
        assert_eq!(larger_example.parse::<Day>().unwrap().part_b(), 81);
    }
}

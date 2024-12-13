use crate::prelude::*;

use std::str::FromStr;

use crate::table::{from_pattern, into_shape, parse_char_table, shift, CharTable, TableDir};

use crate::graph::{get_node_or_insert, NodeMap};

use petgraph::{algo::kosaraju_scc, prelude::*};

type PlotGraph = UnGraph<(), ()>;

impl AoC for Day {
    fn run(input: &str) -> anyhow::Result<AoCResult> {
        let parsed: Day = input.parse()?;

        Ok(AoCResult {
            part_a: Some(parsed.part_a()),
            part_b: Some(parsed.part_b()),
        })
    }
}

const ALL_CARD_DIRS: [TableDir; 4] = [
    TableDir::new(0, 1),
    TableDir::new(0, -1),
    TableDir::new(1, 0),
    TableDir::new(-1, 0),
];

const ALL_DIAGS: [TableDir; 4] = [
    TableDir::new(1, 1),
    TableDir::new(-1, -1),
    TableDir::new(1, -1),
    TableDir::new(-1, 1),
];

#[derive(Debug)]
pub struct Day {
    table: CharTable,
    graph: PlotGraph,
    regions: Vec<Vec<NodeIndex>>,
    nodemap: NodeMap,
}

impl Day {
    fn part_a(&self) -> usize {
        self.regions
            .iter()
            .map(|r| {
                let area = r.len();
                let perimeter: usize = r
                    .iter()
                    .map(|n| ALL_CARD_DIRS.len() - self.graph.edges(*n).count())
                    .sum();

                area * perimeter
            })
            .sum()
    }

    fn part_b(&self) -> usize {
        self.regions
            .iter()
            .enumerate()
            .map(|(idx, r)| {
                let area = r.len();
                let corners: usize = r
                    .iter()
                    .map(|node| {
                        let nodepos = self.nodemap.get_by_left(node).unwrap();
                        let neighbordirs: Vec<TableDir> = self
                            .graph
                            .neighbors(*node)
                            .map(|neigh| {
                                let neighborpos = self.nodemap.get_by_left(&neigh).unwrap();
                                neighborpos.as_() - nodepos.as_()
                            })
                            .collect();

                        let check_diag = |diag| {
                            let diagpos = shift(*nodepos, diag);
                            let diag = self.table.get(into_shape(diagpos))?;

                            if !self.regions[idx]
                                .contains(self.nodemap.get_by_right(&diagpos).unwrap())
                            {
                                return Some(false);
                            }

                            Some(self.table.get(into_shape(*nodepos)).unwrap() == diag)
                        };

                        let count_different_diag_regions = |x: &[TableDir]| -> usize {
                            x
                                .iter()
                                .filter_map(|dir| {
                                    if let Some(false) = check_diag(*dir) {
                                        Some(())
                                    } else {
                                        None
                                    }
                                })
                                .count()};

                        match neighbordirs.len() {
                            0 => 4,
                            1 => 2,
                            2 => {
                                if neighbordirs[0].dot(neighbordirs[1]) == 0 {
                                    if check_diag(neighbordirs[0] + neighbordirs[1]).unwrap() {
                                        1
                                    } else {
                                        2
                                    }
                                } else {
                                    0
                                }
                            }
                            3 => {
                                let (center, tail) = match (
                                    neighbordirs[0].dot(neighbordirs[1]).abs(),
                                    neighbordirs[0].dot(neighbordirs[2]).abs(),
                                ) {
                                    (0,0) => (neighbordirs[0], neighbordirs[1]),
                                    (1,0) => (neighbordirs[2], neighbordirs[0]),
                                    (0,1) => (neighbordirs[1], neighbordirs[0]),
                                        _ => unreachable!()
                                };

                                count_different_diag_regions(&[center + tail, center - tail])
                            }
                            4 => count_different_diag_regions(&ALL_DIAGS),
                            _ => unreachable!(),
                        }
                    })
                    .sum();

                area * corners
            })
            .sum()
    }
}

impl FromStr for Day {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Day> {
        let table = parse_char_table(s)?;
        let mut graph: PlotGraph = Graph::new_undirected();
        let mut nodemap = NodeMap::new();

        for (x, curplot) in table.indexed_iter() {
            let curpos = from_pattern(x);
            let curnode = get_node_or_insert(&curpos, &mut graph, &mut nodemap);

            for dir in &ALL_CARD_DIRS {
                let neighborpos = shift(curpos, *dir);
                if let Some(neighborplot) = table.get(into_shape(neighborpos)) {
                    if curplot == neighborplot {
                        let neighbornode =
                            get_node_or_insert(&neighborpos, &mut graph, &mut nodemap);
                        graph.update_edge(curnode, neighbornode, ());
                    }
                }
            }
        }
        let regions = kosaraju_scc(&graph);
        Ok(Day {
            table,
            graph,
            regions,
            nodemap,
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
        AAAA\n\
        BBCD\n\
        BBCC\n\
        EEEC\n\
        "
    }

    #[fixture]
    fn xo_example() -> &'static str {
        "\
        OOOOO\n\
        OXOXO\n\
        OOOOO\n\
        OXOXO\n\
        OOOOO\n\
        "
    }

    #[fixture]
    fn ex_example() -> &'static str {
        "\
        EEEEE\n\
        EXXXX\n\
        EEEEE\n\
        EXXXX\n\
        EEEEE\n\
        "
    }

    #[fixture]
    fn ab_example() -> &'static str {
        "\
        AAAAAA\n\
        AAABBA\n\
        AAABBA\n\
        ABBAAA\n\
        ABBAAA\n\
        AAAAAA\n\
        "
    }

    #[fixture]
    fn small_example_parsed(small_example: &'static str) -> Day {
        small_example.parse().unwrap()
    }

    #[fixture]
    fn xo_example_parsed(xo_example: &'static str) -> Day {
        xo_example.parse().unwrap()
    }

    #[fixture]
    fn ex_example_parsed(ex_example: &'static str) -> Day {
        ex_example.parse().unwrap()
    }

    #[fixture]
    fn ab_example_parsed(ab_example: &'static str) -> Day {
        ab_example.parse().unwrap()
    }

    #[rstest]
    fn test_small_example_part_a(small_example_parsed: Day) {
        assert_eq!(small_example_parsed.part_a(), 140)
    }

    #[rstest]
    fn test_xo_example_part_a(xo_example_parsed: Day) {
        assert_eq!(xo_example_parsed.part_a(), 772)
    }

    #[rstest]
    fn test_small_example_part_b(small_example_parsed: Day) {
        assert_eq!(small_example_parsed.part_b(), 80)
    }

    #[rstest]
    fn test_xo_example_part_b(xo_example_parsed: Day) {
        assert_eq!(xo_example_parsed.part_b(), 436)
    }

    #[rstest]
    fn test_ex_example_part_b(ex_example_parsed: Day) {
        assert_eq!(ex_example_parsed.part_b(), 236)
    }

    #[rstest]
    fn test_ab_example_part_b(ab_example_parsed: Day) {
        assert_eq!(ab_example_parsed.part_b(), 368)
    }
}

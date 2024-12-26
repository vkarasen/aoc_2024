use crate::prelude::*;

use std::str::FromStr;

use crate::table::{
    from_pattern, into_shape, parse_char_table, shift, CharTable, PPCharTable, TableDir, TableIdx,
};

use std::collections::VecDeque;


impl AoC for Day {
    fn run(input: &str) -> anyhow::Result<AoCResult> {
        let parsed: Day = input.parse()?;

        Ok(AoCResult {
            part_a : None,
            part_b : None
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Day {
    table : CharTable
}

impl Day {
}

impl FromStr for Day {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Day> {
        let table = parse_char_table(s)?;
        Ok(Day { table })
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    fn example() -> &'static str {
        "\
        "
    }

    #[fixture]
    fn example_parsed(example: &'static str) -> Day {
        example.parse().unwrap()
    }

    #[rstest]
    fn parse_example_a(example: &'static str, example_parsed: Day) {
        let result: Day = example.parse().unwrap();
        assert_eq!(result, example_parsed)
    }
}

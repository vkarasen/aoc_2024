use clap::{Parser, ValueEnum};
use strum_macros::Display;

use std::path::PathBuf;
use std::fs::File;
use anyhow::Result;

mod prelude {
    use nom::{
        character::complete::digit1,
        combinator::map_res,
        IResult,
    };

    pub struct AoCResult {
        pub part_a : Option<usize>,
        pub part_b : Option<usize>
    }

    pub trait AoC {
        fn run(input: &str) -> anyhow::Result<AoCResult>;
    }

    pub fn parse_usize(input: &str) -> IResult<&str, usize> {
        map_res(digit1, str::parse)(input)
    }
}

use crate::prelude::AoC;

mod table;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

#[derive(ValueEnum, Clone, Debug, Display)]
enum Days {
    Day1,
    Day2,
    Day3,
    Day4,
    Day5,
    Day6,
}

fn run_day(day: Days, input: &str) -> Result<()> {
    let result = match day {
        Days::Day1 => crate::day1::Day::run(input),
        Days::Day2 => crate::day2::Day::run(input),
        Days::Day3 => crate::day3::Day::run(input),
        Days::Day4 => crate::day4::Day::run(input),
        Days::Day5 => crate::day5::Day::run(input),
        Days::Day6 => crate::day6::Day::run(input),
    }?;

    if let Some(val) = result.part_a {
        println!("part a: {}", val);
    }

    if let Some(val) = result.part_b {
        println!("part b: {}", val);
    }

    Ok(())

}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(value_enum)]
    day: Days,

    #[arg(short, long, default_value = "./input/")]
    input: PathBuf
}

fn main() -> Result<()> {
    let args = Args::parse();

    let inputfilepath = {
        if args.input.is_dir() {
            args.input.join(args.day.to_string().to_lowercase()).with_extension("txt")
        } else {
            args.input
        }
    };

    let inputfile = File::open(&inputfilepath)?;

    let inputstr = std::io::read_to_string(&inputfile)?;

    run_day(args.day, &inputstr)

}

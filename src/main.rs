use clap::{Parser, ValueEnum};
use strum_macros::Display;

use std::path::PathBuf;
use std::fs::File;
use anyhow::Result;

mod prelude {
    pub struct AoCResult {
        pub part_a : Option<usize>,
        pub part_b : Option<usize>
    }

    pub trait AoC {
        fn run(input: &str) -> anyhow::Result<AoCResult>;
    }
}

use crate::prelude::AoC;

mod day1;

#[derive(ValueEnum, Clone, Debug, Display)]
enum Days {
    Day1
}

fn run_day(day: Days, input: &str) -> Result<()> {
    let result = match day {
        Days::Day1 => crate::day1::Day1::run(input)
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
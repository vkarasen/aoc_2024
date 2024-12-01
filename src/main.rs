use clap::{Parser, ValueEnum};
use strum_macros::Display;

use std::path::PathBuf;
use std::fs::File;
use anyhow::Result;

mod prelude {
    pub trait AoC {
        fn run(input: &str) -> anyhow::Result<()>;
    }
}


mod day1;
use crate::prelude::AoC;

#[derive(ValueEnum, Clone, Debug, Display)]
enum Days {
    Day1
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

    println!("using input from {:?}", &inputfilepath);

    let inputfile = File::open(&inputfilepath)?;

    let inputstr = std::io::read_to_string(&inputfile)?;

    match args.day {
        Days::Day1 => crate::day1::Day1::run(&inputstr)
    }

}

use crate::days::{dispatch_to_day, AoCArgs};

pub mod days;

const HELP: &str = "\
ADvent of Code 2024 Solutions

Attempting to complete it, in Rust no less

USAGE:
    aoc2024 [Options] DAY

FLAGS:
  -h, --help            Prints help information

OPTIONS:
  --part NUMBER         Select an optional puzzle part to solve

ARGS:
  Day:                  Select which days puzzle to solve
  Input:                Path to the puzzle input
";

fn parse_args() -> Result<AoCArgs, pico_args::Error> {
    let mut pargs = pico_args::Arguments::from_env();
    if pargs.contains(["-h", "--help"]) {
        print!("{}", HELP);
        std::process::exit(0);
    }
    let args = AoCArgs {
        part: pargs.opt_value_from_str("--part")?,
        day: pargs.free_from_str()?,
        input: pargs.free_from_str()?,
    };
    let leftovers = pargs.finish();
    if !leftovers.is_empty() {
        eprintln!("Warning: unused arguments: {:?}.", leftovers);
    }
    Ok(args)
}

fn main() {
    let args = match parse_args() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error: {}.", e);
            std::process::exit(1);
        }
    };
    if let Err(e) = dispatch_to_day(&args) {
        println!("Caught error {} processing day {}", e, args.day)
    }
}

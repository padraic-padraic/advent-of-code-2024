use std::result::Result;

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;

#[derive(PartialEq, Debug)]
pub enum PartSelection {
    All,
    PartOne,
    PartTwo,
}

#[derive(Debug)]
pub struct AoCArgs {
    pub day: u8,
    pub part: Option<u8>,
    pub input: std::path::PathBuf,
}


pub fn dispatch_to_day(args: &AoCArgs) -> Result<(), String> {
    if (args.day > 24 || args.day == 0) {
        return Err(String::from("Day must be in the range 1-24"));
    }
    let part_selection = match args.part {
        None => Some(PartSelection::All),
        Some(1u8) => Some(PartSelection::PartOne),
        Some(2u8) => Some(PartSelection::PartTwo),
        _ => None,
    };
    if let Some(parts) = part_selection {
        let input = args.input.as_path();
        let e_msg = format!("Day {} not implemented yet!", args.day);
        if parts == PartSelection::All || parts == PartSelection::PartOne {
            match args.day {
                1 => day01::part1(input)?,
                2 => day02::part1(input)?,
                3 => day03::part1(input)?,
                4 => day04::part1(input)?,
                5 => day05::part1(input)?,
                _ => return Err(e_msg)
            };
        }
        if parts == PartSelection::All || parts == PartSelection::PartTwo {
            match args.day {
                1 => day01::part2(input)?,
                2 => day02::part2(input)?,
                3 => day03::part2(input)?,
                4 => day04::part2(input)?,
                5 => day05::part2(input)?,
                _ => return Err(e_msg)
            };
        };
        Ok(())
    } else {
        Err(String::from("If specified, part must be one or two!"))
    }
}

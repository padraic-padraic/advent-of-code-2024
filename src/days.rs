use std::result::Result;

pub mod day01;
pub mod day02;

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
        match args.day {
            1 => day01::solution(args.input.as_path(), parts),
            2 => day02::solution(args.input.as_path(), parts),
            _ => Err(format!("Day {} not yet implemented.", args.day)),
        }
    } else {
        Err(String::from("If specified, part must be one or two!"))
    }
}

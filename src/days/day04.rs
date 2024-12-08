use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use super::PartSelection;

enum Dir {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW
}

fn get_coordinates()

fn part1(words: &[String]) -> Result<(), String> {
    Ok(())
}

pub fn solution(input: &std::path::Path, part: PartSelection) -> Result<(), String> {
    let f = match File::open(input) {
        Ok(f) => {f}
        Err(message)  => {
            println!("Unable to open {:?} due to {}", input, message);
            return Err("IO/Error".to_string());
        }
    };
    let mut reader = BufReader::new(f);
    let words: Vec<String> = reader.lines().map(|s| s.trim().collect();
)
    Ok(())
}
use std::io::BufReader;
use std::fs::File;
use utf8_chars::BufReadCharsExt;

use super::PartSelection;

#[derive(Debug)]
enum ValidInstruction {
    CharM,
    CharU,
    CharL,
    LBracket,
    LDigit,
    Comma,
    RDigit,
    Start,
    CharD,
    CharO,
    CharN,
    CharT,
    CharApos
}


const TEST: &str = "\
xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
";

fn part1(input: &std::path::Path) -> Result<(), String> {
    let f = match File::open(input) {
        Ok(f) => {f}
        Err(message)  => {
            println!("Unable to open {:?} due to {}", input, message);
            return Err("IO/Error".to_string());
        }
    };
    let mut reader = BufReader::new(f);
    let mut state = ValidInstruction::Start;
    let mut l_digits: String = String::new();
    let mut r_digits: String = String::new();
    let mut result = 0;
    for c in reader.chars() {
        let char = c.unwrap();
        state = match (state, char) {
            (ValidInstruction::Start, 'm') => {
                ValidInstruction::CharM
            }
            (ValidInstruction::CharM, 'u') => {
                ValidInstruction::CharU
            }
            (ValidInstruction::CharU, 'l') => {
                ValidInstruction::CharL
            }
            (ValidInstruction::CharL, '(') => {
                ValidInstruction::LBracket
            }
            (ValidInstruction::LBracket, '0'..='9') => {
                l_digits.push(char);
                ValidInstruction::LDigit
            }
            (ValidInstruction::LDigit, '0'..='9') => {
                l_digits.push(char);
                ValidInstruction::LDigit
            }
            (ValidInstruction::LDigit, ',') => {
                ValidInstruction::Comma
            }
            (ValidInstruction::Comma | ValidInstruction::RDigit, '0'..='9') => {
                r_digits.push(char);
                ValidInstruction::RDigit
            }
            (ValidInstruction::RDigit, ')') => {
                // println!("Finished, found numbers {}, {}", l_digits, r_digits);
                match (l_digits.parse::<i64>(), r_digits.parse::<i64>()) {
                    (Ok(l), Ok(r)) => {
                        // println!("Parsed numbers as {}, {}", l, r);
                        result += l * r;
                    }
                    _ => {
                        panic!("Error parsing the digits {}, {}", l_digits, r_digits);
                    }
                };
                l_digits.clear();
                r_digits.clear();
                ValidInstruction::Start
            }
            _ => {
                l_digits.clear();
                r_digits.clear();
                ValidInstruction::Start
            }
        }
    };
    println!("Result is {}", result);
    Ok(())
}

pub fn solution(input: &std::path::Path, part: PartSelection) -> Result<(), String> {

    part1(input);

    Ok(())
}
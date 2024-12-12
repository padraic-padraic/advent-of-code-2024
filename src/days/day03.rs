use std::io::BufReader;
use std::fs::File;
use utf8_chars::{
    BufReadCharsExt,
    Chars
};

#[derive(Debug)]
enum MulInstruction {
    CharM,
    CharU,
    CharL,
    LBracket,
    LDigit,
    Comma,
    RDigit,
    Start,
}
#[derive(Debug)]
enum ControlInstruction {
    CharD,
    CharO,
    CharN,
    Apos,
    CharT,
    LBracket
}

#[derive(Debug)]
struct ProgrammeState {
    result: i64,
    do_instructions: bool,
}

fn handle_mul_instruction(state: &mut ProgrammeState, chars: &mut Chars<BufReader<File>>) {
    let mut instruction = MulInstruction::CharM;
    let mut l_digits = String::new();
    let mut r_digits = String::new();
    loop {
        let char = match chars.next(){
            Some(next_c) => {
                match next_c {
                    Err(message) => {panic!("Error {} reading characteter", message)}
                    Ok(c) => c
                }
            }
            _ => break
        };
        instruction = match (instruction, char) {
            (MulInstruction::CharM, 'u') => {
                MulInstruction::CharU
            }
            (MulInstruction::CharU, 'l') => {
                MulInstruction::CharL
            }
            (MulInstruction::CharL, '(') => {
                MulInstruction::LBracket
            }
            (MulInstruction::LBracket, '0'..='9') => {
                l_digits.push(char);
                MulInstruction::LDigit
            }
            (MulInstruction::LDigit, '0'..='9') => {
                l_digits.push(char);
                MulInstruction::LDigit
            }
            (MulInstruction::LDigit, ',') => {
                MulInstruction::Comma
            }
            (MulInstruction::Comma | MulInstruction::RDigit, '0'..='9') => {
                r_digits.push(char);
                MulInstruction::RDigit
            }
            (MulInstruction::RDigit, ')') => {
                // println!("Finished, found numbers {}, {}", l_digits, r_digits);
                match (l_digits.parse::<i64>(), r_digits.parse::<i64>()) {
                    (Ok(l), Ok(r)) => {
                        // println!("Parsed numbers as {}, {}", l, r);
                        if state.do_instructions {
                            state.result += l * r;
                        }
                    }
                    _ => {
                        panic!("Error parsing the digits {}, {}", l_digits, r_digits);
                    }
                };
                break;
            }
            _ => {
               break;
            }
        };
    }
}

fn handle_do_instruction(state: &mut ProgrammeState, chars: &mut Chars<BufReader<File>>) {
    let mut instruction = ControlInstruction::CharD;
    let mut command = "d".to_string();
    loop {
        let char = match chars.next(){
            Some(next_c) => {
                match next_c {
                    Err(message) => {panic!("Error {} reading characteter", message)}
                    Ok(c) => c
                }
            }
            _ => break
        };
        instruction = match (instruction, char) {
            (ControlInstruction::CharD, 'o') => {
                command.push(char);
                ControlInstruction::CharO
            }
            (ControlInstruction::CharO, '(') => {
                ControlInstruction::LBracket
            }
            (ControlInstruction::CharO, 'n') => {
                command.push(char);
                ControlInstruction::CharN
            }
            (ControlInstruction::CharN, '\'') => {
                command.push(char);
                ControlInstruction::Apos
            }
            (ControlInstruction::Apos, 't') => {
                command.push(char);
                ControlInstruction::CharT
            }
            (ControlInstruction::CharT, '(') => {
                ControlInstruction::LBracket
            }
            (ControlInstruction::LBracket, ')') => {
                if command == "do" {
                    state.do_instructions = true;
                } else {
                    state.do_instructions = false;
                }
                break;
            }
            _ => {
                break;
            }
        };
    }
}

const TEST: &str = "\
xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
";

const TEST2: &str = "\
xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
";

pub fn part1(input: &std::path::Path) -> Result<(), String> {
    let f = match File::open(input) {
        Ok(f) => {f}
        Err(message)  => {
            println!("Unable to open {:?} due to {}", input, message);
            return Err("IO/Error".to_string());
        }
    };
    let mut reader = BufReader::new(f);
    let mut state = ProgrammeState{result:0, do_instructions: true};
    let mut chars = reader.chars();
    loop {
        let char = match chars.next(){
            Some(next_c) => {
                match next_c {
                    Err(message) => {panic!("Error {} reading characteter", message)}
                    Ok(c) => c
                }
            }
            _ => break
        };
        if char == 'm' {
            handle_mul_instruction(&mut state, &mut chars);
        }
    };
    println!("Result is {}", state.result);
    Ok(())
}

pub fn part2(input: &std::path::Path) -> Result<(), String> {
    let f = match File::open(input) {
        Ok(f) => {f}
        Err(message)  => {
            println!("Unable to open {:?} due to {}", input, message);
            return Err("IO/Error".to_string());
        }
    };
    let mut reader = BufReader::new(f);
    let mut state = ProgrammeState{result:0, do_instructions: true};
    let mut chars = reader.chars();
    loop {
        let char = match chars.next(){
            Some(next_c) => {
                match next_c {
                    Err(message) => {panic!("Error {} reading characteter", message)}
                    Ok(c) => c
                }
            }
            _ => break
        };
        if char == 'm' {
            handle_mul_instruction(&mut state, &mut chars);
        }
        else if char == 'd' {
            handle_do_instruction(&mut state, &mut chars);
        }
    };
    println!("Result is {}", state.result);
    Ok(())
}
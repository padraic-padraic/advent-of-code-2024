use std::iter::FromIterator;
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

type Coordinate = (usize, usize);

fn is_valid(coord: &Coordinate, size: Coordinate) -> bool {
    coord.0 < size.0  && coord.1 < size.1
}

type Xmas = [Coordinate; 4];
type MasMas = [Coordinate; 5];

const DIRS: [Dir; 8] = [Dir::N, Dir::NE, Dir::E, Dir::SE, Dir::S, Dir::SW, Dir::W, Dir::NW];


fn shift_coords(initial_pos: Coordinate, dir: &Dir, shift: isize) -> Result<Coordinate, &str> {
    let (row, col) = initial_pos;
    let r_shift = match dir {
        Dir::N | Dir::NE | Dir::NW => {-1},
        Dir::S | Dir::SW | Dir::SE => {1},
        Dir::W | Dir::E => {0}
    };
    let c_shift = match dir {
        Dir::W | Dir::NW | Dir::SW => {-1},
        Dir::E | Dir::NE | Dir::SE => {1},
        Dir::N | Dir::S => {0}
    };
    match (row.checked_add_signed(r_shift * shift), col.checked_add_signed(c_shift * shift)) {
        (Some(new_row), Some(new_col)) => {
            Ok((new_row, new_col))
        }
        _ => {
            Err("Overflow detected, invalid coordinate")
        }
    }
}

fn get_xmas_coordinates(coordinate: Coordinate, dir: &Dir, puzzle_size: Coordinate) ->  Option<Xmas> {
    match (
        shift_coords(coordinate, dir,1), shift_coords(coordinate, dir, 2), shift_coords(coordinate, dir, 3)) {
            (Ok(m), Ok(a), Ok(s)
    ) => {
            let xmas = [coordinate, m, a, s];
            if xmas.iter().all(|coord| is_valid(coord, puzzle_size)) {
                Some(xmas)
            } else {
                None
            }
        }
        _ => {
            None
        }
    }
}

fn get_x_mas_coordinates(coord: Coordinate, puzzle_size: Coordinate) -> Option<MasMas> {
    let dirs = [Dir::E, Dir::S, Dir::SE];
    match (
        shift_coords(coord, &dirs[0], 2),
        shift_coords(coord, &dirs[1], 2),
        shift_coords(coord, &dirs[2], 1),
        shift_coords(coord, &dirs[2], 2),
    ) {
        (Ok(tr), Ok(bl), Ok(mid), Ok (br)) => {
            let x_mas = [coord, tr, bl, mid, br];
            if x_mas.iter().all(|coord| is_valid(coord, puzzle_size)) {
                Some(x_mas)
            } else {
                None
            }
        }
        _ => {None}
    }
}

fn get_word(words: &[Vec<char>], coords: Xmas) -> String{
    coords.iter().map(|c| {
        words[c.0][c.1]
    }).collect()
}

fn get_x_mas_word(words: &[Vec<char>], coords: MasMas) -> String {
    coords.iter().map(|c| {
        words[c.0][c.1]
    }).collect()
}

fn part1(words: &[Vec<char>]) -> Result<(), String> {
    let puzzle_size = (words.len(), words[0].len());
    let mut r_pos = 0;
    let mut word_count = 0;
    while r_pos < puzzle_size.0 {
        let mut c_pos = 0;
        while c_pos < puzzle_size.1 {
            if words[r_pos][c_pos] == 'X' {
                for d in DIRS.iter() {
                    if let Some(coords) = get_xmas_coordinates((r_pos, c_pos), d, puzzle_size) {
                        if get_word(words, coords).as_str() == "XMAS" {
                            word_count += 1;
                        }
                    }
                }
            }
            c_pos +=1;
        }
        r_pos += 1;
    }
    println!("Part 1: {} Words Found", word_count);
    Ok(())
}

fn part2(words: &[Vec<char>]) -> Result<(), String> {
    let puzzle_size = (words.len(), words[0].len());
    let mut r_pos = 0;
    let mut word_count = 0;
    while r_pos < puzzle_size.0 {
        let mut c_pos = 0;
        while c_pos < puzzle_size.1 {
            let tl_char = words[r_pos][c_pos];
            if tl_char == 'M' || tl_char == 'S' {
                if let Some(coords) = get_x_mas_coordinates((r_pos, c_pos), puzzle_size) {
                    let word = get_x_mas_word(words, coords);
                    match word.as_str() {
                        "MMSAS" | "MSMAS" | "SMSAM" | "SSMAM" => {
                            word_count += 1;
                        }
                        _ => {}
                    }
                }
            }
            c_pos +=1;
        }
        r_pos += 1;
    }
    println!("Part 2: {} X-MAS instances found", word_count);
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
    let reader = BufReader::new(f);
    let words: Vec<Vec<char>> = reader.lines().map(|s| {
        if let Ok(line) = s {
            let chars: Vec<char> = line.trim().chars().collect();
            chars
        } else {
            panic!("Error unpacking text from {:?}", input);
        }
    }).collect();
    part1(&words);
    part2(&words);
    Ok(())
}
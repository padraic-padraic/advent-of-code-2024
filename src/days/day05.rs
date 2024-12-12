use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use regex::Regex;


type Dependecies = HashMap<i64, Vec<i64>>;
type Updates = Vec<Vec<i64>>;


fn read_input(input: &std::path::Path) -> Result<(Dependecies, Updates), String> {
    let dep_rule = Regex::new(r"(?<pred>\d+)\|(?<succ>\d+)$").unwrap();
    let f = match File::open(input) {
        Ok(f) => f,
        Err(message) => {
            println!("Unable to open {:?} due to {}", input, message);
            return Err("IO/Error".to_string())
        }
    };
    let reader = BufReader::new(f);
    let mut deps: Dependecies = HashMap::new();
    let mut updates: Updates = Vec::new();
    for line in reader.lines() {
        let trim_line = match line {
            Ok(l) => l,
            Err(error) => {
                println!("Caught IO error {} reading {:?}", error, input);
                return Err("I/O Error".to_string());
            }
        };
        let trim_line = trim_line.trim();
        if let Some(dep_cap) = dep_rule.captures(trim_line) {
            match (dep_cap.name("pred"), dep_cap.name("succ")) {
                (Some(pred_str), Some(succ_str)) => {
                    if let (Ok(pre), Ok(succ)) = (pred_str.as_str().parse::<i64>(), succ_str.as_str().parse::<i64>()) {
                        if let Some(successors) = deps.get_mut(&pre) {
                            successors.push(succ);
                        } else {
                            deps.insert(pre, vec![succ]);
                        }
                    }
                }
                _ => {return Err(format!("Error unwraping dependency line {}", trim_line));}
            };
        } else {
            if trim_line.len() > 0 {
                let to_print: Vec<i64> = trim_line.split(",").map(|num| num.parse::<i64>().unwrap()).collect();
                updates.push(to_print);
            }
        }
    }
    Ok((deps, updates))
}


fn valid_update(update: &Vec<i64>, deps: &Dependecies) -> bool {
    let mut visited: HashSet<i64> = HashSet::new();
    for page_num in update {
        visited.insert(*page_num);
        if let Some(successors) = deps.get(page_num) {
            if successors.iter().any(|succ| {
                visited.contains(succ)
            }) {
                return false
            }
        }
    }
    true
}

pub fn part1(input: &std::path::Path) -> Result<(), String> {
    let (deps, updates) = read_input(input)?;
    let page_num_sum = updates.iter().fold(0, |sum, update| {
        if valid_update(update, &deps) {
            let middle_index = update.len() / 2;
            sum + update.get(middle_index).unwrap()
        } else {
            sum
        }
    });
    println!("Part 1: Total valid page nums is {}", page_num_sum);
    Ok(())
}

fn sorted_update_middle_page(update: &Vec<i64>, deps: &Dependecies) -> i64 {
    let mut page_nums = update.clone();
    let mut sorted_update: Vec<i64> = Vec::new();
    while page_nums.len() > 0 {
        let mut pop_index: usize = 0;
        loop {
            let p: &i64 = page_nums.get(pop_index).unwrap();
            let predecessors: Vec<i64> = deps.iter().filter(|(k, v)| {
                v.contains(p) && update.contains(k)
            }).map(|(k, _v)| *k).collect();
            if predecessors.len() == 0 || predecessors.iter().all(|v| sorted_update.contains(v)) {
                sorted_update.push(*p);
                break
            }
            pop_index += 1
        };
        page_nums.remove(pop_index);
    }
    let middle_index = sorted_update.len() / 2;
    *sorted_update.get(middle_index).unwrap()
}

pub fn part2(input: &std::path::Path) -> Result<(), String> {
    let (deps, updates) = read_input(input)?;
    let page_num_sum = updates.iter().filter(
        |update| !valid_update(update, &deps)
    ).fold(0, |sum, update| {
        sum + sorted_update_middle_page(update, &deps)
    });
    println!("Part 2: Total corrected page nums is {}", page_num_sum);
    Ok(())
}
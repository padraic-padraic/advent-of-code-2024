use itertools::Itertools;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::iter::zip;


fn read_input(input: &std::path::Path) -> Result<(Vec<u64>, Vec<u64>), String> {
    let f = match File::open(input) {
        Ok(f) => {f}
        Err(message)  => {
            println!("Unable to open {:?} due to {}", input, message);
            return Err("IO/Error".to_string());
        }
    };
    let reader = BufReader::new(f);
    let mut v1: Vec<u64> = Vec::new();
    let mut v2: Vec<u64> = Vec::new();
    for l in reader.lines() {
        let nums: Vec<u64> = l
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse::<u64>().unwrap())
            .collect();
        v1.push(nums[0]);
        v2.push(nums[1]);
    }
    Ok((v1, v2))
}

pub fn part1(input: &std::path::Path) -> Result<(), String> {
    let (list1, list2) = read_input(input)?;
    let mut sum = 0;
    for tup in zip(
        Itertools::sorted(list1.iter()),
        Itertools::sorted(list2.iter()),
    ) {
        sum += tup.0.abs_diff(*tup.1)
    }
    println!("Part 1: Result is {}", sum);
    Ok(())
}

fn count_instances(x: &u64, list: &[u64]) -> u64 {
    let count = list.iter().filter(|y| **y == *x).count();
    return count as u64;
}

pub fn part2(input: &std::path::Path) -> Result<(), String>{
    let (list1, list2) = read_input(input)?;
    let slice = list2.as_slice();
    let result: u64 = list1.iter().map(|x| x * count_instances(x, slice)).sum();
    println!("Part 2: Result is {}", result);
    Ok(())
}

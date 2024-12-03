use super::PartSelection;
use itertools::Itertools;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::iter::zip;

fn part1(list1: Vec<u64>, list2: Vec<u64>) {
    let mut sum = 0;
    for tup in zip(
        Itertools::sorted(list1.iter()),
        Itertools::sorted(list2.iter()),
    ) {
        sum += tup.0.abs_diff(*tup.1)
    }
    println!("Part 1: Result is {}", sum)
}

fn count_instances(x: &u64, list: &[u64]) -> u64 {
    let count = list.iter().filter(|y| **y == *x).count();
    return count as u64;
}

fn part2(list1: Vec<u64>, list2: Vec<u64>) {
    let slice = list2.as_slice();
    let result: u64 = list1.iter().map(|x| x * count_instances(x, slice)).sum();
    println!("Part 2: Result is {}", result)
}

pub fn solution(input: &std::path::Path, parts: PartSelection) -> Result<(), String> {
    let f = File::open(input).unwrap();
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
    if parts == PartSelection::PartOne || parts == PartSelection::All {
        part1(v1.clone(), v2.clone());
    }
    if parts == PartSelection::PartTwo || parts == PartSelection::All {
        part2(v1, v2);
    }
    Ok(())
}

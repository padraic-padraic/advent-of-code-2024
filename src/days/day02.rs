use itertools::multizip;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn get_reports(input: &std::path::Path) -> Result<Vec<Vec<i64>>, String> {
    let f = match File::open(input) {
        Ok(f) => {f}
        Err(message)  => {
            println!("Unable to open {:?} due to {}", input, message);
            return Err("IO/Error".to_string());
        }
    };
    let reader = BufReader::new(f);
    let reports: Vec<Vec<i64>> = reader
        .lines()
        .map(|line| {
            line.unwrap()
                .split_whitespace()
                .map(|x| x.parse::<i64>().unwrap())
                .collect()
        })
        .collect();
    Ok(reports)
}

fn is_unsafe_diff(diff: i64) -> bool {
    let abs_diff = diff.abs();
    abs_diff < 1 || abs_diff > 3
}

fn differing_sign(diff: i64, diff2: i64) -> bool {
    diff.signum() == diff2.signum()
}

fn is_safe(report: &Vec<i64>) -> Result<(), ()> {
    let length = report.len();
    let v1 = &report[..length - 2];
    let v2 = &report[1..length - 1];
    let v3 = &report[2..length];
    for vals in multizip((v1.iter(), v2.iter(), v3.iter())) {
        let diff1 = vals.0 - vals.1;
        let diff2 = vals.2 - vals.1;
        if differing_sign(diff1, diff2) {
            return Err(());
        }
        if is_unsafe_diff(diff1) || is_unsafe_diff(diff2) {
            return Err(());
        }
    }
    Ok(())
}

pub fn part1(input: &std::path::Path) -> Result<(), String> {
    let reports = get_reports(input)?;
    let num_safe = reports.iter().fold(0, |acc, report| {
        if let Ok(()) = is_safe(report) {
            acc + 1
        } else {
            acc
        }
    });
    println!("Part 1: {} safe reports", num_safe);
    Ok(())
}

pub fn part2(input: &std::path::Path) -> Result<(), String> {
    let reports = get_reports(input)?;
    let mut num_safe = 0;
    for r in reports {
        if let Ok(()) = is_safe(&r) {
            num_safe += 1;
            continue;
        }
        let mut index = 0;
        while index < r.len() {
            let sub_report: Vec<i64> = r
                .iter()
                .enumerate()
                .filter(|tup| tup.0 != index)
                .map(|tup| *tup.1)
                .collect();
            if let Ok(()) = is_safe(&sub_report) {
                num_safe += 1;
                break;
            }
            index += 1;
        }
    }
    println!("Part 2: {} safe reports", num_safe);
    Ok(())
}

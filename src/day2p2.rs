use regex::Regex;
use std::collections::BTreeSet;

const INPUT: &str = include_str!("../data/day2/input.txt");

fn count_repeated_patterns(min: u64, max: u64) -> u64 {
    let mut unique_patterns = BTreeSet::new();
    let max_digits = (((max as f64).log10().floor() + 1.) / 2.).floor() as usize;
    let max_n: usize = "9".repeat(max_digits).parse().unwrap();
    for n in 1..=max_n {
        let init = n.to_string();
        let mut pattern_str = init.clone();
        loop {
            pattern_str.push_str(&init.clone());
            let pattern = pattern_str.parse().unwrap();
            if pattern >= min && pattern <= max {
                if !unique_patterns.contains(&pattern) {
                    println!("invalid code found: {}", pattern);
                    unique_patterns.insert(pattern);
                }
            }
            if pattern > max {
                break;
            }
        }
    }
    println!("{} total invalid codes found", unique_patterns.len());
    unique_patterns.iter().sum()
}

pub fn run() {
    let re = Regex::new(r"(\d+)-(\d+)").unwrap();
    let mut answer = 0;
    for group in INPUT.split(',') {
        let caps = re.captures(group).unwrap();
        let start = caps.get(1).unwrap().as_str().parse().unwrap();
        let end = caps.get(2).unwrap().as_str().parse().unwrap();
        let invalid_codes = count_repeated_patterns(start, end);
        println!("Completed group ({} invalid codes)", invalid_codes);
        println!();
        answer = answer + invalid_codes;
    }

    println!("Day 2 part 2 answer = {}", answer);
}
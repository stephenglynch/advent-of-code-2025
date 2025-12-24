use regex::Regex;

const INPUT: &str = include_str!("../data/day2/input.txt");

fn count_palindromes(min: u64, max: u64) -> u64 {
    let mut palindromes = vec![];
    for n in 0.. {
        let left = n.to_string();
        let right = left.clone();
        let palin_str = left + &right;
        let palin = palin_str.parse().unwrap();
        if palin >= min && palin <= max {
            palindromes.push(palin);
        }
        if palin > max {
            break;
        }
    }
    palindromes.iter().sum()
}

pub fn run() {
    let re = Regex::new(r"(\d+)-(\d+)").unwrap();
    let mut answer = 0;
    for group in INPUT.split(',') {
        let caps = re.captures(group).unwrap();
        let start = caps.get(1).unwrap().as_str().parse().unwrap();
        let end = caps.get(2).unwrap().as_str().parse().unwrap();
        answer = answer + count_palindromes(start, end);
    }

    println!("Day 2 part 1 answer = {}", answer);
}
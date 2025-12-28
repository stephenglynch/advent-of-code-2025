use std::cmp::Ordering;

const INPUT: &str = include_str!("../data/day3/input.txt");

type Joltage = u64;

#[derive(Debug, PartialEq, Eq)]
struct Battery(Joltage, usize);

type Bank = Vec<Battery>;

impl PartialOrd for Battery {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let joltage_cmp = self.0.cmp(&other.0);
        Some(match joltage_cmp {
            Ordering::Equal => self.1.cmp(&other.1),
            _ => joltage_cmp.reverse()
        })
    }
}

impl Ord for Battery {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn parse_bank(line: &str) -> Bank {
    let mut bank: Bank = line.chars().map(|c| c.to_string().parse().unwrap()).enumerate().map(|(a, b)| Battery(b, a)).collect();
    // We sort as it makes find the biggest joltages extremely efficient
    bank.sort();
    bank
}

fn find_biggest_joltage(bank: &Bank) -> u64 {
    let len = bank.len();
    let mut total_joltage = 0;
    let mut prev_i = 0;
    for i_max in (0..12).rev() {
        // Get next joltage, must after previous and must still have enough space at end for more
        let &Battery(joltage, i) = bank.iter()
            .filter(|&&Battery(_, i)| i >= prev_i && i < len - i_max)
            .nth(0)
            .unwrap();
        prev_i = i + 1;
        total_joltage = total_joltage * 10 + joltage;
    }
    total_joltage
}

pub fn run() {
    let mut answer: u64 = 0;
    for line in INPUT.lines() {
        let bank = parse_bank(line);
        answer = answer + find_biggest_joltage(&bank);
    }

    println!("Day 3 part 2 answer = {}", answer);
}

#[cfg(test)]
mod tests {
    use super::*;

macro_rules! test_joltage {
        ($($name:ident: $input:expr, $expected:expr),*) => {
            $(
                #[test]
                fn $name() {
                    let bank = parse_bank($input);
                    assert_eq!(find_biggest_joltage(&bank), $expected);
                }
            )*
        }
    }

    test_joltage! {
        example_line1: "987654321111111", 987654321111,
        example_line2: "811111111111119", 811111111119,
        example_line3: "234234234234278", 434234234278,
        example_line4: "818181911112111", 888911112111
    }
}
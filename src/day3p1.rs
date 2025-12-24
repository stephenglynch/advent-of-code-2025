use std::cmp::Ordering;

const INPUT: &str = include_str!("../data/day3/input.txt");

type Joltage = u8;

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

fn find_biggest_joltage(bank: &Bank) -> u32 {
    let len = bank.len();
    // Get first joltage make sure it's the not the last value in the bank
    let &Battery(joltage1, i1) = bank.iter().filter(|&&Battery(_, i)| i < len - 1).nth(0).unwrap();
    // Get the biggest joltage to the right of the largest joltage found in the previous step
    let joltage2 = bank.iter().filter(|&&Battery(_, i)| i > i1).nth(0).unwrap().0;
    (joltage1 * 10 + joltage2) as u32
}

pub fn run() {
    let mut answer: u32 = 0;
    for line in INPUT.lines() {
        let bank = parse_bank(line);
        answer = answer + find_biggest_joltage(&bank);
    }

    println!("Day 3 part 1 answer = {}", answer);
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
        example_line1: "987654321111111", 98,
        example_line2: "811111111111119", 89,
        test1:  "5367", 67,
        test2:  "11117511111", 75,
        test3:  "7642", 76,
        test4:  "6872", 87,
        test5:  "7920", 92,
        test6:  "1106", 16,
        test7:  "6440", 64,
        test8:  "2543", 54,
        test9:  "5532", 55,
        test10:  "2954", 95,
        test11: "3133", 33,
        test12: "6918", 98
    }
}
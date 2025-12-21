use std::ops;

const INPUT: &str = include_str!("../data/day1/input.txt");

struct Wrapping100(i16);

impl ops::Add for Wrapping100 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Wrapping100((self.0 + rhs.0) % 100)
    }
}

impl ops::Sub for Wrapping100 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Wrapping100((self.0 - rhs.0) % 100)
    }
}

pub fn part1() {
    let mut clock = Wrapping100(50);
    let mut answer = 0;

    for line in INPUT.lines() {
        let prefix = line.chars().nth(0).unwrap();
        let num = line[1..].parse().unwrap();
        if prefix == 'L' {
            clock = clock - Wrapping100(num);
        } else {
            clock = clock + Wrapping100(num);
        }

        if clock.0 == 0 {
            answer = answer + 1;
        }
    }

    println!("answer = {}", answer);
}
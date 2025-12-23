use std::ops;

const INPUT: &str = include_str!("../data/day1/input.txt");

struct Wrapping100 {
    value: i32,
    zero_xs: i32
}

impl Wrapping100 {
    fn new(value: i32) -> Wrapping100 {
        Wrapping100 {
            value: value,
            zero_xs: 0
        }
    }
}

impl ops::Add for Wrapping100 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Wrapping100 {
            value: (self.value + rhs.value) % 100,
            zero_xs: self.zero_xs + (self.value + rhs.value / 100)
        }
    }
}

impl ops::Sub for Wrapping100 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Wrapping100 {
            value: (self.value - rhs.value) % 100,
            zero_xs: if self.value - rhs.value <= 0 {
                self.zero_xs + (self.value - rhs.value).abs() / 100 + 1
            } else {
                self.zero_xs
            }
        }
    }
}

pub fn part2() {
    let mut clock = Wrapping100::new(50);
    let mut answer = 0;

    for line in INPUT.lines() {
        let prefix = line.chars().nth(0).unwrap();
        let num = line[1..].parse().unwrap();
        if prefix == 'L' {
            clock = clock - Wrapping100::new(num);
        } else {
            clock = clock + Wrapping100::new(num);
        }
    }

    println!("answer = {}", clock.zero_xs);
}
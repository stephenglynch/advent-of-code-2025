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

    fn right(&mut self, value: i32) {
        for _ in 0..value {
            self.value = self.value + 1;
            if self.value == 100 {
                self.value = 0;
                self.zero_xs = self.zero_xs + 1;
            }
        }
    }

    fn left(&mut self, value: i32) {
        for _ in 0..value {
            self.value = self.value - 1;
            if self.value == 0 {
                self.zero_xs = self.zero_xs + 1;
            } else if self.value == -1 {
                self.value = 99;
            }
        }
    }
}

pub fn run() {
    let mut clock = Wrapping100::new(50);

    for line in INPUT.lines() {
        let prefix = line.chars().nth(0).unwrap();
        let num = line[1..].parse().unwrap();
        if prefix == 'L' {
            clock.left(num);
        } else {
            clock.right(num);
        }
    }

    println!("Day 1 part 2 answer = {}", clock.zero_xs);
}
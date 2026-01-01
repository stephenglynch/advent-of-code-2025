use core::panic;
use std::collections::BTreeMap;
use itertools::Itertools;

const INPUT: &str = include_str!("../data/day7/input.txt");

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Point {
    Outside,
    Empty,
    Start,
    Splitter,
    Termination
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Grid {
    x_len: usize,
    y_len: usize,
    start: (usize, usize),
    grid: Vec<Point>,
    mem: BTreeMap<(usize, usize), u64>
}

impl Grid {
    fn new(text: &str) -> Grid {
        Grid {
            mem: BTreeMap::new(),
            x_len: text.lines().nth(0).unwrap().chars().count(),
            y_len: text.lines().count(),
            start: (text.chars().find_position(|&c| c == 'S').unwrap().0, 0),
            grid: text.lines().flat_map(|l| {
                l.chars().map(|c| {
                    match c {
                        '.' => Point::Empty,
                        'S' => Point::Start,
                        '^' => Point::Splitter,
                        _ => panic!()
                    }
                })
            }).collect(),
        }
    }

    fn get(&self, x: usize, y: usize) -> Point {
        if x >= self.x_len {
            Point::Outside
        } else if y >= self.y_len {
            Point::Termination
        } else {
            self.grid[self.x_len * y + x]
        }
    }

    fn propagate_system(&mut self) -> u64 {
        assert!(self.get(self.start.0, self.start.1) == Point::Start);
        self.propagate(self.start.0, self.start.1)
    }

    fn propagate(&mut self, x: usize, y: usize) -> u64 {
        // We use memoization as number of paths grows exponentially but so
        // does the number of revisited paths. This keeps the problem tractable
        if let Some(val) = self.mem.get(&(x, y)) {
            *val
        } else {
            let val = match self.get(x, y) {
                Point::Start => self.propagate(x, y+1),
                Point::Empty => self.propagate(x, y+1),
                Point::Splitter => {
                    self.propagate(x-1, y) +
                    self.propagate(x+1, y)
                }
                Point::Termination => 1,
                Point::Outside => 0
            };
            self.mem.insert((x, y), val);
            val
        }
    }
}

pub fn run() {
    let mut grid = Grid::new(INPUT);
    let answer= grid.propagate_system();

    println!("Day 7 part 2 answer = {}", answer);
}

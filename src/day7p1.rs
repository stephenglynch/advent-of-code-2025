use core::panic;

use itertools::Itertools;

const INPUT: &str = include_str!("../data/day7/input.txt");

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Point {
    Outside,
    Empty,
    Start,
    Splitter,
    UsedSplitter
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Grid {
    x_len: usize,
    y_len: usize,
    start: (usize, usize),
    grid: Vec<Point>,
}

impl Grid {
    fn new(text: &str) -> Grid {
        Grid {
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
        if x >= self.x_len || y >= self.y_len {
            Point::Outside
        } else {
            self.grid[self.x_len * y + x]
        }
    }

    fn set(&mut self, x: usize, y: usize, p: Point) {
        self.grid[self.x_len * y + x] = p;
    }

    fn propagate_system(&mut self) -> u32 {
        assert!(self.get(self.start.0, self.start.1) == Point::Start);
        self.propagate(self.start.0, self.start.1)
    }

    fn propagate(&mut self, x: usize, y: usize) -> u32 {
        match self.get(x, y) {
            Point::Start => self.propagate(x, y+1),
            Point::Empty => self.propagate(x, y+1),
            Point::Splitter => {
                self.set(x, y, Point::UsedSplitter);
                1 +
                self.propagate(x-1, y) +
                self.propagate(x+1, y)
            }
            Point::UsedSplitter => 0,
            Point::Outside => 0
        }
    }
}

pub fn run() {
    let mut grid = Grid::new(INPUT);
    let answer= grid.propagate_system();

    println!("Day 7 part 1 answer = {}", answer);
}

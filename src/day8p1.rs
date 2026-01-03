use itertools::Itertools;

const INPUT: &str = include_str!("../data/day8/example.txt");

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Point(i32, i32, i32);

impl Point {
    fn dist(&self, other: &Self) -> i32 {
        (self.0 - other.0).pow(2) + (self.1 - other.1).pow(2) + (self.2 - other.2).pow(2)
    }
}

fn parse_points(text: &str) -> Vec<Point> {
    text.lines().map(|l| {
        let mut it = l.split(",").map(|s| s.parse().unwrap());
        Point(it.next().unwrap(), it.next().unwrap(), it.next().unwrap())
    }).collect()
}

fn shortest_pairs(points: Vec<Point>) -> Vec<(Point, Point)> {
    points.iter().copied().combinations(2).map(|p| (p[0], p[1])).sorted_by(|(a0, a1), (b0, b1)| {
        let dist1 = a0.dist(a1);
        let dist2 = b0.dist(b1);
        dist1.cmp(&dist2)
    }).take(10).collect()
}

pub fn run() {
    let answer = 123;

    let points = parse_points(INPUT);

    println!("{:?}", shortest_pairs(points));

    println!("Day 8 part 1 answer = {}", answer);
}

use regex::Regex;

const INPUT: &str = include_str!("../data/day5/input.txt");

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct IdRange {
    start: u64,
    end: u64
}

impl IdRange {
    fn parse_range(range_str: &str) -> Option<Self> {
        let re = Regex::new(r"(\d+)-(\d+)").unwrap();
        let caps = re.captures(range_str)?;
        let start = caps.get(1)?.as_str().parse().unwrap();
        let end = caps.get(2)?.as_str().parse().unwrap();
        Some(IdRange {
            start: start,
            end: end
        })
    }

    fn combine(self, other: Self) -> Option<Self> {
        if self.start >= other.start && self.end <= other.end {
            Some(other)
        } else if other.start >= self.start && other.end <= self.end {
            Some(self)
        } else if self.start >= other.start && self.start <= other.end {
            Some(IdRange {start: other.start, end: self.end})
        } else if self.end >= other.start && self.end <= other.end {
            Some(IdRange {start: self.start, end: other.end})
        } else {
            None
        }
    }
}

fn parse_database(text: &str) -> Vec<IdRange> {
    let mut id_ranges = Vec::new();
    let mut lines = text.lines();

    // Process ID ranges
    while let Some(line) = lines.next() {
        if let Some(range) = IdRange::parse_range(line) {
            id_ranges.push(range);
        } else {
            break;
        }
    }

    id_ranges
}

pub fn run() {
    let mut answer = 0;
    let mut id_ranges: Vec<IdRange> = parse_database(INPUT).into_iter().collect();

    id_ranges.sort();
    let mut merged = Vec::new();
    let mut current = id_ranges[0];
    for range in id_ranges {
        if let Some(combined) = current.combine(range) {
            current = combined;
        } else {
            merged.push(current);
            current = range;
        }
    }
    merged.push(current);

    // Count the number of IDs contained in all the non-overlapping ranges
    for range in merged  {
        answer += range.end - range.start + 1;
    }

    println!("Day 5 part 2 answer = {}", answer)
}

use regex::Regex;

const INPUT: &str = include_str!("../data/day5/input.txt");

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

    fn in_range(&self, num: u64) -> bool {
        num >= self.start && num <= self.end
    }
}

fn parse_database(text: &str) -> (Vec<IdRange>, Vec<u64>) {
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

    // lines.next(); // Skip newline

    // Process ingredients IDs
    let ingredients = lines.map(|x| x.parse().unwrap()).collect();

    (id_ranges, ingredients)
}

pub fn run() {
    let mut answer = 0;
    let (id_ranges, ingredients) = parse_database(INPUT);
    for id in ingredients.iter() {
        for range in id_ranges.iter() {
            if range.in_range(*id) {
                answer += 1;
                break;
            }
        }
    }
    println!("Day 5 part 1 answer = {}", answer)
}

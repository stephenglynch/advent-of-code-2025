const INPUT: &str = include_str!("../data/day4/input.txt");

struct Grid {
    grid: Vec<bool>,
    num_rows: usize,
    num_cols: usize
}

impl Grid {
    fn new(text: &str) -> Self {
        let mut grid = Vec::new();
        let num_rows = text.lines().count();
        let num_cols = text.lines().nth(0).unwrap().len();
        for l in text.lines() {
            for c in l.chars() {
                grid.push(c == '@');
            }
        }
        Grid {
            grid: grid,
            num_rows: num_rows,
            num_cols: num_cols
        }
    }

    fn get(&self, col: usize, row: usize) -> Option<bool>
    {
        if col < self.num_cols && row < self.num_rows {
            self.grid.get(self.num_cols * row + col).copied()
        } else {
            None
        }
    }

    fn check_collectable(&self, col: usize, row: usize) -> bool {
        if !self.get(col, row).unwrap_or(false) {
            return false
        }
        let mut adjacent_rolls = 0;
        let col_start = col.saturating_sub(1);
        let row_start = row.saturating_sub(1);
        for c in col_start..=col+1 {
            for r in row_start..=row+1 {
                if !(c == col && r == row) {
                    if let Some(roll) = self.get(c, r) {
                        adjacent_rolls += roll as u8;
                    }
                }
            }
        }
        adjacent_rolls < 4
    }

    fn collect_rolls(&mut self) -> u32 {
        let mut rolls_collected = 0;
        for r in 0..self.num_rows {
            for c in 0..self.num_cols {
                if self.check_collectable(c, r) {
                    rolls_collected += 1;
                    self.grid[self.num_cols * r + c] = false;
                }
            }
        }
        rolls_collected
    }
}

pub fn run() {
    let mut grid = Grid::new(INPUT);
    let mut answer = 0;
    loop {
        let rolls_collected = grid.collect_rolls();
        answer += rolls_collected;
        if rolls_collected == 0 {
            break;
        }
    }
    println!("Day 4 part 2 answer = {}", answer);
}

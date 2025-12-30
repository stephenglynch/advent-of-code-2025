use core::panic;

const INPUT: &str = include_str!("../data/day6/input.txt");

#[derive(Clone, Copy)]
enum Operator {
    Add, Mult
}

#[derive(Clone)]
struct Problem {
    operands: Vec<u64>,
    operator: Operator
}

type HomeWork = Vec<Problem>;

impl Problem {
    fn new() -> Problem {
        Problem {operands: Vec::new(), operator: Operator::Add}
    }
}

fn parse_homework(text: &str) -> HomeWork {
    let mut lines: Vec<_> = text.lines().collect();
    let line_len = lines[0].len();
    let mut operators = lines.pop().unwrap().split_whitespace().map(|s| {
        match s {
            "+" => Operator::Add,
            "*" => Operator::Mult,
            _ => panic!()
        }
    });

    // With operators removed, transpose text like a matrix
    let text_trans: Vec<String> = (0..line_len).map(|i|{
        lines.iter().map(|&l| l.chars().nth(i).unwrap()).collect()
    }).collect();

    // Parse each group of lines and create the problems
    let mut homework: Vec<Problem> = Vec::new();
    let mut problem = Problem::new();
    for s in text_trans.into_iter() {
        if s.trim().is_empty() {
            problem.operator = operators.next().unwrap();
            homework.push(problem);
            problem = Problem::new();
        } else {
            problem.operands.push(s.trim().parse().unwrap());
        }
    }
    // Push last problem
    problem.operator = operators.next().unwrap();
    homework.push(problem);

    homework
}

pub fn run() {
    let homework = parse_homework(INPUT);
    let answer: u64 = homework.into_iter().fold(0, |acc, problem| {
        let computed: u64 = match problem.operator {
            Operator::Add => problem.operands.into_iter().sum(),
            Operator::Mult => problem.operands.into_iter().product()
        };
        acc + computed
    });
    println!("Day 6 part 2 answer = {}", answer)
}

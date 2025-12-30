use core::panic;

const INPUT: &str = include_str!("../data/day6/input.txt");

enum Operator {
    Add, Mult
}
struct Problem {
    operands: Vec<u64>,
    operator: Operator
}

type HomeWork = Vec<Problem>;

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|v| v.into_iter()).collect();
    (0..len).map(|_| {
        iters.iter_mut().map(|i| i.next().unwrap()).collect()
    }).collect()
}

fn parse_homework(text: &str) -> HomeWork {
    let chunked: Vec<Vec<&str>> = text.lines().map(|l| l.split_whitespace().collect()).collect();
    let chunked = transpose(chunked);
    chunked.into_iter().map(|mut v| {
        let op = match v.pop().unwrap() {
            "+" => Operator::Add,
            "*" => Operator::Mult,
            _ => panic!()
        };
        Problem {
            operands: v.into_iter().map(|s| s.parse().unwrap()).collect(),
            operator: op
        }
    }).collect()
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
    println!("Day 6 part 1 answer = {}", answer)
}

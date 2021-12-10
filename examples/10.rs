use aoc2021::{get_input, Error};
use std::io::{BufRead, BufReader};

fn get_matching_closing(c: char) -> Option<char> {
    Some(match c {
        '(' => ')',
        '<' => '>',
        '[' => ']',
        '{' => '}',
        _ => return None,
    })
}
fn get_score(c: char) -> u64 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("Not a closing delimiter: {}", c),
    }
}
fn get_score_completion(c: char) -> u64 {
    match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => panic!("Not a closing delimiter: {}", c),
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum LineType {
    Corrupted(u64),
    Incomplete(u64),
}

impl LineType {
    fn val(&self) -> u64 {
        match self {
            Self::Corrupted(v) => *v,
            Self::Incomplete(v) => *v,
        }
    }
}

fn validate_line(line: &str) -> LineType {
    let mut expected = Vec::new();
    for c in line.chars() {
        if let Some(close) = get_matching_closing(c) {
            expected.push(close);
        } else {
            if let Some(exp) = expected.pop() {
                if exp != c {
                    return LineType::Corrupted(get_score(c));
                }
            }
        }
    }
    let res = expected
        .iter()
        .rev()
        .copied()
        .map(get_score_completion)
        .reduce(|accu, val| accu * 5 + val)
        .unwrap();
    LineType::Incomplete(res)
}

fn main() -> Result<(), Error> {
    let input = BufReader::new(get_input(10)?);
    let (corrupted, mut incomplete): (Vec<LineType>, Vec<LineType>) = input
        .lines()
        .map(|l| validate_line(&l.unwrap()))
        .partition(|lt| match lt {
            LineType::Corrupted(_) => true,
            LineType::Incomplete(_) => false,
        });
    incomplete.sort();
    println!(
        "Answer 1: {}",
        corrupted.iter().map(LineType::val).sum::<u64>()
    );
    println!("Answer 2: {}", incomplete[incomplete.len() / 2].val());

    Ok(())
}

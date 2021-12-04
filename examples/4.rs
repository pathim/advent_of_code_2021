use aoc2021::{get_input, Error};
use std::{
    collections::HashSet,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
struct Board {
    lines: [HashSet<u32>; 10],
}
impl Board {
    fn new(data: &mut impl Iterator<Item = String>) -> Option<Self> {
        let mut lines: [HashSet<u32>; 10] = Default::default();
        for i in 0..5 {
            if let Some(d) = data.next() {
                let line: Vec<u32> = d
                    .split_ascii_whitespace()
                    .map(|s| s.parse().unwrap())
                    .collect();
                lines[i] = HashSet::from_iter(line.iter().copied());
                for (j, v) in line.iter().enumerate() {
                    lines[5 + j].insert(*v);
                }
            } else {
                return None;
            }
        }
        for l in lines.iter() {
            assert_eq!(l.len(), 5);
        }
        Some(Self { lines })
    }
    fn mark(&mut self, number: u32) -> Option<u32> {
        for l in self.lines.iter_mut() {
            l.remove(&number);
            if l.is_empty() {
                let rest_sum: u32 = self.lines[..5].iter().flatten().sum();
                return Some(rest_sum * number);
            }
        }
        None
    }
    fn is_finished(&self) -> bool {
        self.lines.iter().any(|l| l.is_empty())
    }
}

fn find_next_win(numbers: impl Iterator<Item = u32>, boards: &mut Vec<Board>) -> Option<u32> {
    for p in numbers {
        let res = boards.iter_mut().filter_map(|b| b.mark(p)).last();
        if res.is_some() {
            return res;
        }
    }
    None
}
fn main() -> Result<(), Error> {
    let input = BufReader::new(get_input(4)?);
    let mut lines = input.lines().map(|l| l.unwrap()).filter(|l| !l.is_empty());
    let first_line = lines.next().unwrap();
    let mut picked = first_line.split(',').map(|s| s.parse().unwrap());
    let mut boards = Vec::new();
    while let Some(b) = Board::new(&mut lines) {
        boards.push(b);
    }
    let first = find_next_win(&mut picked, &mut boards).unwrap();
    println!("Answer 1: {}", first);
    let mut last: u32 = 0;
    while let Some(val) = find_next_win(&mut picked, &mut boards) {
        last = val;
        boards.retain(|b| !b.is_finished());
    }
    println!("Answer 2: {}", last);

    Ok(())
}

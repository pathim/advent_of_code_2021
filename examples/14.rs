use aoc2021::{get_input, Error};
use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
};

fn count(counter: &mut HashMap<char, u64>, c: char) {
    if let Some(count) = counter.get_mut(&c) {
        *count += 1;
    } else {
        counter.insert(c, 1u64);
    }
}
fn main() -> Result<(), Error> {
    let input = BufReader::new(get_input(14)?);
    let mut lines = input.lines();
    let mut start = lines.next().unwrap().unwrap().chars().collect::<Vec<_>>();
    lines.next();
    let mut rules = HashMap::new();
    for l in lines {
        let l = l.unwrap();
        let (from, to) = l.split_once("->").unwrap();
        rules.insert(from.trim().to_string(), to.trim().chars().next().unwrap());
    }
    let mut counter = HashMap::new();

    for _ in 0..10 {
        let last = start.last().unwrap().clone();
        start = start
            .windows(2)
            .flat_map(|x| {
                [
                    x[0].clone(),
                    rules.get(&x.iter().collect::<String>()).unwrap().clone(),
                ]
            })
            .collect();
        start.push(last);
    }

    for c in start {
        count(&mut counter, c);
    }

    let max = counter.values().max().unwrap();
    let min = counter.values().min().unwrap();

    println!("Answer 1: {}", max - min);

    Ok(())
}

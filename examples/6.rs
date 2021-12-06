use aoc2021::{get_input, Error};
use std::io::{BufRead, BufReader};

fn advance(state: &[u64]) -> [u64; 9] {
    let mut res = [0u64; 9];
    for i in 1..res.len() {
        res[i - 1] = state[i];
    }
    res[6] += state[0];
    res[8] = state[0];
    res
}
fn main() -> Result<(), Error> {
    let input = BufReader::new(get_input(6)?);
    let mut state = [0u64; 9];
    for v in input.split(b',').map(|x| {
        String::from_utf8_lossy(&x.unwrap())
            .trim()
            .parse::<usize>()
            .unwrap()
    }) {
        state[v] += 1;
    }
    for _ in 0..80 {
        state = advance(&state);
    }
    let answer1: u64 = state.iter().sum();
    for _ in 0..(256 - 80) {
        state = advance(&state);
    }
    let answer2: u64 = state.iter().sum();
    println!("Answer 1: {}", answer1);
    println!("Answer 2: {}", answer2);
    Ok(())
}

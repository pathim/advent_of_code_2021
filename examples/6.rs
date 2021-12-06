use aoc2021::{get_input, Error};
use std::io::{BufRead, BufReader};

fn advance(state: &mut [u64], spawn_index: &mut usize) {
    let cycle_index = (*spawn_index + 7) % 9;
    state[cycle_index] += state[*spawn_index];
    *spawn_index = (*spawn_index + 1) % 9;
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
    let mut spawn_index = 0usize;
    for _ in 0..80 {
        advance(&mut state, &mut spawn_index);
    }
    let answer1: u64 = state.iter().sum();
    for _ in 0..(256 - 80) {
        advance(&mut state, &mut spawn_index);
    }
    let answer2: u64 = state.iter().sum();
    println!("Answer 1: {}", answer1);
    println!("Answer 2: {}", answer2);
    Ok(())
}

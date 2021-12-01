use aoc2021::get_input;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), ureq::Error> {
    let input = BufReader::new(get_input(1)?);
    let depth = input
        .lines()
        .map(|l| l.unwrap().parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    let result1 = depth
        .iter()
        .zip(depth.iter().skip(1))
        .filter(|(a, b)| b > a)
        .count();
    println!("first answer: {}", result1);
    let w_iter = depth.windows(3).map(|w| w[0] + w[1] + w[2]);
    let result2 = w_iter
        .clone()
        .zip(w_iter.skip(1))
        .filter(|(a, b)| b > a)
        .count();
    println!("second answer: {}", result2);

    Ok(())
}

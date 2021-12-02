use aoc2021::{get_input, Error};
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Error> {
    let input = BufReader::new(get_input(2)?);
    let lines = input.lines().map(|l| l.unwrap());
    let mut pos = (0, 0);
    let mut aim = 0i32;
    let mut pos2 = (0i32, 0);
    for l in lines {
        let (dir, distance) = l.split_once(" ").unwrap();
        let dist: i32 = distance.parse().unwrap();
        match dir {
            "forward" => {
                pos.0 += dist;
                pos2.0 += dist;
                pos2.1 += aim * dist;
            }
            "up" => {
                pos.1 -= dist;
                aim -= dist;
            }
            "down" => {
                pos.1 += dist;
                aim += dist;
            }
            _ => panic!("invalid direction: {}", dir),
        }
    }
    println!("Answer 1: {}", pos.0 * pos.1);
    println!("Answer 2: {}", pos2.0 * pos2.1);
    Ok(())
}

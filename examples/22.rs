use aoc2021::{get_input, Error};
use std::{
    collections::HashSet,
    io::{BufRead, BufReader},
    num::ParseIntError,
    str::FromStr,
};

#[derive(Debug)]
struct Step {
    turn_on: bool,
    range: [(i64, i64); 3],
}

impl FromStr for Step {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (on, rest) = s.split_once(' ').unwrap();
        let turn_on = on == "on";
        let mut range: [(i64, i64); 3] = Default::default();
        for (i, r) in rest.split(',').enumerate() {
            let (_, val) = r.split_once('=').unwrap();
            let (min, max) = val.split_once("..").unwrap();
            range[i] = (min.parse()?, max.parse::<i64>()? + 1);
        }

        Ok(Self { turn_on, range })
    }
}

impl Step {
    fn apply_limited(&self, cells: &mut HashSet<(i64, i64, i64)>) {
        for x in self.range[0].0.max(-50)..self.range[0].1.min(50) {
            for y in self.range[1].0.max(-50)..self.range[1].1.min(50) {
                for z in self.range[2].0.max(-50)..self.range[2].1.min(50) {
                    if self.turn_on {
                        cells.insert((x, y, z));
                    } else {
                        cells.remove(&(x, y, z));
                    }
                }
            }
        }
    }
}
fn main() -> Result<(), Error> {
    let input = BufReader::new(get_input(22)?);
    let sequence = input.lines().map(|l| l.unwrap().parse::<Step>().unwrap());
    let mut res = HashSet::new();
    for step in sequence {
        step.apply_limited(&mut res);
    }

    println!("Answer 1: {}", res.len());
    Ok(())
}

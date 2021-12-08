use aoc2021::{get_input, Error};

use std::{
    collections::HashSet,
    io::{BufRead, BufReader},
};

struct SegmentedDisplay {
    pattern: Vec<HashSet<char>>,
    output: Vec<HashSet<char>>,
}

impl std::str::FromStr for SegmentedDisplay {
    type Err = bool;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (pattern, output) = s.split_once('|').unwrap();
        let pattern = pattern
            .trim()
            .split(' ')
            .map(|s| s.chars().collect())
            .collect();
        let output = output
            .trim()
            .split(' ')
            .map(|s| s.chars().collect())
            .collect();

        Ok(Self { pattern, output })
    }
}

impl SegmentedDisplay {
    fn count_simple(&self) -> usize {
        self.output
            .iter()
            .map(HashSet::len)
            .filter(|x| *x == 2 || *x == 4 || *x == 3 || *x == 7)
            .count()
    }

    fn decode(&self) -> u32 {
        let mut number_map: [HashSet<char>; 10] = Default::default();
        // unique count of segments
        for p in &self.pattern {
            let idx = match p.len() {
                2 => 1,
                3 => 7,
                4 => 4,
                7 => 8,
                _ => 0xff,
            };
            if idx < 10 {
                number_map[idx] = p.clone();
            }
        }

        for p in &self.pattern {
            let idx = match p.len() {
                2 | 3 | 4 | 7 => continue,
                5 => {
                    if p.is_superset(&number_map[1]) {
                        3
                    } else if number_map[4]
                        .difference(&number_map[1])
                        .all(|x| p.contains(x))
                    {
                        5
                    } else {
                        2
                    }
                }
                6 => {
                    if p.is_superset(&number_map[4]) {
                        9
                    } else if p.is_superset(&number_map[1]) {
                        0
                    } else {
                        6
                    }
                }
                l @ _ => panic!("Invalid length: {}", l),
            };
            number_map[idx] = p.clone();
        }
        let mut res = 0;
        for o in &self.output {
            for (v, p) in number_map.iter().enumerate() {
                if o == p {
                    res = 10 * res + v;
                }
            }
        }
        res as u32
    }
}

fn main() -> Result<(), Error> {
    let input = BufReader::new(get_input(8)?);
    let lines = input.lines();
    let displays = lines.map(|x| x.unwrap().parse::<SegmentedDisplay>().unwrap());
    let (answer1, answer2) = displays
        .map(|x| (x.count_simple(), x.decode()))
        .reduce(|acc, val| (acc.0 + val.0, acc.1 + val.1))
        .unwrap();

    println!("Answer 1: {}", answer1);
    println!("Answer 2: {}", answer2);

    Ok(())
}

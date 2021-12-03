use aoc2021::{get_input, Error};
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone)]
struct Bindata(Vec<bool>);

#[derive(Debug)]
enum BinParseError {
    InvalidDigit(char),
}

impl From<&Bindata> for u32 {
    fn from(d: &Bindata) -> Self {
        d.0.iter().fold(0, |a, b| a * 2 + (*b as u32))
    }
}

impl std::str::FromStr for Bindata {
    type Err = BinParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut res = Vec::new();
        for c in s.chars() {
            match c {
                '1' => res.push(true),
                '0' => res.push(false),
                _ => return Err(BinParseError::InvalidDigit(c)),
            }
        }
        Ok(Self(res))
    }
}

fn count_ones(data: &Vec<Bindata>, pos: usize) -> usize {
    data.iter().filter(|d| d.0[pos]).count()
}

fn filter_oxygen(data: &mut Vec<Bindata>, pos: usize) {
    if data.len() != 1 {
        let threshold = (data.len() + 1) / 2;
        let one_count = count_ones(data, pos);
        data.retain(|d| d.0[pos] == (one_count >= threshold))
    }
}
fn filter_co2(data: &mut Vec<Bindata>, pos: usize) {
    if data.len() != 1 {
        let threshold = (data.len() + 1) / 2;
        let one_count = count_ones(data, pos);
        data.retain(|d| d.0[pos] != (one_count >= threshold))
    }
}

fn main() -> Result<(), Error> {
    let input = BufReader::new(get_input(3)?);
    let data: Vec<Bindata> = input.lines().map(|l| l.unwrap().parse().unwrap()).collect();
    let mut oxygen = data.clone();
    let mut co2 = data.clone();
    let threshold = data.len() / 2;
    let mut gamma = 0;
    let mut epsilon = 0;
    let bitcount = data[0].0.len();
    for i in 0..bitcount {
        gamma *= 2;
        epsilon *= 2;
        let cnt = count_ones(&data, i);
        if cnt > threshold {
            gamma += 1;
        } else {
            epsilon += 1;
        }
        filter_oxygen(&mut oxygen, i);
        filter_co2(&mut co2, i);
    }
    assert_eq!(oxygen.len(), 1);
    assert_eq!(co2.len(), 1);
    let oxy_value: u32 = oxygen.first().unwrap().into();
    let co2_value: u32 = co2.first().unwrap().into();
    println!("Answer 1: {}", gamma * epsilon);
    println!("Answer 2: {}", oxy_value * co2_value);

    Ok(())
}

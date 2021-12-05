use aoc2021::{get_input, Error};
use std::{
    collections::{HashMap, HashSet},
    io::{BufRead, BufReader},
    num::ParseIntError,
};

#[derive(Debug, Clone)]
struct Line {
    pub points: HashSet<(i32, i32)>,
}

impl Line {
    fn new(x1: i32, y1: i32, x2: i32, y2: i32) -> Self {
        let mut points = HashSet::new();
        if (x1 - x2).abs() > (y1 - y2).abs() {
            let (x1, y1, x2, y2) = if x2 < x1 {
                (x2, y2, x1, y1)
            } else {
                (x1, y1, x2, y2)
            };
            for x in x1..=x2 {
                let y = (y2 - y1).signum() * (x - x1) + y1;
                points.insert((x, y));
            }
        } else {
            let (x1, y1, x2, y2) = if y2 < y1 {
                (x2, y2, x1, y1)
            } else {
                (x1, y1, x2, y2)
            };
            for y in y1..=y2 {
                let x = (x2 - x1).signum() * (y - y1) + x1;
                points.insert((x, y));
            }
        }
        Self { points }
    }
    fn is_horz(&self) -> bool {
        self.points
            .iter()
            .map(|x| x.0)
            .collect::<HashSet<_>>()
            .len()
            == 1
    }
    fn is_vert(&self) -> bool {
        self.points
            .iter()
            .map(|x| x.1)
            .collect::<HashSet<_>>()
            .len()
            == 1
    }
    fn is_hv(&self) -> bool {
        self.is_horz() || self.is_vert()
    }
}
#[derive(Debug)]
enum ParseLineError {
    NoArrow,
    WrongPointFormat,
    IntError(ParseIntError),
}

impl From<ParseIntError> for ParseLineError {
    fn from(v: ParseIntError) -> Self {
        Self::IntError(v)
    }
}

impl std::str::FromStr for Line {
    type Err = ParseLineError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (p1, p2) = s.split_once(" -> ").ok_or(ParseLineError::NoArrow)?;
        let (x1, y1) = p1.split_once(',').ok_or(ParseLineError::WrongPointFormat)?;
        let (x2, y2) = p2.split_once(',').ok_or(ParseLineError::WrongPointFormat)?;
        Ok(Self::new(
            x1.parse()?,
            y1.parse()?,
            x2.parse()?,
            y2.parse()?,
        ))
    }
}
fn main() -> Result<(), Error> {
    let input = BufReader::new(get_input(5)?);
    let lines: Vec<Line> = input.lines().map(|l| l.unwrap().parse().unwrap()).collect();
    let mut map = HashMap::new();

    for l in lines.iter().filter(|l| l.is_hv()) {
        for p in &l.points {
            if let Some(v) = map.get_mut(p) {
                *v += 1;
            } else {
                map.insert(p, 1);
            }
        }
    }
    let answer1 = map.values().filter(|v| **v > 1).count();
    for l in lines.iter().filter(|l| !l.is_hv()) {
        for p in &l.points {
            if let Some(v) = map.get_mut(p) {
                *v += 1;
            } else {
                map.insert(p, 1);
            }
        }
    }
    let answer2 = map.values().filter(|v| **v > 1).count();
    println!("Answer 1: {}", answer1);
    println!("Answer 2: {}", answer2);

    Ok(())
}

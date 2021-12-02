use aoc2021::{get_input, Error};
use std::{
    io::{BufRead, BufReader},
    num::ParseIntError,
};

enum Dir {
    F(i32),
    U(i32),
    D(i32),
}

#[derive(Debug)]
enum ParseDirError {
    NoInt(ParseIntError),
    InvalidDirection(String),
    NoSpace,
}

impl From<ParseIntError> for ParseDirError {
    fn from(e: ParseIntError) -> Self {
        Self::NoInt(e)
    }
}

impl std::str::FromStr for Dir {
    type Err = ParseDirError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, dist) = s.split_once(" ").ok_or(ParseDirError::NoSpace)?;
        let dist: i32 = dist.parse()?;
        match dir {
            "forward" => Ok(Dir::F(dist)),
            "up" => Ok(Dir::U(dist)),
            "down" => Ok(Dir::D(dist)),
            _ => Err(ParseDirError::InvalidDirection(dir.to_string())),
        }
    }
}

struct Pos1 {
    h: i32,
    v: i32,
}
impl Pos1 {
    fn new() -> Self {
        Self { h: 0, v: 0 }
    }
    fn do_move(self, dir: &Dir) -> Self {
        match dir {
            Dir::F(dist) => Self {
                h: self.h + dist,
                ..self
            },
            Dir::U(dist) => Self {
                v: self.v - dist,
                ..self
            },
            Dir::D(dist) => Self {
                v: self.v + dist,
                ..self
            },
        }
    }
    fn result(&self) -> i32 {
        self.h * self.v
    }
}
struct Pos2 {
    h: i32,
    v: i32,
    aim: i32,
}
impl Pos2 {
    fn new() -> Self {
        Self { h: 0, v: 0, aim: 0 }
    }
    fn do_move(self, dir: &Dir) -> Self {
        match dir {
            Dir::F(dist) => Self {
                h: self.h + dist,
                v: self.v + self.aim * dist,
                ..self
            },
            Dir::U(dist) => Self {
                aim: self.aim - dist,
                ..self
            },
            Dir::D(dist) => Self {
                aim: self.aim + dist,
                ..self
            },
        }
    }
    fn result(&self) -> i32 {
        self.h * self.v
    }
}

fn main() -> Result<(), Error> {
    let input = BufReader::new(get_input(2)?);
    let lines = input.lines().map(|l| l.unwrap());
    let dirs = lines.map(|l| l.parse().unwrap());
    let (res1, res2) = dirs.fold((Pos1::new(), Pos2::new()), |p, d| {
        (p.0.do_move(&d), p.1.do_move(&d))
    });
    println!("Answer 1: {}", res1.result());
    println!("Answer 2: {}", res2.result());
    Ok(())
}

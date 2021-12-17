use aoc2021::{get_input, Error};
use std::{
    collections::{HashMap, HashSet},
    io::{BufRead, BufReader},
};

type Coord = (i32, i32);
fn parse_arg(a: &str) -> Coord {
    let (_, v) = a.trim().split_once('=').unwrap();
    let (min, max) = v.split_once("..").unwrap();
    (min.parse().unwrap(), max.parse().unwrap())
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Time {
    Single(i32),
    OpenEnd(i32),
}

impl Time {
    fn contains(&self, other: &Self) -> bool {
        let oi = match other {
            &Time::Single(v) => v,
            &Time::OpenEnd(v) => v,
        };
        match self {
            &Time::Single(v) => v == oi,
            &Time::OpenEnd(v) => v <= oi,
        }
    }
}

fn count_times(x: i32) -> HashMap<Time, i32> {
    let mut res = HashMap::new();
    for n in 1.. {
        let num = 2 * x + n * n - n;
        let den = 2 * n;
        let v = num / den;
        let r = num % den;
        if r == 0 {
            let t = if x > 0 && n >= v {
                Time::OpenEnd(n)
            } else {
                Time::Single(n)
            };
            res.insert(t, v);
        }
        if if x > 0 { n >= v } else { v > -x } {
            return res;
        }
    }
    unreachable!()
}

fn count_times_area(x: Coord, y: Coord) -> usize {
    let mut x_map = Vec::new();
    let mut y_map = Vec::new();
    let mut res = HashSet::new();
    for x in x.0..=x.1 {
        x_map.push(count_times(x));
    }
    for y in y.0..=y.1 {
        y_map.push(count_times(y));
    }
    for x_res in &x_map {
        for y_res in &y_map {
            for (y_time, y_vel) in y_res {
                for (x_time, x_vel) in x_res {
                    if x_time.contains(y_time) {
                        let vel = (x_vel, y_vel);
                        res.insert(vel);
                    }
                }
            }
        }
    }
    res.len()
}

fn main() -> Result<(), Error> {
    /*
     * x(n)=(vx0+1)*vx0/2-((vx0-n)+1)*max(vx0-n,0)/2
     */
    let input = BufReader::new(get_input(17)?)
        .lines()
        .next()
        .unwrap()
        .unwrap();

    let (_, args) = input.split_once(':').unwrap();
    let (x_arg, y_arg) = args.split_once(',').unwrap();
    let x = parse_arg(x_arg);
    let y = parse_arg(y_arg);

    let vy0 = -y.0 - 1;
    println!("Answer 1: {}", (vy0 + 1) * vy0 / 2);
    println!("Answer 2: {}", count_times_area(x, y));

    Ok(())
}

#![feature(drain_filter)]
use aoc2021::{get_input, Error};
use std::{
    io::{BufRead, BufReader},
    num::ParseIntError,
    str::FromStr,
};

#[derive(Debug, Clone)]
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
    fn limit(mut self) -> Self {
        for (min, max) in self.range.iter_mut() {
            *min = (*min).max(-50);
            *max = (*max).min(50);
        }
        self
    }

    fn intersects(&self, other: &Self) -> bool {
        for (s, o) in self.range.iter().zip(&other.range) {
            if s.0 >= o.1 || s.1 <= o.0 {
                return false;
            }
        }
        true
    }

    fn volume(&self) -> i64 {
        self.range
            .iter()
            .map(|(min, max)| (max - min).max(0))
            .product()
    }

    fn apply_vec(self, v: &mut Vec<Step>) {
        if self.turn_on {
            self.union_vec(v);
        } else {
            self.difference_vec(v);
        }
    }

    fn union_vec(self, v: &mut Vec<Step>) {
        assert!(self.turn_on);
        let inter_idx = v.iter().position(|x| self.intersects(x));
        if let Some(inter_idx) = inter_idx {
            let inter = v.swap_remove(inter_idx);
            for n in inter.union(self) {
                n.union_vec(v);
            }
        } else {
            v.push(self);
        }
    }
    fn difference_vec(self, v: &mut Vec<Step>) {
        assert!(!self.turn_on);
        while let Some(inter_idx) = v.iter().position(|x| self.intersects(x)) {
            let inter = v.swap_remove(inter_idx);
            v.append(&mut inter.difference(self.clone()));
        }
    }

    fn union(self, other: Self) -> Vec<Self> {
        assert!(self.turn_on);
        assert!(other.turn_on);
        let mut res = Vec::new();
        if !self.intersects(&other) {
            res.push(self);
            res.push(other);
            return res;
        }
        let (smaller, larger) = if self.volume() > other.volume() {
            (other, self)
        } else {
            (self, other)
        };
        for i in 0..smaller.range.len() {
            let mut new_step = smaller.clone();
            new_step.range[i].1 = larger.range[i].0;
            if new_step.volume() > 0 {
                new_step.union_vec(&mut res);
            }
            let mut new_step = smaller.clone();
            new_step.range[i].0 = larger.range[i].1;
            if new_step.volume() > 0 {
                new_step.union_vec(&mut res);
            }
        }
        res.push(larger);
        res
    }
    fn difference(self, other: Self) -> Vec<Self> {
        assert!(self.turn_on);
        assert!(!other.turn_on);
        let mut res = Vec::new();
        if !self.intersects(&other) {
            res.push(self);
            return res;
        }
        let (smaller, larger) = (self, other);
        for i in 0..smaller.range.len() {
            let mut new_step = smaller.clone();
            new_step.range[i].1 = larger.range[i].0;
            if new_step.volume() > 0 {
                new_step.union_vec(&mut res);
            }
            let mut new_step = smaller.clone();
            new_step.range[i].0 = larger.range[i].1;
            if new_step.volume() > 0 {
                new_step.union_vec(&mut res);
            }
        }

        res
    }
}
fn main() -> Result<(), Error> {
    let input = BufReader::new(get_input(22)?);
    let sequence = input
        .lines()
        .map(|l| l.unwrap().parse::<Step>().unwrap())
        .collect::<Vec<_>>();
    let mut res1 = Vec::new();
    let mut res2 = Vec::new();
    for s in sequence {
        s.clone().limit().apply_vec(&mut res1);
        s.apply_vec(&mut res2);
    }
    println!("Answer 1: {}", res1.iter().map(|x| x.volume()).sum::<i64>());
    println!("Answer 2: {}", res2.iter().map(|x| x.volume()).sum::<i64>());

    Ok(())
}

use aoc2021::{get_input, Error};
use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
    num::ParseIntError,
    ops::{Add, AddAssign},
    str::FromStr,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Mod10(usize);

impl FromStr for Mod10 {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}

impl Add for Mod10 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self((self.0 + rhs.0 - 1) % 10 + 1)
    }
}

impl AddAssign for Mod10 {
    fn add_assign(&mut self, rhs: Self) {
        self.0 = (self.0 + rhs.0 - 1) % 10 + 1;
    }
}

impl Mod10 {
    fn new(v: usize) -> Self {
        Self((v - 1) % 10 + 1)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum GameState {
    Running { pos: [Mod10; 2], score: [usize; 2] },
    Win(usize),
}

impl GameState {
    fn new(start1: usize, start2: usize) -> Self {
        Self::Running {
            pos: [Mod10::new(start1), Mod10::new(start2)],
            score: [0, 0],
        }
    }

    fn next_states(&self, who: usize) -> HashMap<GameState, usize> {
        const DICE_SUM_WAYS: [usize; 10] = [0, 0, 0, 1, 3, 6, 7, 6, 3, 1];
        let mut res = HashMap::new();
        match self {
            Self::Running { pos, score } => {
                for (dice_result, count) in DICE_SUM_WAYS.iter().enumerate().skip(3) {
                    let mut new_pos = pos.clone();
                    let mut new_score = score.clone();
                    new_pos[who] += Mod10::new(dice_result);
                    new_score[who] += new_pos[who].0;
                    let new_state = if new_score[who] < 21 {
                        Self::Running {
                            pos: new_pos,
                            score: new_score,
                        }
                    } else {
                        Self::Win(who)
                    };
                    if let Some(value) = res.get_mut(&new_state) {
                        *value += *count;
                    } else {
                        res.insert(new_state, *count);
                    }
                }
            }
            _ => {
                res.insert(self.clone(), 1);
            }
        }
        res
    }

    fn is_over(&self) -> bool {
        match self {
            Self::Running { .. } => false,
            _ => true,
        }
    }
}

fn main() -> Result<(), Error> {
    let input = BufReader::new(get_input(21)?);
    let mut lines = input.lines().map(|l| l.unwrap());
    let p1_start = lines
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .trim()
        .parse()
        .unwrap();
    let p2_start = lines
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .trim()
        .parse()
        .unwrap();

    let mut p1 = Mod10::new(p1_start);
    let mut p2 = Mod10::new(p2_start);
    let mut score1 = 0;
    let mut score2 = 0;

    let mut die = (1..=100).cycle().map(Mod10::new);
    let mut roll = 0;

    let res1 = loop {
        let mut s = Mod10::new(10);
        for _ in 0..3 {
            s = s + die.next().unwrap();
            roll += 1;
        }
        p1 = p1 + s;
        score1 += p1.0;
        if score1 >= 1000 {
            break score2 * roll;
        }

        let mut s = Mod10::new(10);
        for _ in 0..3 {
            s = s + die.next().unwrap();
            roll += 1;
        }
        p2 = p2 + s;
        score2 += p2.0;
        if score2 >= 1000 {
            break score1 * roll;
        }
    };

    println!("Answer 1: {}", res1);

    let mut state_count = HashMap::new();
    state_count.insert(GameState::new(p1_start, p2_start), 1);
    let mut who = 0;
    while state_count.keys().any(|s| !s.is_over()) {
        let mut new_state_count = HashMap::new();
        for (state, count) in state_count.iter() {
            let next_states = state.next_states(who);
            for (next_state, next_count) in next_states.iter() {
                if let Some(s) = new_state_count.get_mut(next_state) {
                    *s += count * next_count;
                } else {
                    new_state_count.insert(next_state.clone(), count * next_count);
                }
            }
        }
        state_count = new_state_count;
        who = if who == 0 { 1 } else { 0 };
    }
    println!("Answer 2: {}", state_count.values().max().unwrap());

    Ok(())
}

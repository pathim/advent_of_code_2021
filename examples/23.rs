use aoc2021::{get_input, Error};
use std::{
    collections::{BinaryHeap, HashMap},
    io::{BufRead, BufReader},
};

const COST_MAP: [usize; 4] = [1, 10, 100, 1000];

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct State {
    holes: [[Option<usize>; 2]; 4],
    corridor: [Option<usize>; 7],
}

fn to_real_pos(pos: usize) -> usize {
    if pos < 2 {
        pos
    } else if pos > 4 {
        pos + 4
    } else {
        2 * pos - 1
    }
}

impl State {
    fn get_next(&self) -> Vec<(State, usize)> {
        let mut res = Vec::new();
        // find possible moves out of holes
        for (i, v) in self.holes.iter().enumerate() {
            if v[0] == Some(i) && v[1] == Some(i) {
                // Hole correctly filled. ignore
                continue;
            }
            if let Some(upper) = v[0] {
                for f in self.find_free_left(i + 2) {
                    let mut new_state = self.clone();
                    new_state.holes[i][0] = None;
                    new_state.corridor[f] = Some(upper);
                    let cost = to_real_pos(i + 2) - to_real_pos(f);
                    res.push((new_state, cost * COST_MAP[upper]));
                }
                for f in self.find_free_right(i + 2) {
                    let mut new_state = self.clone();
                    new_state.holes[i][0] = None;
                    new_state.corridor[f] = Some(upper);
                    let cost = to_real_pos(f) - to_real_pos(i + 2) + 2;
                    res.push((new_state, cost * COST_MAP[upper]));
                }
            } else {
                if let Some(lower) = v[1] {
                    if lower == i {
                        // lower position is correct
                        continue;
                    }
                    for f in self.find_free_left(i + 2) {
                        let mut new_state = self.clone();
                        new_state.holes[i][1] = None;
                        new_state.corridor[f] = Some(lower);
                        let cost = to_real_pos(i + 2) - to_real_pos(f) + 1;
                        res.push((new_state, cost * COST_MAP[lower]));
                    }
                    for f in self.find_free_right(i + 2) {
                        let mut new_state = self.clone();
                        new_state.holes[i][1] = None;
                        new_state.corridor[f] = Some(lower);
                        let cost = to_real_pos(f) - to_real_pos(i + 2) + 2 + 1;
                        res.push((new_state, cost * COST_MAP[lower]));
                    }
                }
            }
        }
        // find possible moves into holes
        for (i, v) in self.corridor.iter().enumerate() {
            if let Some(v) = v {
                let hole_idx = *v;
                let hv = self.holes[hole_idx];
                if hv[0].is_some() {
                    continue;
                }
                let hole_depth = if let Some(lower) = hv[1] {
                    if lower != hole_idx {
                        continue;
                    }
                    1
                } else {
                    2
                };
                if i < hole_idx + 2 {
                    if self.corridor[i + 1..hole_idx + 2]
                        .iter()
                        .all(|x| x.is_none())
                    {
                        let mut new_state = self.clone();
                        new_state.corridor[i] = None;
                        new_state.holes[hole_idx][hole_depth - 1] = Some(*v);
                        let cost = to_real_pos(hole_idx + 2) - to_real_pos(i) + hole_depth - 1;
                        res.push((new_state, cost * COST_MAP[*v]));
                    }
                } else {
                    if self.corridor[hole_idx + 2..i].iter().all(|x| x.is_none()) {
                        let mut new_state = self.clone();
                        new_state.corridor[i] = None;
                        new_state.holes[hole_idx][hole_depth - 1] = Some(*v);
                        let cost = to_real_pos(i) - to_real_pos(hole_idx + 2) + hole_depth + 1;
                        res.push((new_state, cost * COST_MAP[*v]));
                    }
                }
            }
        }
        res
    }

    fn find_free_left(&self, pos: usize) -> Vec<usize> {
        (0..pos)
            .rev()
            .take_while(|x| self.corridor[*x].is_none())
            .collect()
    }
    fn find_free_right(&self, pos: usize) -> Vec<usize> {
        (pos..7)
            .take_while(|x| self.corridor[*x].is_none())
            .collect()
    }
    fn is_finished(&self) -> bool {
        self.holes
            .iter()
            .enumerate()
            .all(|(i, x)| x.iter().all(|x| *x == Some(i)))
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Path {
    cost: usize,
    state: State,
}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn search(start: &State) -> usize {
    let mut visited = HashMap::new();
    let mut active_states = BinaryHeap::new();
    active_states.push(Path {
        state: start.clone(),
        cost: 0,
    });
    visited.insert(start.clone(), 0);
    while let Some(s) = active_states.pop() {
        if s.state.is_finished() {
            return s.cost;
        }
        let next_states = s.state.get_next();
        for ns in next_states {
            let new_cost = ns.1 + s.cost;
            if let Some(v) = visited.get_mut(&ns.0) {
                if *v <= new_cost {
                    continue;
                } else {
                    *v = new_cost;
                }
            } else {
                visited.insert(ns.0.clone(), new_cost);
            }
            active_states.push(Path {
                state: ns.0,
                cost: new_cost,
            });
        }
    }
    unreachable!()
}

fn main() -> Result<(), Error> {
    let input = BufReader::new(get_input(23)?);
    let mut lines = input.lines().map(|l| l.unwrap());
    lines.next(); //skip wall
    assert_eq!(
        lines.next().unwrap().chars().filter(|x| *x == '.').count(),
        7 + 4
    ); // make sure length of corridor is correct
    let mut start = State {
        holes: Default::default(),
        corridor: Default::default(),
    };
    for j in 0..=1 {
        for (i, c) in lines
            .next()
            .unwrap()
            .trim()
            .chars()
            .filter(|x| *x != '#')
            .enumerate()
        {
            start.holes[i][j] = Some(c as usize - 'A' as usize);
        }
    }

    let res1 = search(&start);
    println!("Answer 1: {}", res1);

    Ok(())
}

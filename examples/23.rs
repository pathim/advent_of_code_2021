use aoc2021::{get_input, Error};
use std::{
    collections::{BinaryHeap, HashMap},
    io::{BufRead, BufReader},
};

const COST_MAP: [usize; 4] = [1, 10, 100, 1000];

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct State {
    holes: [[Option<usize>; 4]; 4],
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
            if v.iter().all(|x| *x == Some(i) || x.is_none()) {
                // Hole correctly filled. ignore
                continue;
            }
            if let Some((hp, hv)) = v.iter().enumerate().skip_while(|(_, v)| v.is_none()).next() {
                assert!(hv.is_some());
                for f in self.find_free_left(i + 2) {
                    let mut new_state = self.clone();
                    new_state.holes[i][hp] = None;
                    new_state.corridor[f] = hv.clone();
                    let cost = to_real_pos(i + 2) - to_real_pos(f) + hp;
                    res.push((new_state, cost * COST_MAP[hv.unwrap()]));
                }
                for f in self.find_free_right(i + 2) {
                    let mut new_state = self.clone();
                    new_state.holes[i][hp] = None;
                    new_state.corridor[f] = hv.clone();
                    let cost = to_real_pos(f) - to_real_pos(i + 2) + 2 + hp;
                    res.push((new_state, cost * COST_MAP[hv.unwrap()]));
                }
            }
        }
        // find possible moves into holes
        for (i, v) in self.corridor.iter().enumerate() {
            if let Some(v) = v {
                let hole_idx = *v;
                let hv = self.holes[hole_idx];
                if hv.iter().filter_map(|x| *x).any(|x| x != hole_idx) {
                    //skip if there are wrong values in the hole
                    continue;
                }
                let hole_depth = hv
                    .iter()
                    .enumerate()
                    .skip_while(|(_, x)| x.is_none())
                    .map(|(x, _)| x)
                    .next()
                    .unwrap_or(hv.len());
                assert_ne!(hole_depth, 0);
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
    let mut start2 = start.clone();
    for j in 2..=3 {
        for (i, h) in start.holes.iter_mut().enumerate() {
            h[j] = Some(i);
        }
    }
    const MIDDLE: [[usize; 2]; 4] = [[3, 3], [2, 1], [1, 0], [0, 2]];
    for (base, middle) in start2.holes.iter_mut().zip(MIDDLE.iter()) {
        base[3] = base[1];
        base[1] = Some(middle[0]);
        base[2] = Some(middle[1]);
    }
    println!("{:?}", start2);

    let res1 = search(&start);
    println!("Answer 1: {}", res1);
    let res2 = search(&start2);
    println!("Answer 1: {}", res2);

    Ok(())
}

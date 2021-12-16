use aoc2021::{get_input, Error};
use std::{
    collections::{HashMap, HashSet},
    io::{BufRead, BufReader},
};

type Coord = (i32, i32);

fn display_cost(cost: &HashMap<Coord, u32>) {
    let size = cost.keys().max_by_key(|(x, y)| x + y).unwrap().clone();
    for y in 0..=size.1 {
        for x in 0..=size.0 {
            let c = (cost[&(x, y)] + '0' as u32) as u8 as char;
            print!("{}", c);
        }
        println!("");
    }
}
fn display_path(cost: &HashMap<Coord, u32>, path: &Vec<Coord>) {
    let size = cost.keys().max_by_key(|(x, y)| x + y).unwrap().clone();
    let path = path.iter().collect::<HashSet<_>>();
    for y in 0..=size.1 {
        for x in 0..=size.0 {
            let c = if path.contains(&&(x, y)) {
                (cost[&(x, y)] + '0' as u32) as u8 as char
            } else {
                ' '
            };
            print!("{}", c);
        }
        println!("");
    }
}

fn add_tile(pos: Coord, cost: &HashMap<Coord, u32>, whole_cost: &mut HashMap<Coord, u32>) {
    let size = cost.keys().max_by_key(|(x, y)| x + y).unwrap().clone();
    for (coord, c) in cost {
        let cost_delta = (pos.0 + pos.1) as u32;
        let new_cost_raw = c + cost_delta;
        let new_cost = if new_cost_raw > 9 {
            new_cost_raw - 9
        } else {
            new_cost_raw
        };
        let new_coord = (
            coord.0 + pos.0 * (size.0 + 1),
            coord.1 + pos.1 * (size.0 + 1),
        );
        whole_cost.insert(new_coord, new_cost);
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Node {
    cost: u32,
    pos: Coord,
    dist: u32,
    prev: Option<Coord>,
}
impl Node {
    fn val(&self) -> u32 {
        self.cost + 1 * self.dist
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.val().partial_cmp(&self.val())
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.val().cmp(&self.val())
    }
}
fn do_a_star(start: Coord, target: Coord, cost: &HashMap<Coord, u32>) -> Node {
    let mut paths = std::collections::BinaryHeap::new();
    let mut nodes = HashMap::new();
    paths.push(Node {
        cost: 0,
        pos: start,
        dist: (target.0 - start.0 + target.1 - start.1) as u32,
        prev: None,
    });
    nodes.insert(
        start,
        Node {
            cost: 0,
            pos: start,
            dist: (target.0 - start.0 + target.1 - start.1) as u32,
            prev: None,
        },
    );
    loop {
        let p = paths.pop().unwrap();
        let cur_pos = p.pos;
        if cur_pos == target {
            return p;
        }
        let c = p.cost;
        let x = cur_pos.0;
        let y = cur_pos.1;
        for pos in [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)] {
            if let Some(cost) = cost.get(&pos) {
                let dx = target.0 - pos.0;
                let dy = target.1 - pos.1;
                let node = Node {
                    cost: c + cost,
                    dist: (dx + dy) as u32,
                    pos,
                    prev: Some(cur_pos),
                };
                if let Some(old) = nodes.get(&pos) {
                    if old.cost > c + cost {
                        paths.push(node.clone());
                        nodes.insert(pos, node);
                    }
                } else {
                    paths.push(node.clone());
                    nodes.insert(pos, node);
                }
            }
        }
    }
}

fn main() -> Result<(), Error> {
    let input = BufReader::new(get_input(15)?);
    let cost: HashMap<Coord, u32> = input
        .lines()
        .map(|l| l.unwrap())
        .enumerate()
        .flat_map(|(y, l)| {
            l.clone()
                .chars()
                .enumerate()
                .map(move |(x, c)| ((x as i32, y as i32), c as u32 - '0' as u32))
                .collect::<Vec<_>>()
        })
        .collect();

    let target = cost.keys().max_by_key(|(x, y)| x + y).unwrap().clone();

    let answer1 = do_a_star((0, 0), target, &cost);
    println!("Answer 1: {}", answer1.cost);

    let mut whole_cost = HashMap::new();
    for x in 0..5 {
        for y in 0..5 {
            add_tile((x, y), &cost, &mut whole_cost);
        }
    }
    let target = whole_cost
        .keys()
        .max_by_key(|(x, y)| x + y)
        .unwrap()
        .clone();

    let answer2 = do_a_star((0, 0), target, &whole_cost);
    println!("Answer 2: {}", answer2.cost);

    Ok(())
}

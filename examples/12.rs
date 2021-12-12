use aoc2021::{get_input, Error};
use std::{
    collections::{HashMap, HashSet},
    io::{BufRead, BufReader},
};

fn new_paths(from_to: &HashMap<String, HashSet<String>>, path: &Vec<String>) -> Vec<Vec<String>> {
    let last = path.last().unwrap();
    let mut res = Vec::new();
    for next in from_to[last].iter() {
        if next.chars().next().unwrap().is_lowercase() {
            if path.contains(next) {
                continue;
            }
        }
        let mut new_path = path.clone();
        new_path.push(next.clone());
        res.push(new_path);
    }
    res
}

fn cave_still_possible(path: &Vec<String>, cave: &str) -> bool {
    if cave == "end" {
        return true;
    }
    if cave.chars().next().unwrap().is_ascii_uppercase() {
        return true;
    }
    let mut unique = HashSet::new();
    let mut already_in = false;
    let mut already_double = false;
    for c in path {
        if !c.chars().next().unwrap().is_ascii_lowercase() {
            continue;
        }
        if c == cave {
            if already_in {
                return false;
            }
            already_in = true;
        }
        if !unique.insert(c) {
            already_double = true;
        }
    }
    if already_double {
        !already_in
    } else {
        true
    }
}

fn new_paths2(from_to: &HashMap<String, HashSet<String>>, path: &Vec<String>) -> Vec<Vec<String>> {
    let last = path.last().unwrap();
    let mut res = Vec::new();
    for next in from_to[last].iter() {
        if cave_still_possible(path, next) {
            let mut new_path = path.clone();
            new_path.push(next.clone());
            res.push(new_path);
        }
    }
    res
}

fn main() -> Result<(), Error> {
    let input = BufReader::new(get_input(12)?);
    let connections: Vec<(String, String)> = input
        .lines()
        .map(|l| l.unwrap())
        .map(|l| {
            let (a, b) = l.split_once('-').unwrap();
            (a.to_string(), b.to_string())
        })
        .collect();
    let mut from_to = std::collections::HashMap::new();
    for (c1, c2) in connections {
        for (a, b) in [(&c1, &c2), (&c2, &c1)] {
            if b == "start" || a == "end" {
                continue;
            }
            if !from_to.contains_key(a) {
                from_to.insert(a.clone(), HashSet::new());
            }
            let m = from_to.get_mut(a).unwrap();
            m.insert(b.clone());
        }
    }
    let mut paths = Vec::new();
    let start = vec!["start".to_string()];
    paths.push(start);
    let mut paths2 = paths.clone();

    let mut answer1 = 0;
    while !paths.is_empty() {
        let (finished, unifished) = paths
            .iter()
            .flat_map(|p| new_paths(&from_to, p))
            .partition::<Vec<_>, _>(|x| x.last().unwrap() == "end");
        answer1 += finished.len();
        paths = unifished;
    }
    println!("Answer 1: {}", answer1);

    let mut answer2 = 0;
    while !paths2.is_empty() {
        let (finished, unfished) = paths2
            .iter()
            .flat_map(|p| new_paths2(&from_to, p))
            .partition::<Vec<_>, _>(|x| x.last().unwrap() == "end");
        answer2 += finished.len();
        paths2 = unfished;
    }

    println!("Answer 2: {}", answer2);
    Ok(())
}

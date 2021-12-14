use aoc2021::{get_input, Error};
use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
};

fn count<T>(counter: &mut HashMap<T, u64>, c: &T, v: u64)
where
    T: Eq + std::hash::Hash + Clone,
{
    if let Some(count) = counter.get_mut(c) {
        *count += v;
    } else {
        counter.insert(c.clone(), v);
    }
}

fn get_min_max_diff(dual_counter: &HashMap<String, u64>, last: char) -> u64 {
    let mut counter = HashMap::new();
    counter.insert(last, 1);
    for (k, v) in dual_counter {
        count(&mut counter, &k.chars().next().unwrap(), *v);
    }

    let max = counter.values().max().unwrap();
    let min = counter.values().min().unwrap();

    max - min
}
fn main() -> Result<(), Error> {
    let input = BufReader::new(get_input(14)?);
    let mut lines = input.lines();
    let start = lines.next().unwrap().unwrap().chars().collect::<Vec<_>>();
    lines.next();
    let mut rules = HashMap::new();
    for l in lines {
        let l = l.unwrap();
        let (from, to) = l.split_once("->").unwrap();
        let mut chars = from.chars();
        let to = to.trim().chars().next().unwrap();
        let n1: String = [chars.next().unwrap(), to].iter().collect();
        let n2: String = [to, chars.next().unwrap()].iter().collect();
        rules.insert(from.trim().to_string(), (n1, n2));
    }

    let mut dual_counter = HashMap::new();
    for s in start.windows(2) {
        let s = s.iter().collect::<String>();
        count(&mut dual_counter, &s, 1);
    }

    for i in 0..40 {
        if i == 10 {
            println!(
                "Answer 1: {}",
                get_min_max_diff(&dual_counter, *start.last().unwrap())
            );
        }
        let mut new_count = HashMap::<String, u64>::new();
        for (k, v) in dual_counter.iter() {
            let (r1, r2) = &rules[k];
            count(&mut new_count, r1, *v);
            count(&mut new_count, r2, *v);
        }
        dual_counter = new_count;
    }

    println!(
        "Answer 2: {}",
        get_min_max_diff(&dual_counter, *start.last().unwrap())
    );
    Ok(())
}

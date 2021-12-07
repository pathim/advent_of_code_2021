use aoc2021::{get_input, intcode, Error};
use std::io::{BufRead, BufReader};

fn int_sum(n: i32) -> i32 {
    ((n + 1) * n) / 2
}

fn main() -> Result<(), Error> {
    let input = BufReader::new(get_input(7)?);
    let data: Vec<i32> = input
        .split(b',')
        .map(|x| {
            String::from_utf8_lossy(&x.unwrap())
                .trim()
                .parse::<i32>()
                .unwrap()
        })
        .collect();
    let mut m = intcode::Machine::from_iter(data.iter().map(|x| *x as intcode::Int));
    let easter_egg = m.run(None).unwrap();
    println!(
        "Easter egg: {}",
        String::from_iter(
            easter_egg
                .1
                .iter()
                .map(|x| char::from_u32(*x as u32).unwrap())
        )
    );

    let min = *data.iter().min().unwrap();
    let max = *data.iter().max().unwrap();
    let answer1 = (min..=max)
        .map(|n| data.iter().map(|x| (x - n).abs()).sum::<i32>())
        .min()
        .unwrap();
    let answer2 = (min..=max)
        .map(|n| data.iter().map(|x| int_sum((x - n).abs())).sum::<i32>())
        .min()
        .unwrap();

    println!("Answer 1: {}", answer1);
    println!("Answer 2: {}", answer2);

    Ok(())
}

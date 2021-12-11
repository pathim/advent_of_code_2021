use aoc2021::{get_input, Error};
use std::io::{BufRead, BufReader};

#[derive(Clone, Copy, Debug, Default)]
struct Octopus(u32);

impl TryFrom<char> for Octopus {
    type Error = char;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        if value.is_ascii_digit() {
            Ok(Self(value as u32 - '0' as u32))
        } else {
            Err(value)
        }
    }
}
impl Octopus {
    fn inc(&mut self) -> bool {
        self.0 += 1;
        self.0 == 10
    }
    fn reset(&mut self) {
        if self.0 >= 10 {
            self.0 = 0;
        }
    }
}

fn increase(area: &mut [[Octopus; 10]; 10], x: i32, y: i32) -> u32 {
    let mut flashes = 0;
    if let Some(octo) = area.get_mut(y as usize).and_then(|r| r.get_mut(x as usize)) {
        if octo.inc() {
            flashes += 1;
            for (dx, dy) in [-1, 0, 1]
                .iter()
                .flat_map(|dx| [-1, 0, 1].iter().map(|dy| (*dx, *dy)))
                .filter(|v| *v != (0, 0))
            {
                flashes += increase(area, x + dx, y + dy);
            }
        }
    }
    flashes
}

fn step(area: &mut [[Octopus; 10]; 10]) -> u32 {
    let mut flashes = 0;
    for y in 0..10 {
        for x in 0..10 {
            flashes += increase(area, x as i32, y as i32);
        }
    }
    for y in 0..10 {
        for x in 0..10 {
            area[y][x].reset();
        }
    }
    flashes
}

fn main() -> Result<(), Error> {
    let input = BufReader::new(get_input(11)?);
    let mut area: [[Octopus; 10]; 10] = Default::default();
    for (y, l) in input.lines().enumerate() {
        for (x, c) in l.unwrap().chars().enumerate() {
            area[y][x] = c.try_into().unwrap();
        }
    }
    let mut total_flashes = 0;
    let max_flashes = 100;
    for _ in 0..100 {
        total_flashes += step(&mut area);
    }
    println!("Answer 1: {}", total_flashes);
    for n in 1.. {
        if step(&mut area) == max_flashes {
            println!("Answer 2: {}", n + 100);
            break;
        }
    }
    Ok(())
}

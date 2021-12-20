use aoc2021::{get_input, Error};
use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
};

type Coord = (i64, i64);

fn get_pixel(image: &HashMap<Coord, bool>, algo: &Vec<bool>, (x, y): Coord, outside: bool) -> bool {
    let mut index = 0;
    for dy in [-1, 0, 1] {
        for dx in [-1, 0, 1] {
            let c = (x + dx, y + dy);
            index <<= 1;
            let val = image.get(&c).copied().unwrap_or(outside);
            if val {
                index |= 1;
            }
        }
    }
    algo[index]
}

fn get_size(image: &HashMap<Coord, bool>) -> (Coord, Coord) {
    let x_min = image.keys().map(|x| x.0).min().unwrap();
    let x_max = image.keys().map(|x| x.0).max().unwrap();
    let y_min = image.keys().map(|x| x.1).min().unwrap();
    let y_max = image.keys().map(|x| x.1).max().unwrap();
    ((x_min, y_min), (x_max, y_max))
}

fn enhance(image: &HashMap<Coord, bool>, algo: &Vec<bool>, outside: bool) -> HashMap<Coord, bool> {
    let ((x_min, y_min), (x_max, y_max)) = get_size(image);
    let mut new_image = HashMap::new();
    for x in x_min - 1..=x_max + 1 {
        for y in y_min - 1..=y_max + 1 {
            let coord = (x, y);
            new_image.insert(coord, get_pixel(image, algo, coord, outside));
        }
    }
    new_image
}

fn main() -> Result<(), Error> {
    let input = BufReader::new(get_input(20)?);
    let mut lines = input.lines();
    let algo = lines
        .next()
        .unwrap()
        .unwrap()
        .chars()
        .map(|c| c == '#')
        .collect::<Vec<_>>();
    lines.next(); // discard empty line
    let mut image = HashMap::new();
    for (y, l) in lines.map(|l| l.unwrap()).enumerate() {
        for (x, c) in l.chars().enumerate() {
            let coord = (x as i64, y as i64);
            image.insert(coord, c == '#');
        }
    }

    image = enhance(&mut image, &algo, false);
    image = enhance(&mut image, &algo, algo[0]);
    println!("Answer 1: {}", image.values().filter(|&&x| x).count());
    for _ in 0..24 {
        image = enhance(&mut image, &algo, false);
        image = enhance(&mut image, &algo, algo[0]);
    }
    println!("Answer 2: {}", image.values().filter(|&&x| x).count());

    Ok(())
}

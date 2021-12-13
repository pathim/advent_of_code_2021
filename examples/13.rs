use aoc2021::{get_input, Error};
use std::{
    collections::HashSet,
    io::{BufRead, BufReader},
};

fn fold_x(dots: HashSet<(i32, i32)>, pos: i32) -> HashSet<(i32, i32)> {
    let mut res = HashSet::new();
    for d in dots {
        res.insert(if d.0 < pos { d } else { (2 * pos - d.0, d.1) });
    }
    res
}
fn fold_y(dots: HashSet<(i32, i32)>, pos: i32) -> HashSet<(i32, i32)> {
    let mut res = HashSet::new();
    for d in dots {
        res.insert(if d.1 < pos { d } else { (d.0, 2 * pos - d.1) });
    }
    res
}

fn fold(dots: HashSet<(i32, i32)>, (is_x, pos): (bool, i32)) -> HashSet<(i32, i32)> {
    if is_x {
        fold_x(dots, pos)
    } else {
        fold_y(dots, pos)
    }
}

fn draw(dots: &HashSet<(i32, i32)>) {
    let mut image = Vec::new();
    for d in dots {
        while image.len() <= d.1 as usize {
            image.push(Vec::new());
        }
        let line = &mut image[d.1 as usize];
        while line.len() <= d.0 as usize {
            line.push(' ');
        }
        line[d.0 as usize] = '█';
    }
    for line in image {
        for c in line {
            print!("{}", c);
        }
        println!("");
    }
}

fn main() -> Result<(), Error> {
    let input = BufReader::new(get_input(13)?);
    let mut dots = HashSet::new();
    let mut folds = Vec::new();
    for l in input.lines() {
        let l = l.unwrap();
        if l.starts_with("fold") {
            let (_, f) = l.rsplit_once(' ').unwrap();
            let (dir, pos) = f.split_once('=').unwrap();
            folds.push((dir == "x", pos.parse::<i32>().unwrap()));
        } else {
            if let Some((x, y)) = l.split_once(',') {
                dots.insert((x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap()));
            }
        }
    }
    let mut f_it = folds.into_iter();
    dots = fold(dots, f_it.next().unwrap());
    println!("Answer 1: {}", dots.len());
    for f in f_it {
        dots = fold(dots, f);
    }
    println!("Answer 2:");
    draw(&dots);

    Ok(())
}

use aoc2021::{get_input, Error};
use std::io::{BufRead, BufReader};

fn get_pixel(image: &Vec<Vec<bool>>, algo: &Vec<bool>, (x, y): (i64, i64), outside: bool) -> bool {
    let mut index = 0;
    for dy in [-1, 0, 1] {
        for dx in [-1, 0, 1] {
            let x_pos = x + dx;
            let y_pos = y + dy;
            index <<= 1;
            let val = image
                .get(y_pos as usize)
                .and_then(|line| line.get(x_pos as usize))
                .copied()
                .unwrap_or(outside);
            if val {
                index |= 1;
            }
        }
    }
    algo[index]
}

fn enhance(image: &Vec<Vec<bool>>, algo: &Vec<bool>, outside: bool) -> Vec<Vec<bool>> {
    let mut new_image = Vec::new();
    for y in -1..=image.len() as i64 {
        let mut nv = Vec::new();
        for x in -1..=image[0].len() as i64 {
            let coord = (x, y);
            nv.push(get_pixel(image, algo, coord, outside));
        }
        new_image.push(nv);
    }
    new_image
}

fn count_light(image: &Vec<Vec<bool>>) -> usize {
    image.iter().flatten().filter(|&&x| x).count()
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
    let mut image = Vec::new();
    for l in lines.map(|l| l.unwrap()) {
        let mut v = Vec::new();
        for c in l.chars() {
            v.push(c == '#');
        }
        image.push(v);
    }

    image = enhance(&mut image, &algo, false);
    image = enhance(&mut image, &algo, algo[0]);
    println!("Answer 1: {}", count_light(&image));
    for _ in 0..24 {
        image = enhance(&mut image, &algo, false);
        image = enhance(&mut image, &algo, algo[0]);
    }
    println!("Answer 1: {}", count_light(&image));

    Ok(())
}

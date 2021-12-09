use aoc2021::{get_input, Error};

use std::io::{BufRead, BufReader};

fn get_risk(heightmap: &Vec<Vec<u32>>, x: usize, y: usize) -> Option<u32> {
    let point_height = heightmap[y][x];
    let mut low_count = 0;
    for dx in [-1, 1] {
        if heightmap
            .get(y)
            .and_then(|v| v.get((x as i32 + dx) as usize))
            .map(|o| *o > point_height)
            .unwrap_or(true)
        {
            low_count += 1;
        }
    }
    for dy in [-1, 1] {
        if heightmap
            .get((y as i32 + dy) as usize)
            .and_then(|v| v.get(x))
            .map(|o| *o > point_height)
            .unwrap_or(true)
        {
            low_count += 1;
        }
    }
    if low_count == 4 {
        Some(point_height + 1)
    } else {
        None
    }
}

fn get_basin_size(heightmap: &mut Vec<Vec<u32>>, x: i32, y: i32) -> u32 {
    if let Some(h) = heightmap
        .get_mut(y as usize)
        .and_then(|v| v.get_mut(x as usize))
    {
        if *h == 9 {
            return 0;
        }
        *h = 9;
    } else {
        return 0;
    }
    let mut res = 1;
    for dx in [-1, 1] {
        res += get_basin_size(heightmap, x + dx, y);
    }
    for dy in [-1, 1] {
        res += get_basin_size(heightmap, x, y + dy);
    }
    res
}

fn main() -> Result<(), Error> {
    let input = BufReader::new(get_input(9)?);
    let heightmap: Vec<Vec<u32>> = input
        .lines()
        .map(|l| {
            l.unwrap()
                .chars()
                .map(|c| c.to_string().parse().unwrap())
                .collect()
        })
        .collect();
    let mut basinmap = heightmap.clone();
    let mut basins = Vec::new();

    let height = heightmap.len();
    let width = heightmap[0].len();
    let mut answer1 = 0;
    for x in 0..width {
        for y in 0..height {
            if let Some(risk) = get_risk(&heightmap, x, y) {
                answer1 += risk;
                basins.push(get_basin_size(&mut basinmap, x as i32, y as i32));
            }
        }
    }
    println!("Answer 1: {}", answer1);

    basins.sort();
    println!(
        "Answer 2: {:?}",
        basins.iter().rev().take(3).product::<u32>()
    );

    Ok(())
}

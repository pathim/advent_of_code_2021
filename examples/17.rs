use aoc2021::{get_input, Error};
use std::io::{BufRead, BufReader};

type Coord = (i32, i32);
fn parse_arg(a: &str) -> Coord {
    let (_, v) = a.trim().split_once('=').unwrap();
    let (min, max) = v.split_once("..").unwrap();
    (min.parse().unwrap(), max.parse().unwrap())
}

#[derive(Debug, Clone)]
struct Projectile {
    vel: Coord,
    path: Vec<Coord>,
}

impl Projectile {
    fn new(vel: Coord) -> Self {
        let path = vec![(0, 0)];
        Self { vel, path }
    }

    fn step(&mut self) -> Coord {
        let mut pos = self.pos();
        pos.0 += self.vel.0;
        pos.1 += self.vel.1;
        self.vel.0 -= self.vel.0.signum();
        self.vel.1 -= 1;
        self.path.push(pos);
        pos
    }

    fn run_to_end(&mut self, limits: Coord) {
        loop {
            let pos = self.step();
            if pos.0 >= limits.0 || pos.1 <= limits.1 {
                break;
            }
        }
    }
    fn pos(&self) -> Coord {
        *self.path.last().unwrap()
    }

    fn max_height(&self) -> i32 {
        self.path.iter().map(|x| x.1).max().unwrap()
    }

    fn check_hit(&self, target_x: Coord, target_y: Coord) -> bool {
        self.check_hit_x(target_x) && self.check_hit_y(target_y)
    }
    fn check_hit_x(&self, target_x: Coord) -> bool {
        let len = self.path.len();
        for n in 1..=2 {
            if let Some(p) = self.path.get(len - n) {
                if p.0 >= target_x.0 && p.0 <= target_x.1 {
                    return true;
                }
            }
        }
        false
    }
    fn check_hit_y(&self, target_y: Coord) -> bool {
        let len = self.path.len();
        for n in 1..=2 {
            if let Some(p) = self.path.get(len - n) {
                if p.1 >= target_y.0 && p.1 <= target_y.1 {
                    return true;
                }
            }
        }
        false
    }
}

fn main() -> Result<(), Error> {
    /*
     * x(n)=(vx0+1)*vx0/2-((vx0-n)+1)*max(vx0-n,0)/2
     */
    let input = BufReader::new(get_input(17)?)
        .lines()
        .next()
        .unwrap()
        .unwrap();
    //let input="target area: x=20..30, y=-10..-5";
    let (_, args) = input.split_once(':').unwrap();
    let (x_arg, y_arg) = args.split_once(',').unwrap();
    let x = parse_arg(x_arg);
    let y = parse_arg(y_arg);

    let vx0 = (((((x.0 + x.1) * 4 + 1) as f64).sqrt() - 1.0) / 2.0).floor() as i32;
    let vy0 = -y.0 - 1;
    println!("Answer 1: {}", (vy0 + 1) * vy0 / 2);

    Ok(())
}

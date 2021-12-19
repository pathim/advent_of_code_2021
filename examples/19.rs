use aoc2021::{get_input, Error};
use std::{
    cell::Cell,
    collections::{HashMap, HashSet},
    io::{BufRead, BufReader},
    ops::{Add, Sub},
    str::FromStr,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord([i64; 3]);
impl FromStr for Coord {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(',');
        let x = parts.next().unwrap().parse().unwrap();
        let y = parts.next().unwrap().parse().unwrap();
        let z = parts.next().unwrap().parse().unwrap();
        Ok(Self([x, y, z]))
    }
}
impl Sub for &Coord {
    type Output = Coord;

    fn sub(self, rhs: Self) -> Self::Output {
        Coord([
            self.0[0] - rhs.0[0],
            self.0[1] - rhs.0[1],
            self.0[2] - rhs.0[2],
        ])
    }
}
impl Sub for Coord {
    type Output = Coord;

    fn sub(self, rhs: Self) -> Self::Output {
        &self - &rhs
    }
}
impl Add for &Coord {
    type Output = Coord;

    fn add(self, rhs: Self) -> Self::Output {
        Coord([
            self.0[0] + rhs.0[0],
            self.0[1] + rhs.0[1],
            self.0[2] + rhs.0[2],
        ])
    }
}
impl Add for Coord {
    type Output = Coord;

    fn add(self, rhs: Self) -> Self::Output {
        &self + &rhs
    }
}
impl Coord {
    fn normalize(&self) -> Self {
        let mut c = [self.0[0].abs(), self.0[1].abs(), self.0[2].abs()];
        c.sort();
        Self([c[0], c[1], c[2]])
    }
    fn transform(&self, transform: &Self) -> Self {
        let mut res = [0; 3];
        for i in 0..3 {
            res[i] = transform.0[i].signum() * self.0[transform.0[i].abs() as usize - 1];
        }
        Self(res)
    }
    fn calc_transform(&self, to: &Self) -> Self {
        let mut res = [0; 3];
        for i in 0..3 {
            for j in 0..3 {
                if self.0[j].abs() == to.0[i].abs() {
                    res[i] = (self.0[j].signum() * to.0[i].signum()) * (j as i64 + 1);
                }
            }
        }
        Self(res)
    }
}
#[derive(Debug)]
struct Sensor {
    position: Cell<Option<Coord>>,
    transformation: Cell<Coord>,
    beacons: Vec<Coord>,
    diffs_inv: HashMap<Coord, (usize, usize)>,
}

impl Sensor {
    fn new(iter: &mut impl Iterator<Item = String>) -> Self {
        let mut beacons = Vec::new();
        while let Some(l) = iter.next() {
            if l.is_empty() {
                break;
            }
            beacons.push(l.parse().unwrap());
        }
        let diffs_inv = HashMap::new();

        let position = Cell::new(None);
        let transformation = Cell::new(Coord([1, 2, 3]));
        let mut res = Self {
            position,
            transformation,
            beacons,
            diffs_inv,
        };
        res.fill_diff_inv();
        res
    }
    fn fill_diff_inv(&mut self) {
        for (i, c1) in self.beacons.iter().enumerate() {
            for (j, c2) in self.beacons.iter().enumerate() {
                if i != j {
                    let diff: Coord = c1 - c2;
                    let norm = diff.normalize();
                    self.diffs_inv.insert(norm, (i, j));
                }
            }
        }
    }

    fn count_same(&self, other: &Self) -> usize {
        let k = self.diffs_inv.keys().copied().collect::<HashSet<_>>();
        let o = other.diffs_inv.keys().copied().collect::<HashSet<_>>();
        k.intersection(&o).count()
    }

    fn find_overlapping<'a>(&self, other: &'a Vec<Self>) -> Option<&'a Self> {
        for o in other {
            if o.position.get().is_some() {
                continue;
            }
            if self.count_same(o) >= 66 {
                return Some(o);
            }
        }
        None
    }

    fn find_position(&self, reference: &Self) {
        let mut coord_index_candidates: HashMap<usize, HashSet<usize>> = HashMap::new();
        for (k, v) in self.diffs_inv.iter() {
            if let Some(v2) = reference.diffs_inv.get(k) {
                if let Some(cic) = coord_index_candidates.get_mut(&v.0) {
                    cic.retain(|&c| c == v2.0 || c == v2.1);
                } else {
                    coord_index_candidates.insert(v.0, HashSet::from_iter([v2.0, v2.1]));
                }
                if let Some(cic) = coord_index_candidates.get_mut(&v.1) {
                    cic.retain(|&c| c == v2.0 || c == v2.1);
                } else {
                    coord_index_candidates.insert(v.1, HashSet::from_iter([v2.0, v2.1]));
                }
            }
        }
        let mut coord_iter = coord_index_candidates.into_iter();
        let beacon_index = coord_iter.next().unwrap();
        let c1s = self.beacons[beacon_index.0];
        let c1o = reference.beacons[beacon_index.1.into_iter().next().unwrap()];
        let beacon_index = coord_iter.next().unwrap();
        let c2s = self.beacons[beacon_index.0];
        let c2o = reference.beacons[beacon_index.1.into_iter().next().unwrap()];

        let tr = reference.transformation.get();
        let t = (c1s - c2s).calc_transform(&(c1o - c2o).transform(&tr));
        self.position.set(Some(
            reference.position.get().unwrap() + c1o.transform(&tr) - c1s.transform(&t),
        ));
        self.transformation.set(t);
    }

    fn absolute_beacons<'a>(&'a self) -> impl Iterator<Item = Coord> + 'a {
        let t = self.transformation.get();
        let p = self.position.get().unwrap();
        self.beacons.iter().map(move |c| c.transform(&t) + p)
    }
}

fn main() -> Result<(), Error> {
    let input = BufReader::new(get_input(19)?);
    let mut lines = input.lines().map(|l| l.unwrap());
    let mut sensors = Vec::new();
    while let Some(l) = lines.next() {
        if !l.starts_with("---") {
            panic!("Unexpected line: '{}'", l);
        }
        sensors.push(Sensor::new(&mut lines));
    }
    sensors[0].position = Cell::new(Some(Coord([0, 0, 0])));

    while sensors.iter().any(|s| s.position.get().is_none()) {
        for s1 in sensors.iter().filter(|s| s.position.get().is_some()) {
            while let Some(s2) = s1.find_overlapping(&sensors) {
                s2.find_position(s1);
            }
        }
    }
    let mut beacons = HashSet::new();
    for s in sensors.iter() {
        for b in s.absolute_beacons() {
            beacons.insert(b);
        }
    }
    println!("Answer 1: {}", beacons.len());

    let max_dist = sensors
        .iter()
        .map(|x| x.position.get().unwrap())
        .flat_map(|x| sensors.iter().map(move |y| y.position.get().unwrap() - x))
        .map(|x| x.0.iter().map(|a| a.abs()).sum::<i64>())
        .max()
        .unwrap();
    println!("Answer 2: {}", max_dist);

    Ok(())
}

use aoc2021::{get_input, Error};
use std::{
    fmt::Display,
    io::{BufRead, BufReader},
    iter::Sum,
    num::ParseIntError,
    str::FromStr,
};

#[derive(Debug, Clone)]
enum Num {
    Basic(i64),
    Pair(Box<Num>, Box<Num>),
}

#[derive(Debug)]
enum NumParseError {
    ImbalancedBrackets,
    NumError(ParseIntError),
}

impl From<ParseIntError> for NumParseError {
    fn from(e: ParseIntError) -> Self {
        Self::NumError(e)
    }
}

impl FromStr for Num {
    type Err = NumParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with('[') {
            if !s.ends_with(']') {
                return Err(NumParseError::ImbalancedBrackets);
            }
            // Find correct comma
            let inner = &s[1..s.len() - 1];
            let mut comma_pos = 0;
            let mut level = 0;
            for (idx, c) in inner.chars().enumerate() {
                if c == '[' {
                    level += 1;
                }
                if c == ']' {
                    level -= 1;
                }
                if c == ',' && level == 0 {
                    comma_pos = idx;
                    break;
                }
            }
            let first = &inner[..comma_pos];
            let second = &inner[comma_pos + 1..];
            let first_num: Num = first.parse()?;
            let second_num: Num = second.parse()?;
            Ok(Self::Pair(Box::new(first_num), Box::new(second_num)))
        } else {
            let value: i64 = s.parse()?;
            Ok(Self::Basic(value))
        }
    }
}
impl std::ops::Add for Num {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut res = Num::Pair(Box::new(self), Box::new(rhs));
        res.reduce();
        res
    }
}
impl Sum for Num {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|a, b| a + b).unwrap()
    }
}
impl Display for Num {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Basic(v) => write!(f, "{}", v),
            Self::Pair(v1, v2) => write!(f, "[{},{}]", *v1, *v2),
        }
    }
}
impl Num {
    fn reduce(&mut self) {
        let mut cont = true;
        while cont {
            if self.explode(0).2 {
                cont = true;
            } else {
                cont = self.split();
            }
        }
    }
    fn split(&mut self) -> bool {
        match self {
            Self::Basic(v) => {
                if *v >= 10 {
                    *self = Self::Pair(
                        Box::new(Self::Basic(*v / 2)),
                        Box::new(Self::Basic((*v + 1) / 2)),
                    );
                    true
                } else {
                    false
                }
            }
            Self::Pair(v1, v2) => {
                if !v1.split() {
                    v2.split()
                } else {
                    true
                }
            }
        }
    }

    fn explode(&mut self, level: u32) -> (Option<i64>, Option<i64>, bool) {
        let mut res = (None, None, false);
        match self {
            Self::Basic(_) => {}
            Self::Pair(v1, v2) => {
                if level == 4 {
                    res = (Some(v1.get_basic_value()), Some(v2.get_basic_value()), true);
                    *self = Self::Basic(0);
                    return res;
                }
                let (r1, r2, has_exp) = v1.explode(level + 1);
                res.2 = has_exp;
                if r1.is_some() || r2.is_some() {
                    if let Some(val) = r2 {
                        v2.add_exploded_value_left(val);
                    }
                    res.0 = r1;
                    res.2 = true;
                } else {
                    if !has_exp {
                        let (r1, r2, has_exp) = v2.explode(level + 1);
                        if let Some(val) = r1 {
                            v1.add_exploded_value_right(val);
                        }
                        res.1 = r2;
                        res.2 = has_exp;
                    }
                }
            }
        }
        res
    }
    fn get_basic_value(&self) -> i64 {
        match self {
            Self::Basic(v) => *v,
            _ => panic!("Not a Basic value: {:?}", self),
        }
    }
    fn add_exploded_value_right(&mut self, val: i64) {
        match self {
            Num::Basic(v) => *v += val,
            Num::Pair(_, v2) => v2.add_exploded_value_right(val),
        };
    }
    fn add_exploded_value_left(&mut self, val: i64) {
        match self {
            Num::Basic(v) => *v += val,
            Num::Pair(v1, _) => v1.add_exploded_value_left(val),
        };
    }

    fn magnitude(&self) -> i64 {
        match self {
            Self::Basic(v) => *v,
            Self::Pair(v1, v2) => 3 * v1.magnitude() + 2 * v2.magnitude(),
        }
    }
}
fn main() -> Result<(), Error> {
    let input = BufReader::new(get_input(18)?);
    let numbers = input
        .lines()
        .map(|l| l.unwrap().parse::<Num>().unwrap())
        .collect::<Vec<_>>();
    let res = numbers.iter().cloned().sum::<Num>();
    println!("Answer 1: {}", res.magnitude());
    let mut res2 = 0;
    for (i1, v1) in numbers.iter().enumerate() {
        for (i2, v2) in numbers.iter().cloned().enumerate() {
            if i1 == i2 {
                continue;
            }
            let mag = (v1.clone() + v2).magnitude();
            res2 = res2.max(mag);
        }
    }

    println!("Answer 2: {}", res2);

    Ok(())
}

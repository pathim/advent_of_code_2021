use aoc2021::{get_input, Error};
use std::io::{BufRead, BufReader};
#[derive(Debug)]
enum Reg {
    W,
    X,
    Y,
    Z,
}
impl std::str::FromStr for Reg {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.chars().next().unwrap() {
            'w' => Self::W,
            'x' => Self::X,
            'y' => Self::Y,
            'z' => Self::Z,
            c @ _ => panic!("Invalid register: {}", c),
        })
    }
}
#[derive(Debug)]
enum Val {
    R(Reg),
    I(i64),
}
impl std::str::FromStr for Val {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.parse::<i64>() {
            Ok(v) => Self::I(v),
            Err(_) => Self::R(s.parse().unwrap()),
        })
    }
}
#[derive(Debug)]
enum Command {
    Inp(Reg),
    Add(Reg, Val),
    Mul(Reg, Val),
    Div(Reg, Val),
    Mod(Reg, Val),
    Eql(Reg, Val),
}
impl std::str::FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cmd, args) = s.split_once(' ').unwrap();
        if cmd == "inp" {
            return Ok(Self::Inp(args.parse().unwrap()));
        }
        let (arg1, arg2) = args.split_once(' ').unwrap();
        let arg1 = arg1.parse().unwrap();
        let arg2 = arg2.parse().unwrap();
        Ok(match cmd {
            "add" => Self::Add(arg1, arg2),
            "mul" => Self::Mul(arg1, arg2),
            "div" => Self::Div(arg1, arg2),
            "mod" => Self::Mod(arg1, arg2),
            "eql" => Self::Eql(arg1, arg2),
            c @ _ => panic!("Invalid command: {}", c),
        })
    }
}

#[derive(Debug, Default, Clone)]
struct ALU {
    w: i64,
    x: i64,
    y: i64,
    z: i64,
    input: [i64; 14],
    input_index: usize,
}

impl ALU {
    fn reset(&mut self) {
        self.input_index = 0;
        self.w = 0;
        self.x = 0;
        self.w = 0;
        self.z = 0;
    }
    fn execute(&mut self, cmd: &Command) {
        match cmd {
            Command::Add(t, v) => self.set_reg(t, self.get_reg(t) + self.get_val(v)),
            Command::Mul(t, v) => self.set_reg(t, self.get_reg(t) * self.get_val(v)),
            Command::Div(t, v) => self.set_reg(t, self.get_reg(t) / self.get_val(v)),
            Command::Mod(t, v) => self.set_reg(t, self.get_reg(t) % self.get_val(v)),
            Command::Eql(t, v) => self.set_reg(t, (self.get_reg(t) == self.get_val(v)) as i64),
            Command::Inp(r) => {
                self.set_reg(r, self.input[self.input_index]);
                self.input_index += 1;
            }
        }
    }
    fn execute_vec(&mut self, cmds: &Vec<Command>) {
        for c in cmds {
            self.execute(c);
        }
    }

    fn get_val(&self, v: &Val) -> i64 {
        match v {
            Val::I(x) => *x,
            Val::R(r) => self.get_reg(r),
        }
    }
    fn get_reg(&self, r: &Reg) -> i64 {
        match r {
            Reg::W => self.w,
            Reg::X => self.x,
            Reg::Y => self.y,
            Reg::Z => self.z,
        }
    }

    fn set_reg(&mut self, r: &Reg, val: i64) {
        match r {
            Reg::W => self.w = val,
            Reg::X => self.x = val,
            Reg::Y => self.y = val,
            Reg::Z => self.z = val,
        }
    }
}

fn change_input(input: &mut [i64; 14], digit: usize) {
    input[digit] -= 1;
    if input[digit] < 0 {
        input[digit] = 9;
        change_input(input, digit - 1);
    }
}

fn main() -> Result<(), Error> {
    let input = BufReader::new(get_input(24)?);
    let mut alu = ALU::default();
    alu.input = [1, 2, 9, 9, 6, 9, 9, 7, 8, 2, 9, 3, 9, 9]; // largest
    let mut alu2 = ALU::default();
    alu2.input = [1, 1, 8, 4, 1, 2, 3, 1, 1, 1, 7, 1, 8, 9]; // smallest

    for l in input.lines() {
        let cmd = l.unwrap().parse::<Command>().unwrap();
        alu.execute(&cmd);
        alu2.execute(&cmd);
    }
    assert_eq!(alu.z, 0);
    assert_eq!(alu2.z, 0);

    alu.input.iter();
    let res1 = String::from_iter(alu.input.iter().map(|x| format!("{}", x)));
    let res2 = String::from_iter(alu2.input.iter().map(|x| format!("{}", x)));

    println!("Answer 1: {}", res1);
    println!("Answer 2: {}", res2);

    Ok(())
}

use aoc2021::{get_input, Error};
use std::io::{BufRead, BufReader};

struct NibbleSource {
    nibbles: Vec<u8>,
    pos: usize,
}

fn hex_digit_to_value(d: char) -> Option<u8> {
    if d.is_ascii_hexdigit() {
        let d = (d as u8) & !(1 << 7);
        Some(if d < 'A' as u8 {
            d - '0' as u8
        } else {
            d - 'A' as u8 + 10
        })
    } else {
        None
    }
}

trait BitSource {
    fn next_bit(&mut self) -> Option<bool>;
    fn get(&mut self, n: usize) -> Option<u64> {
        let mut res = 0;
        for _ in 0..n {
            let bit = self.next_bit()?;
            res = (res << 1) + bit as u64;
        }
        Some(res)
    }
}
impl NibbleSource {
    fn new(it: impl IntoIterator<Item = char>) -> Self {
        let mut nibbles = it
            .into_iter()
            .map_while(|v| hex_digit_to_value(v))
            .collect::<Vec<_>>();
        nibbles.reverse();
        let pos = 4;
        Self { nibbles, pos }
    }
}
impl BitSource for NibbleSource {
    fn next_bit(&mut self) -> Option<bool> {
        if self.pos == 0 {
            self.nibbles.pop();
            self.pos = 4;
        }
        self.pos -= 1;
        self.nibbles.last().map(|val| (val >> self.pos) & 1 == 1)
    }
}

struct LimitedSource<'a> {
    source: &'a mut dyn BitSource,
    count: usize,
}

impl<'a> LimitedSource<'a> {
    fn new(count: usize, source: &'a mut dyn BitSource) -> Self {
        Self { source, count }
    }
}

impl<'a> BitSource for LimitedSource<'a> {
    fn next_bit(&mut self) -> Option<bool> {
        if self.count == 0 {
            None
        } else {
            self.count -= 1;
            self.source.next_bit()
        }
    }
}

#[derive(Debug)]
struct Packet {
    version: u64,
    id: u64,
    sub_packets: Vec<Packet>,
    number: Option<u64>,
}

impl Packet {
    fn new(data: &mut impl BitSource) -> Option<Self> {
        let version = data.get(3)?;
        let id = data.get(3)?;
        let mut number = None;
        let mut sub_packets = Vec::new();
        match id {
            4 => {
                number = Some(get_immediate(data)?);
            }
            _ => sub_packets = get_sub_packets(data)?,
        }
        Some(Self {
            version,
            id,
            number,
            sub_packets,
        })
    }

    fn version_sum(&self) -> u64 {
        self.sub_packets
            .iter()
            .map(Packet::version_sum)
            .sum::<u64>()
            + self.version
    }

    fn value(&self) -> u64 {
        match self.id {
            0 => self.sub_packets.iter().map(Packet::value).sum(),
            1 => self.sub_packets.iter().map(Packet::value).product(),
            2 => self.sub_packets.iter().map(Packet::value).min().unwrap(),
            3 => self.sub_packets.iter().map(Packet::value).max().unwrap(),
            4 => self.number.unwrap(),
            5 => (self.sub_packets[0].value() > self.sub_packets[1].value()) as u64,
            6 => (self.sub_packets[0].value() < self.sub_packets[1].value()) as u64,
            7 => (self.sub_packets[0].value() == self.sub_packets[1].value()) as u64,
            i @ _ => panic!("Invalid id: {}", i),
        }
    }
}

fn get_immediate(data: &mut impl BitSource) -> Option<u64> {
    let mut res = 0;
    loop {
        let cont = data.next_bit()?;
        let val = data.get(4)?;
        res = (res << 4) + val;
        if !cont {
            return Some(res);
        }
    }
}

fn get_sub_packets(data: &mut impl BitSource) -> Option<Vec<Packet>> {
    let length_type = data.next_bit()?;
    let mut res = Vec::new();
    if length_type {
        let count = data.get(11)?;
        for _ in 0..count {
            res.push(Packet::new(data)?);
        }
    } else {
        let length = data.get(15)? as usize;
        let mut new_source = LimitedSource::new(length, data);
        while let Some(p) = Packet::new(&mut new_source) {
            res.push(p);
        }
    }
    Some(res)
}

fn main() -> Result<(), Error> {
    let input = BufReader::new(get_input(16)?);
    let mut bits = NibbleSource::new(input.lines().next().unwrap().unwrap().chars());
    let packet = Packet::new(&mut bits).unwrap();
    println!("Answer 1: {}", packet.version_sum());
    println!("Answer 2: {}", packet.value());

    Ok(())
}

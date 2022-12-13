use std::{cmp::Ordering, fmt::Display, iter::Peekable, str::Chars, time::SystemTime};

const DATA: &'static str = include_str!("../data.txt");

#[derive(Debug, Clone)]
enum Packet {
    Integer(i32),
    List(Vec<Packet>),
}

impl Packet {
    fn to_string(&self) -> String {
        format!("{self}")
    }
}

impl Display for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Integer(value) => write!(f, "{value}"),
            Self::List(list) => {
                write!(f, "[")?;
                let mut commas = list.len();

                for packet in list {
                    write!(f, "{packet}")?;
                    commas -= 1;
                    if commas != 0 {
                        write!(f, ",")?;
                    }
                }

                write!(f, "]")
            }
        }
    }
}

impl Eq for Packet {}

impl PartialEq<Packet> for Packet {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Integer(l0), Self::Integer(r0)) => l0 == r0,
            (Self::List(l0), Self::List(r0)) => {
                l0.len() == r0.len() && l0.iter().zip(r0.iter()).all(|(l, r)| l == r)
            }
            _ => false,
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Integer(a), Packet::Integer(b)) => a.cmp(b),
            (int @ Packet::Integer(_), list @ Packet::List(_)) => {
                Packet::List(vec![int.clone()]).cmp(list)
            }
            (list @ Packet::List(_), int @ Packet::Integer(_)) => {
                list.cmp(&Packet::List(vec![int.clone()]))
            }
            (Packet::List(a), Packet::List(b)) => {
                for i in 0..(a.len().max(b.len())) {
                    match (a.get(i), b.get(i)) {
                        (None, Some(_)) | (Some(_), None) => return a.len().cmp(&b.len()),
                        (Some(a), Some(b)) => match a.cmp(b) {
                            Ordering::Equal => continue,
                            result => return result,
                        },
                        (None, None) => unreachable!(),
                    }
                }

                Ordering::Equal
            }
        }
    }
}

impl From<&'static str> for Packet {
    fn from(line: &'static str) -> Self {
        let chars = line.chars();

        Self::from(&mut chars.into_iter().peekable())
    }
}

impl From<&mut Peekable<Chars<'static>>> for Packet {
    fn from(chars: &mut Peekable<Chars<'static>>) -> Self {
        while let Some(ch) = chars.next() {
            match ch {
                '[' => {
                    let mut list = Vec::new();
                    while let Some(next) = chars.peek() {
                        match next {
                            ']' => {
                                chars.next();
                                return Self::List(list);
                            }
                            ',' => {
                                chars.next();
                                list.push(Self::from(chars.into_iter()))
                            }
                            '0'..='9' => {
                                let mut acc = next.to_digit(10).unwrap();
                                chars.next();

                                while let Some(nexter @ '0'..='9') = chars.peek() {
                                    acc *= 10;
                                    acc += nexter.to_digit(10).unwrap();
                                    chars.next();
                                }
                                list.push(Self::Integer(acc as i32));
                            }
                            '[' => list.push(Self::from(chars.into_iter())),
                            _ => unreachable!(),
                        }
                    }
                }
                '0'..='9' => return Self::Integer(ch.to_digit(10).unwrap() as i32),
                _ => unreachable!(),
            }
        }

        unreachable!()
    }
}

fn part_one() -> i32 {
    let mut lines = DATA.lines();

    let mut result = 0;
    let mut index = 1;
    while let (Some(a), Some(b), Some("") | None) = (lines.next(), lines.next(), lines.next()) {
        if Packet::from(a) < Packet::from(b) {
            result += index;
        }
        index += 1
    }

    result
}

fn part_two() -> i32 {
    let mut packets: Vec<_> = DATA
        .lines()
        .flat_map(|line| {
            if line == "" {
                None
            } else {
                Some(Packet::from(line))
            }
        })
        .collect();

    packets.push(Packet::from("[[6]]"));
    packets.push(Packet::from("[[2]]"));

    packets.sort();

    let mut result = 0;

    for (index, packet) in packets.into_iter().enumerate() {
        let string = packet.to_string();
        if string == "[[2]]" || string == "[[6]]" {
            if result == 0 {
                result = index as i32 + 1
            } else {
                return result * (index as i32 + 1);
            }
        }
    }

    unreachable!()
}

fn time_it<F, T>(fun: F) -> T
where
    F: Fn() -> T,
{
    let start = SystemTime::now();
    let result = fun();
    println!("Time elapsed: {} µs", start.elapsed().unwrap().as_micros());
    result
}

fn main() {
    time_it(|| println!("part 1: {}", part_one()));
    time_it(|| println!("part 2: {}", part_two()));
}

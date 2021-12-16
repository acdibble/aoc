use std::env;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

enum LengthMode {
    Bits,
    Packets,
}

impl LengthMode {
    fn from_u8(value: u8) -> Self {
        match value {
            0 => Self::Bits,
            1 => Self::Packets,
            _ => unreachable!(),
        }
    }

    fn bits_to_parse(&self) -> usize {
        match self {
            Self::Bits => 15,
            Self::Packets => 11,
        }
    }
}

#[derive(Debug)]
enum Value {
    Literal(i64),
    Packets(Vec<Packet>),
}

#[derive(Debug)]
struct Packet {
    version: u8,
    type_id: u8,
    contents: Value,
}

impl Packet {
    fn sum_versions(&self) -> i64 {
        let mut result = self.version as i64;

        match &self.contents {
            Value::Packets(packets) => {
                for packet in packets {
                    result += packet.sum_versions();
                }
            }
            _ => (),
        }

        result
    }

    fn iter(&self) -> impl Iterator<Item = &Packet> {
        match &self.contents {
            Value::Packets(packets) => packets.iter(),
            _ => unreachable!(),
        }
    }

    fn operands(&self) -> (&Packet, &Packet) {
        let mut iter = self.iter();

        (iter.next().unwrap(), iter.next().unwrap())
    }

    fn evaluate(&self) -> i64 {
        match self.type_id {
            0 => self.iter().fold(0, |acc, packet| acc + packet.evaluate()),
            1 => self.iter().fold(1, |acc, packet| acc * packet.evaluate()),
            2 => self
                .iter()
                .fold(i64::MAX, |acc, packet| acc.min(packet.evaluate())),
            3 => self
                .iter()
                .fold(i64::MIN, |acc, packet| acc.max(packet.evaluate())),
            4 => match &self.contents {
                Value::Literal(value) => *value,
                _ => unreachable!(),
            },
            5 => {
                let (a, b) = self.operands();
                match a.evaluate().cmp(&b.evaluate()) {
                    std::cmp::Ordering::Greater => 1,
                    _ => 0,
                }
            }
            6 => {
                let (a, b) = self.operands();
                match a.evaluate().cmp(&b.evaluate()) {
                    std::cmp::Ordering::Less => 1,
                    _ => 0,
                }
            }
            7 => {
                let (a, b) = self.operands();
                match a.evaluate().cmp(&b.evaluate()) {
                    std::cmp::Ordering::Equal => 1,
                    _ => 0,
                }
            }
            _ => unreachable!(),
        }
    }
}

fn digit_to_bits(digit: char) -> [u8; 4] {
    match digit {
        '0' => [0, 0, 0, 0],
        '1' => [0, 0, 0, 1],
        '2' => [0, 0, 1, 0],
        '3' => [0, 0, 1, 1],
        '4' => [0, 1, 0, 0],
        '5' => [0, 1, 0, 1],
        '6' => [0, 1, 1, 0],
        '7' => [0, 1, 1, 1],
        '8' => [1, 0, 0, 0],
        '9' => [1, 0, 0, 1],
        'A' => [1, 0, 1, 0],
        'B' => [1, 0, 1, 1],
        'C' => [1, 1, 0, 0],
        'D' => [1, 1, 0, 1],
        'E' => [1, 1, 1, 0],
        'F' => [1, 1, 1, 1],
        _ => unreachable!(),
    }
}

struct Parser<'a> {
    bits: std::iter::FlatMap<std::str::Chars<'a>, [u8; 4], fn(char) -> [u8; 4]>,
    bits_parsed: usize,
}

impl<'a> Parser<'a> {
    fn new(string: &'a str) -> Self {
        Self {
            bits: string.chars().flat_map(digit_to_bits),
            bits_parsed: 0,
        }
    }

    #[inline(always)]
    fn take_next(&mut self) -> u8 {
        match self.bits.next() {
            Some(value) => {
                self.bits_parsed += 1;
                value
            }
            _ => unreachable!("no bits left"),
        }
    }

    #[inline(always)]
    fn take(&mut self, amount: usize) -> i64 {
        let mut output = 0;

        for _ in 0..amount {
            output = (output << 1) | self.take_next() as i64;
        }

        output
    }

    #[inline(always)]
    fn parse_literal_value(&mut self) -> Value {
        let mut value = 0;

        loop {
            let go = self.take_next();

            value = (value << 4) | self.take(4);

            if go != 1 {
                break;
            }
        }

        Value::Literal(value)
    }

    #[inline(always)]
    fn parse_subpackets(&mut self) -> Value {
        let mode = LengthMode::from_u8(self.take_next());

        let mut length = self.take(mode.bits_to_parse()) as usize;

        let mut before_loop_len = self.bits_parsed;
        let mut packets = vec![];

        while length != 0 {
            packets.push(self.parse());

            match mode {
                LengthMode::Bits => {
                    length -= self.bits_parsed - before_loop_len;
                    before_loop_len = self.bits_parsed;
                }
                LengthMode::Packets => length -= 1,
            }
        }

        Value::Packets(packets)
    }

    fn parse(&mut self) -> Packet {
        let version = self.take(3) as u8;
        let type_id = self.take(3) as u8;

        let contents: Value = match type_id {
            4 => self.parse_literal_value(),
            _ => self.parse_subpackets(),
        };

        Packet {
            version,
            type_id,
            contents: contents,
        }
    }
}

fn part_one(input: &str) -> i64 {
    Parser::new(input).parse().sum_versions()
}

fn part_two(input: &str) -> i64 {
    Parser::new(input).parse().evaluate()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        println!("{:?}", Parser::new("D2FE28").parse());
        println!("{:?}", Parser::new("38006F45291200").parse());
        println!("{:?}", Parser::new("EE00D40C823060").parse());

        assert_eq!(16, Parser::new("8A004A801A8002F478").parse().sum_versions());
        assert_eq!(
            12,
            Parser::new("620080001611562C8802118E34")
                .parse()
                .sum_versions()
        );
        assert_eq!(
            23,
            Parser::new("C0015000016115A2E0802F182340")
                .parse()
                .sum_versions()
        );
        assert_eq!(
            31,
            Parser::new("A0016C880162017C3686B18A3D4780")
                .parse()
                .sum_versions()
        );

        assert_eq!(3, Parser::new("C200B40A82").parse().evaluate());
        assert_eq!(54, Parser::new("04005AC33890").parse().evaluate());
        assert_eq!(7, Parser::new("880086C3E88112").parse().evaluate());
        assert_eq!(9, Parser::new("CE00C43D881120").parse().evaluate());
        assert_eq!(1, Parser::new("D8005AC2A8F0").parse().evaluate());
        assert_eq!(0, Parser::new("F600BC2D8F").parse().evaluate());
        assert_eq!(0, Parser::new("9C005AC2F8F0").parse().evaluate());
        assert_eq!(
            1,
            Parser::new("9C0141080250320F1802104A08").parse().evaluate()
        );
    }
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

fn main() -> std::io::Result<()> {
    let file_path = env::current_dir()?.join(Path::new("data.txt"));
    let input = fs::read_to_string(file_path)?;

    time_it(|| println!("part 1: {}", part_one(&input)));
    time_it(|| println!("part 2: {}", part_two(&input)));

    Ok(())
}

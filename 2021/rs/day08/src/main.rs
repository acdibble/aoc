use std::env;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

#[derive(Clone, Copy, Debug)]
struct Digit {
    value: u8,
    size: usize,
}

#[cfg(debug_assertions)]
impl Digit {
    fn is_valid(&self) -> bool {
        self.value != 0
    }
}

impl Default for Digit {
    fn default() -> Self {
        Self { value: 0, size: 0 }
    }
}

fn parse_digit(string: Option<&str>) -> Digit {
    match string {
        Some(string) => Digit {
            size: string.len(),
            value: string.chars().fold(0, |acc, ch| {
                acc | match ch {
                    'a' => 0b0000_0001,
                    'b' => 0b0000_0010,
                    'c' => 0b0000_0100,
                    'd' => 0b0000_1000,
                    'e' => 0b0001_0000,
                    'f' => 0b0010_0000,
                    'g' => 0b0100_0000,
                    _ => unreachable!(),
                }
            }),
        },
        _ => unreachable!(),
    }
}

#[derive(Debug)]
struct Entry {
    signal_patterns: [Digit; 10],
    output_values: [Digit; 4],
    digits: [Digit; 10],
}

fn parse_signal_patterns(string: &str) -> [Digit; 10] {
    let mut parts = string.split_ascii_whitespace();

    [
        parse_digit(parts.next()),
        parse_digit(parts.next()),
        parse_digit(parts.next()),
        parse_digit(parts.next()),
        parse_digit(parts.next()),
        parse_digit(parts.next()),
        parse_digit(parts.next()),
        parse_digit(parts.next()),
        parse_digit(parts.next()),
        parse_digit(parts.next()),
    ]
}

fn parse_output_values(string: &str) -> [Digit; 4] {
    let mut parts = string.split_ascii_whitespace();

    [
        parse_digit(parts.next()),
        parse_digit(parts.next()),
        parse_digit(parts.next()),
        parse_digit(parts.next()),
    ]
}

impl Entry {
    fn new(line: &str) -> Self {
        let mut split = line.split(" | ");

        Entry {
            signal_patterns: parse_signal_patterns(split.next().unwrap()),
            output_values: parse_output_values(split.next().unwrap()),
            digits: [Default::default(); 10],
        }
    }

    fn find_digit(&self, size: usize, f: impl Fn(&Digit) -> bool) -> usize {
        for (index, digit) in self.signal_patterns.iter().enumerate() {
            if digit.size == size && f(digit) {
                return index;
            }
        }

        unreachable!()
    }

    fn find_unique_length(&mut self, len: usize) -> Digit {
        let index = self.find_digit(len, |_| true);

        std::mem::take(&mut self.signal_patterns[index])
    }

    fn assign_simple_digits(&mut self) {
        self.digits[1] = self.find_unique_length(2);
        #[cfg(debug_assertions)]
        assert!(self.digits[1].is_valid());
        self.digits[4] = self.find_unique_length(4);
        #[cfg(debug_assertions)]
        assert!(self.digits[4].is_valid());
        self.digits[7] = self.find_unique_length(3);
        #[cfg(debug_assertions)]
        assert!(self.digits[7].is_valid());
        self.digits[8] = self.find_unique_length(7);
        #[cfg(debug_assertions)]
        assert!(self.digits[8].is_valid());
    }

    fn count_matches(&self) -> usize {
        let mut total = 0;

        for digit in &self.digits {
            for output in self.output_values {
                if digit.value == output.value {
                    total += 1;
                }
            }
        }

        total
    }

    fn decipher_digits(&mut self) {
        // three has 5 digits and contains all of one
        let one = &self.digits[1];
        let index = self.find_digit(5, |digit| (one.value & digit.value) == one.value);
        self.digits[3] = std::mem::take(&mut self.signal_patterns[index]);

        // nine has 6 digits and contains all of three
        let three = &self.digits[3];
        let index = self.find_digit(6, |digit| (three.value & digit.value) == three.value);
        self.digits[9] = std::mem::take(&mut self.signal_patterns[index]);

        // zero has 6 digits and contains all of one
        let one = &self.digits[1];
        let index = self.find_digit(6, |digit| (one.value & digit.value) == one.value);
        self.digits[0] = std::mem::take(&mut self.signal_patterns[index]);

        // six is the only remaining digit with 6 digits
        self.digits[6] = self.find_unique_length(6);

        // six contains all of five
        let six = &self.digits[6];
        let index = self.find_digit(5, |digit| (six.value & digit.value) == digit.value);
        self.digits[5] = std::mem::take(&mut self.signal_patterns[index]);

        // two is the last digit
        let index = self.find_digit(5, |_| true);
        self.digits[2] = std::mem::take(&mut self.signal_patterns[index]);

        #[cfg(debug_assertions)]
        println!("{:?}", self);
    }

    fn decode(&self) -> usize {
        self.output_values.iter().fold(0, |acc, digit| {
            acc * 10
                + match self
                    .digits
                    .iter()
                    .position(|option| option.value == digit.value)
                {
                    Some(position) => position,
                    _ => unreachable!(),
                }
        })
    }
}

fn solve(input: &str) -> (usize, usize) {
    let mut part_one = 0;
    let mut part_two = 0;

    for mut entry in input.lines().map(Entry::new) {
        entry.assign_simple_digits();
        #[cfg(debug_assertions)]
        println!("{:?}", entry);
        let matches = entry.count_matches();
        #[cfg(debug_assertions)]
        println!("{}", matches);
        part_one += matches;

        entry.decipher_digits();

        part_two += entry.decode();
    }

    (part_one, part_two)
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

    time_it(|| println!("parts (1, 2): {:?}", solve(&input)));

    Ok(())
}

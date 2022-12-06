use std::time::SystemTime;

const DATA: &'static str = include_str!("../data.txt");

struct Buffer<const N: usize> {
    chars: [char; N],
    index: usize,
    count: usize,
}

impl<const N: usize> Buffer<N> {
    fn new() -> Self {
        Self {
            chars: ['\0'; N],
            index: 0,
            count: 0,
        }
    }

    fn add(&mut self, ch: char) {
        let index = self.index;
        self.chars[index] = ch;
        self.index = (self.index + 1) % N;
        self.count += 1;
    }

    fn check(&self) -> Option<usize> {
        for (index, ch) in self.chars.iter().enumerate() {
            for other in self.chars.iter().skip(index + 1) {
                if other == ch {
                    return None;
                }
            }
        }

        Some(self.count)
    }
}

fn find_unique_sequence<const N: usize>() -> usize {
    let mut chars = DATA.chars();

    let mut buffer = Buffer::<N>::new();

    for _ in 0..N {
        buffer.add(chars.next().unwrap());
    }

    for ch in chars {
        buffer.add(ch);
        if let Some(result) = buffer.check() {
            return result;
        }
    }

    unreachable!()
}

fn part_one() -> usize {
    find_unique_sequence::<4>()
}

fn part_two() -> usize {
    find_unique_sequence::<14>()
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

use std::time::SystemTime;

const DATA: &'static str = include_str!("../data.txt");

#[derive(Debug)]
struct PasswordIterator<const N: usize> {
    end: [i32; N],
    current: [i32; N],
    done: bool,
}

impl<const N: usize> PasswordIterator<N> {
    fn from(string: &str) -> Self {
        let mut split = string.trim().split('-');

        fn num_to_array<const N: usize>(number: &str) -> [i32; N] {
            let mut output = [0i32; N];

            for (index, ch) in number.chars().take(N - 1).enumerate() {
                output[index] = match ch {
                    '0' => 0,
                    '1' => 1,
                    '2' => 2,
                    '3' => 3,
                    '4' => 4,
                    '5' => 5,
                    '6' => 6,
                    '7' => 7,
                    '8' => 8,
                    '9' => 9,
                    _ => unreachable!(),
                }
            }

            output
        }

        let mut current = num_to_array(split.next().unwrap());
        let mut end = num_to_array(split.next().unwrap());

        fn normalize_password<const N: usize>(password: &mut [i32; N]) {
            let mut initial = None;

            for index in 0..N {
                if let Some(value) = initial {
                    password[index] = value;
                } else if let Some(&next) = password.get(index + 1) {
                    if next < password[index] {
                        initial = Some(password[index])
                    }
                }
            }
        }

        normalize_password(&mut current);
        current[N - 1] -= 1;
        normalize_password(&mut end);

        Self {
            end,
            current,
            done: false,
        }
    }

    fn increment(&mut self, index: usize) -> i32 {
        let value = &mut self.current[index];
        *value += 1;
        if *value < 10 {
            return *value;
        }

        if index == 0 {
            unreachable!()
        }

        let new_value = self.increment(index - 1);
        self.current[index] = new_value;
        return new_value;
    }
}

impl<const N: usize> Iterator for PasswordIterator<N> {
    type Item = [i32; N];

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        self.increment(N - 1);

        if self.current == self.end {
            self.done = true;
            return None;
        }

        Some(self.current)
    }
}

fn part_one() -> i32 {
    let mut count = 0;

    for [one, two, three, four, five, six] in PasswordIterator::<6>::from(DATA) {
        if one > two || two > three || three > four || four > five || five > six {
            continue;
        }

        if one == two || two == three || three == four || four == five || five == six {
            count += 1;
        }
    }

    count
}

fn part_two() -> i32 {
    let mut count = 0;

    'outer: for digits in PasswordIterator::<6>::from(DATA) {
        let mut it = digits.windows(2);

        let mut counts = [0i32; 10];

        counts[digits[0] as usize] += 1;

        while let Some([left, right]) = it.next() {
            if left > right {
                continue 'outer;
            }

            counts[*right as usize] += 1;
        }

        for c in counts {
            if c == 2 {
                count += 1;
                break;
            }
        }
    }

    count
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

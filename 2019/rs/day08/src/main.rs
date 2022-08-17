use std::time::SystemTime;

const DATA: &'static str = include_str!("../data.txt");

const WIDTH: usize = 25;
const HEIGHT: usize = 6;

fn part_one() -> i32 {
    let trimmed = DATA.trim();

    let mut layer_start = 0;
    let mut output = 0;
    let mut fewest_zeros = usize::MAX;

    while layer_start < trimmed.len() {
        let lines = (0..HEIGHT).map(|line_number| {
            let line_start = layer_start + line_number * WIDTH;
            let line_end = line_start + WIDTH;
            &trimmed[line_start..line_end]
        });

        let mut zeros = 0;
        let mut ones = 0;
        let mut twos = 0;

        for line in lines {
            for ch in line.chars() {
                match ch {
                    '0' => zeros += 1,
                    '1' => ones += 1,
                    '2' => twos += 1,
                    _ => {}
                }
            }
        }

        if zeros < fewest_zeros {
            output = ones * twos;
            fewest_zeros = zeros;
        }

        layer_start += WIDTH * HEIGHT;
    }

    output
}

fn part_two() {
    let trimmed = DATA.trim();

    let mut layer_start = 0;

    let mut image = [['2'; WIDTH]; HEIGHT];

    while layer_start < trimmed.len() {
        let lines = (0..HEIGHT).map(|line_number| {
            let line_start = layer_start + line_number * WIDTH;
            let line_end = line_start + WIDTH;
            &trimmed[line_start..line_end]
        });

        for (y, line) in lines.enumerate() {
            for (x, ch) in line.chars().enumerate() {
                let current = &mut image[y][x];

                if *current == '2' {
                    *current = ch;
                }
            }
        }

        layer_start += WIDTH * HEIGHT;
    }

    for line in image {
        for ch in line {
            match ch {
                '1' => print!("#"),
                '0' => print!(" "),
                _ => unreachable!(),
            }
        }

        println!("");
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

fn main() {
    time_it(|| println!("part 1: {}", part_one()));
    time_it(|| println!("part 2: {:?}", part_two()));
}

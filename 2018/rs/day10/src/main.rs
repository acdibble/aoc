use std::env;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

#[derive(Debug, Default)]
struct Dot {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
}

impl Dot {
    fn new() -> Self {
        Default::default()
    }
}

fn get_max(dots: &Vec<Dot>) -> (i32, i32) {
    dots.iter().fold((0, 0), |(x, y), curr| {
        (x.max(curr.x.abs()), y.max(curr.y.abs()))
    })
}

fn parse_file(input: &str) -> Vec<Dot> {
    input
        .lines()
        .map(|line| {
            let mut dot = Dot::new();

            dot.x = line[10..16].trim().parse().unwrap();
            dot.y = line[18..24].trim().parse().unwrap();
            dot.dx = line[36..38].trim().parse().unwrap();
            dot.dy = line[40..42].trim().parse().unwrap();

            dot
        })
        .collect()
}

fn part_one(input: &str) {
    let mut dots = parse_file(input);

    let (mut largest_x, mut largest_y) = get_max(&dots);

    loop {
        let mut new_largest_x = 0;
        let mut new_largest_y = 0;

        for dot in &mut dots {
            dot.x += dot.dx;
            dot.y += dot.dy;
            new_largest_x = new_largest_x.max(dot.x.abs());
            new_largest_y = new_largest_y.max(dot.y.abs());
        }

        if new_largest_x <= largest_x && new_largest_y <= largest_y {
            largest_x = new_largest_x;
            largest_y = new_largest_y;
        } else {
            for dot in &mut dots {
                dot.x -= dot.dx;
                dot.y -= dot.dy;
            }
            break;
        }
    }

    let (smallest_x, smallest_y) = dots.iter().fold((i32::MAX, i32::MAX), |(x, y), curr| {
        (x.min(curr.x), y.min(curr.y))
    });

    let (largest_x, largest_y) = get_max(&dots);

    let mut output = Vec::<Vec<i32>>::new();

    for _ in 0..=(largest_y - smallest_y) {
        let mut next = Vec::new();
        for _ in 0..=(largest_x - smallest_x) {
            next.push(0);
        }
        output.push(next);
    }

    for dot in &mut dots {
        dot.x -= smallest_x;
        dot.y -= smallest_y;
        output[dot.y as usize][dot.x as usize] = 1;
    }

    for line in output {
        for num in line {
            if num == 0 {
                print!(" ")
            } else {
                print!("#")
            }
        }
        print!("\n")
    }
}

fn part_two(input: &str) -> i32 {
    let mut dots = parse_file(input);

    let (mut largest_x, mut largest_y) = get_max(&dots);
    let mut ticks = 0;

    loop {
        let mut new_largest_x = 0;
        let mut new_largest_y = 0;

        for dot in &mut dots {
            dot.x += dot.dx;
            dot.y += dot.dy;
            new_largest_x = new_largest_x.max(dot.x.abs());
            new_largest_y = new_largest_y.max(dot.y.abs());
        }

        if new_largest_x <= largest_x && new_largest_y <= largest_y {
            largest_x = new_largest_x;
            largest_y = new_largest_y;
        } else {
            return ticks;
        }
        ticks += 1;
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

    time_it(|| part_one(&input));
    time_it(|| println!("part 2: {}", part_two(&input)));

    Ok(())
}

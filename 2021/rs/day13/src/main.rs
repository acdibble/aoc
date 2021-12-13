use std::collections::HashSet;
use std::env;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

fn parse_input(input: &str) -> (HashSet<(usize, usize)>, Vec<(char, usize)>) {
    let mut coords: HashSet<(usize, usize)> = HashSet::new();
    let mut folds: Vec<(char, usize)> = vec![];

    let mut lines = input.lines();
    while let Some(line) = lines.next() {
        if line == "" {
            break;
        }

        let mut split = line.split(',');
        let x = split.next().unwrap().parse::<usize>().unwrap();
        let y = split.next().unwrap().parse::<usize>().unwrap();

        coords.insert((x, y));
    }

    while let Some(line) = lines.next() {
        let rule = line.split_ascii_whitespace().nth(2).unwrap();
        let mut parts = rule.split('=');
        let axis = parts.next().unwrap().chars().nth(0).unwrap();
        let location = parts.next().unwrap().parse().unwrap();
        folds.push((axis, location));
    }

    (coords, folds)
}

fn print_code(coords: HashSet<(usize, usize)>) {
    let mut buffer: Vec<Vec<char>> = vec![];

    for (x, y) in coords {
        while buffer.len() <= y {
            buffer.push(vec![]);
        }

        let row = &mut buffer[y];

        while row.len() <= x {
            row.push(' ');
        }

        row[x] = '#';
    }

    for row in buffer {
        for ch in row {
            print!("{}", ch);
        }

        println!()
    }
}

fn solve(input: &str) -> usize {
    let (mut coords, folds) = parse_input(input);

    let mut buffer_set = HashSet::new();
    let mut after_first = 0;

    for (fold, (axis, location)) in folds.into_iter().enumerate() {
        let offset = 2 * location;

        for &(mut coord) in &coords {
            let value = match axis {
                'x' => &mut coord.0,
                'y' => &mut coord.1,
                _ => unreachable!(),
            };

            if *value > location {
                *value = offset - *value;
            }

            buffer_set.insert(coord);
        }

        std::mem::swap(&mut buffer_set, &mut coords);
        buffer_set.clear();

        if fold == 0 {
            after_first = coords.len();
        }
    }

    print_code(coords);

    after_first
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

    time_it(|| println!("part 1: {}", solve(&input)));

    Ok(())
}

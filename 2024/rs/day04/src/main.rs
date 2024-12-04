use std::time::SystemTime;

const DATA: &'static str = include_str!("../data.txt");

fn parse_grid() -> Vec<Vec<char>> {
    DATA.lines().map(|line| line.chars().collect()).collect()
}

fn get_nested<T: Copy + Default>(grid: &Vec<Vec<T>>, x: i32, y: i32) -> T {
    if x < 0 || y < 0 {
        return T::default();
    }

    grid.get(y as usize)
        .map(|l| l.get(x as usize))
        .flatten()
        .copied()
        .unwrap_or_default()
}

fn part_one() -> i32 {
    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let search = ['M', 'A', 'S'];

    let grid = parse_grid();

    let mut count = 0;

    for (y, line) in grid.iter().enumerate() {
        for (x, &ch) in line.iter().enumerate() {
            if ch != 'X' {
                continue;
            }

            'dir: for (dx, dy) in directions {
                let mut x = x as i32 + dx;
                let mut y = y as i32 + dy;
                if x < 0 || y < 0 {
                    continue;
                }

                for i in 0..3 {
                    let ch = get_nested(&grid, x, y);
                    if ch != search[i] {
                        continue 'dir;
                    }

                    x += dx;
                    y += dy;
                }

                count += 1;
            }
        }
    }

    count
}

fn part_two() -> i32 {
    let grid = parse_grid();

    let mut count = 0;

    for (y, line) in grid.iter().enumerate() {
        for (x, &ch) in line.iter().enumerate() {
            if ch != 'A' {
                continue;
            }

            let a = get_nested(&grid, x as i32 - 1, y as i32 - 1);
            let b = get_nested(&grid, x as i32 + 1, y as i32 + 1);

            let c = get_nested(&grid, x as i32 - 1, y as i32 + 1);
            let d = get_nested(&grid, x as i32 + 1, y as i32 - 1);

            match ((a, b), (c, d)) {
                (('M', 'S'), ('S', 'M')) => count += 1,
                (('S', 'M'), ('M', 'S')) => count += 1,
                (('M', 'S'), ('M', 'S')) => count += 1,
                (('S', 'M'), ('S', 'M')) => count += 1,
                _ => {}
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

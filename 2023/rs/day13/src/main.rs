use std::time::SystemTime;

const DATA: &'static str = include_str!("../data.txt");

fn search_horizontal(lines: &[Vec<char>], skip: Option<usize>) -> Option<usize> {
    for i in 1..lines.len() {
        if Some(i) == skip {
            continue;
        }

        let (a, b) = lines.split_at(i);

        if a.iter().rev().zip(b).all(|(a, b)| a == b) {
            return Some(i * 100);
        }
    }

    None
}

fn search_vertical(lines: &[Vec<char>], skip: Option<usize>) -> Option<usize> {
    let line_len = lines[0].len();

    for i in 1..line_len {
        if Some(i) == skip {
            continue;
        }

        let all_lines_mirror = lines.iter().all(|line| {
            let (a, b) = line.split_at(i);

            a.iter().rev().zip(b.iter()).all(|(a, b)| a == b)
        });

        if all_lines_mirror {
            return Some(i);
        }
    }

    None
}

fn parse_patterns() -> impl Iterator<Item = Vec<Vec<char>>> {
    DATA.trim().split("\n\n").map(|p| {
        p.lines()
            .map(|line| line.chars().collect())
            .collect::<Vec<_>>()
    })
}

fn part_one() -> usize {
    parse_patterns()
        .map(|lines| {
            search_horizontal(&lines, None)
                .or(search_vertical(&lines, None))
                .unwrap()
        })
        .sum()
}

fn part_two() -> usize {
    parse_patterns()
        .map(|mut lines| {
            let expected_horizontal = search_horizontal(&lines, None).map(|v| v / 100);
            let expected_vertical = search_vertical(&lines, None);

            for y in 0..lines.len() {
                for x in 0..lines[y].len() {
                    let old = lines[y][x];
                    lines[y][x] = if old == '.' { '#' } else { '.' };

                    if let Some(result) = search_horizontal(&lines, expected_horizontal)
                        .or(search_vertical(&lines, expected_vertical))
                    {
                        return result;
                    }

                    lines[y][x] = old
                }
            }

            unreachable!()
        })
        .sum()
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

use std::env;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

fn get_next_row(input: &str) -> String {
    let mut chars = input.chars();

    let mut buffer = String::with_capacity(input.len());

    let mut left = None;
    let mut center = chars.next();
    let mut right = chars.next();

    while center.is_some() {
        let result = match (left, center, right) {
            (Some('^'), Some('^'), Some('.'))
            | (Some('^'), Some('^'), None)
            | (Some('.'), Some('^'), Some('^'))
            | (None, Some('^'), Some('^'))
            | (Some('^'), Some('.'), Some('.'))
            | (Some('^'), Some('.'), None)
            | (Some('.'), Some('.'), Some('^'))
            | (None, Some('.'), Some('^')) => '^',
            _ => '.',
        };

        buffer.push(result);

        left = center;
        center = right;
        right = chars.next();
    }

    buffer
}

#[test]
fn test_get_next_row() {
    assert_eq!(get_next_row("..^^."), ".^^^^");
    assert_eq!(get_next_row(".^^^^"), "^^..^");

    let ten_by_ten = [
        ".^^.^.^^^^",
        "^^^...^..^",
        "^.^^.^.^^.",
        "..^^...^^^",
        ".^^^^.^^.^",
        "^^..^.^^..",
        "^^^^..^^^.",
        "^..^^^^.^^",
        ".^^^..^.^^",
        "^^.^^^..^^",
    ];

    for i in 0..ten_by_ten.len() - 1 {
        assert_eq!(get_next_row(ten_by_ten[i]), ten_by_ten[i + 1])
    }
}

fn count_safe_tiles(input: &String, iterations: i32) -> i32 {
    let mut safe = 0;

    let mut current = input.clone();

    for _ in 0..iterations {
        for c in current.chars() {
            match c {
                '.' => safe += 1,
                _ => (),
            }
        }

        current = get_next_row(&current);
    }

    safe
}

#[test]
fn test_count_safe_tiles() {
    assert_eq!(count_safe_tiles(&".^^.^.^^^^".to_owned(), 10), 38);
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

    time_it(|| println!("part 1: {}", count_safe_tiles(&input, 40)));
    time_it(|| println!("part 2: {}", count_safe_tiles(&input, 400000)));

    Ok(())
}

use std::env;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

fn get_next_row(input: &Vec<char>, buffer: &mut Vec<char>) {
    let mut chars = input.iter();

    let mut left = Some(&'.');
    let mut center = chars.next();
    let mut right = chars.next();

    let mut i = 0;

    while i < input.len() {
        let result = match (left, center, right) {
            (Some('^'), Some('^'), Some('.'))
            | (Some('.'), Some('^'), Some('^'))
            | (Some('^'), Some('.'), Some('.'))
            | (Some('.'), Some('.'), Some('^')) => '^',
            _ => '.',
        };

        buffer[i] = result;
        i += 1;

        left = center;
        center = right;
        right = chars.next().or(Some(&'.'))
    }
}

#[test]
fn test_get_next_row() {
    let mut buffer = Vec::new();
    get_next_row(&"..^^.".chars().collect(), &mut buffer);
    assert_eq!(buffer, ".^^^^");
    get_next_row(&".^^^^".chars().collect(), &mut buffer);
    assert_eq!(buffer, "^^..^");

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
        get_next_row(&ten_by_ten[i].chars().collect(), &mut buffer);
        assert_eq!(buffer, ten_by_ten[i + 1])
    }
}

fn count_safe_tiles(input: &String, iterations: i32) -> i32 {
    let mut safe = 0;

    let mut current: Vec<char> = input.chars().collect();
    let mut buffer = current.clone();

    for _ in 0..iterations {
        for c in current.iter() {
            match c {
                '.' => safe += 1,
                _ => (),
            }
        }

        get_next_row(&current, &mut buffer);
        std::mem::swap(&mut current, &mut buffer);
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
    println!("Time elapsed: {} ms", start.elapsed().unwrap().as_millis());
    result
}

fn main() -> std::io::Result<()> {
    let file_path = env::current_dir()?.join(Path::new("data.txt"));
    let input = fs::read_to_string(file_path)?;

    time_it(|| println!("part 1: {}", count_safe_tiles(&input, 40)));
    time_it(|| println!("part 2: {}", count_safe_tiles(&input, 400000)));

    Ok(())
}

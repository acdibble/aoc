use std::collections::VecDeque;
use std::env;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

fn play_game(player_count: usize, marble_count: usize) -> usize {
    let mut marbles = VecDeque::new();
    marbles.push_back(0);
    let mut scores = vec![0; player_count];

    for next_marble in 1..=marble_count {
        if next_marble % 23 != 0 {
            marbles.rotate_left(1);
            marbles.rotate_left(1);
            marbles.push_front(next_marble);
        } else {
            marbles.rotate_right(7);
            scores[(next_marble % player_count)] += marbles.pop_front().unwrap() + next_marble;
        }
    }

    *scores.iter().max().unwrap()
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

    let mut parts = input.split_ascii_whitespace();

    let player_count: usize = parts.next().unwrap().parse().unwrap();
    let marble_count: usize = parts.nth(5).unwrap().parse().unwrap();

    time_it(|| println!("part 1: {}", play_game(player_count, marble_count)));
    time_it(|| println!("part 2: {}", play_game(player_count, marble_count * 100)));

    Ok(())
}

use std::env;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

#[inline(always)]
fn iterate(queue: &mut [usize; 9], times: i32) {
    for _ in 0..times {
        queue.rotate_left(1);
        let eights = queue[8];
        queue[6] += eights;
    }
}

fn simulate(input: &str) -> (usize, usize) {
    let mut queue = [0usize; 9];

    for n in input.split(',') {
        if let Ok(num) = n.parse::<usize>() {
            if let Some(slot) = queue.get_mut(num) {
                *slot += 1;
                continue;
            }
        }

        unreachable!()
    }

    iterate(&mut queue, 80);

    let part_one = queue.iter().sum();

    iterate(&mut queue, 256 - 80);

    (part_one, queue.iter().sum::<usize>())
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

    let answers = time_it(|| simulate(&input));
    println!("part (1, 2): {:?}", answers);

    Ok(())
}

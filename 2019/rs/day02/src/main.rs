use std::{env, fs, path::Path, time::SystemTime};

fn run(opcodes: &mut Vec<i32>) {
    let mut pc = 0;

    macro_rules! read_int {
        () => {{
            let code = opcodes[pc];
            pc += 1;
            code
        }};
    }

    macro_rules! read_arg {
        () => {
            opcodes[read_int!() as usize]
        };
    }

    loop {
        match read_int!() {
            1 => {
                let a = read_arg!();
                let b = read_arg!();
                let dest = read_int!() as usize;

                opcodes[dest] = a + b;
            }
            2 => {
                let a = read_arg!();
                let b = read_arg!();
                let dest = read_int!() as usize;

                opcodes[dest] = a * b;
            }
            99 => break,
            instruction => unreachable!("received unexpected opcode {instruction} at {}", pc - 1),
        }
    }
}

fn part_one(mut input: Vec<i32>) -> i32 {
    input[1] = 12;
    input[2] = 2;

    run(&mut input);

    input[0]
}

fn part_two(input: Vec<i32>) -> i32 {
    let mut buffer = input.clone();

    for a in 0..=99 {
        for b in 0..=99 {
            for (index, dest) in buffer.iter_mut().enumerate() {
                *dest = input[index];
            }

            buffer[1] = a;
            buffer[2] = b;
            run(&mut buffer);

            if buffer[0] == 19690720 {
                return 100 * a + b;
            }
        }
    }

    unreachable!()
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
    let opcodes: Vec<i32> = fs::read_to_string(file_path)?
        .trim_end()
        .split(',')
        .flat_map(|x| x.parse())
        .collect();

    time_it(|| println!("part 1: {}", part_one(opcodes.clone())));
    time_it(|| println!("part 1: {}", part_two(opcodes.clone())));

    Ok(())
}

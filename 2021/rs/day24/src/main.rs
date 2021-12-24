use std::{env, fs, path::Path, time::SystemTime};

#[derive(Default, Debug)]
struct Step {
    id: usize,
    pop: bool,
    x_mod: i32,
    w_mod: i32,
}

fn parse_into_pairs(input: &str) -> [(Step, Step); 7] {
    let mut steps: [Step; 14] = Default::default();

    let mut lines = input.lines();

    for step in 0..14 {
        let mut next_step: Step = Default::default();
        next_step.id = step + 1;
        let mut first_one_seen = false;

        while let Some(line) = lines.next() {
            let mut parts = line.split_ascii_whitespace();

            match (parts.next(), parts.next(), parts.next()) {
                (Some("div"), Some("z"), Some("1")) => next_step.pop = false,
                (Some("div"), Some("z"), Some("26")) => next_step.pop = true,
                (Some("add"), Some("x"), Some(amount)) if amount != "z" => {
                    next_step.x_mod = amount.parse().unwrap();
                }
                (Some("add"), Some("y"), Some("1")) if !first_one_seen => first_one_seen = true,
                (Some("add"), Some("y"), Some(amount)) if amount != "w" && amount != "25" => {
                    next_step.w_mod = amount.parse().unwrap();
                    break;
                }
                _ => (),
            }
        }

        steps[step] = next_step;
    }

    let mut pairs: [(Step, Step); 7] = Default::default();

    for step in steps {
        if step.pop {
            for pair in pairs.iter_mut().rev() {
                match pair {
                    (Step { id: 1..=14, .. }, pop @ Step { id: 0, .. }) => {
                        *pop = step;
                        break;
                    }
                    _ => (),
                }
            }
        } else {
            for pair in pairs.iter_mut() {
                match pair {
                    (push @ Step { id: 0, .. }, _) => {
                        *push = step;
                        break;
                    }
                    _ => (),
                }
            }
        }
    }

    pairs
}

enum Kind {
    Min,
    Max,
}

fn solve(input: &str, kind: Kind) -> i64 {
    let mut model_number = [0; 14];

    for (push_step, pop_step) in parse_into_pairs(input) {
        let x_mod = pop_step.x_mod;
        let w_mod = push_step.w_mod;

        let mut push_value = match kind {
            Kind::Max => 9,
            Kind::Min => 1,
        };
        let mut pop_value;

        loop {
            pop_value = push_value + x_mod + w_mod;
            if (1..=9).contains(&pop_value) {
                break;
            }
            push_value += match kind {
                Kind::Max => -1,
                Kind::Min => 1,
            };
        }

        model_number[push_step.id - 1] = push_value;
        model_number[pop_step.id - 1] = pop_value;
    }

    let mut result = 0i64;

    for value in model_number {
        result *= 10;
        result += value as i64;
    }

    result
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

    time_it(|| println!("part 1: {}", solve(&input, Kind::Max)));
    time_it(|| println!("part 2: {}", solve(&input, Kind::Min)));

    Ok(())
}

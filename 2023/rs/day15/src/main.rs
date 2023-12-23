use std::time::SystemTime;

const DATA: &'static str = include_str!("../data.txt");

fn hash(string: &str) -> i32 {
    let mut value = 0;

    for ch in string.chars() {
        value += ch as i32;
        value *= 17;
        value %= 256;
    }

    value
}

fn part_one() -> i32 {
    DATA.trim().split(",").map(hash).sum()
}

type Lens = (&'static str, i32);

fn part_two() -> i32 {
    let mut boxes: Vec<Vec<Lens>> = (0..0xff + 1).map(|_| Vec::new()).collect();

    macro_rules! find_lens {
        ($box:expr, $label:expr) => {
            $box.iter().enumerate().find_map(
                |(index, (n, _))| {
                    if *n == $label {
                        Some(index)
                    } else {
                        None
                    }
                },
            )
        };
    }

    for op in DATA.trim().split(",") {
        println!("{op}");

        if op.ends_with('-') {
            let label = &op[..op.len() - 1];
            let b = hash(label) as usize;

            if let Some(index) = find_lens!(&boxes[b], label) {
                boxes[b].remove(index);
            }
        } else {
            let label = &op[..op.len() - 2];
            let b = hash(label) as usize;
            let len = op[op.len() - 1..].parse().unwrap();

            if let Some(index) = find_lens!(&boxes[b], label) {
                boxes[b][index].1 = len;
            } else {
                boxes[b].push((label, len))
            }
        }

        for b in boxes.iter().take(8) {
            println!("{b:?}");
        }
        println!()
    }

    boxes
        .into_iter()
        .enumerate()
        .flat_map(|(box_index, b)| {
            let box_power = box_index as i32 + 1;

            b.into_iter().enumerate().map(move |(index, (_, len))| {
                let lens_power = index as i32 + 1;

                box_power * lens_power * len
            })
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

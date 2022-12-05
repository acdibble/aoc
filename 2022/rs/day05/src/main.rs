use std::time::SystemTime;

const DATA: &'static str = include_str!("../data.txt");

struct Move {
    count: i32,
    src: usize,
    dest: usize,
}

fn parse_stacks() -> ([Vec<char>; 9], impl Iterator<Item = Move>) {
    let mut stacks: [Vec<char>; 9] = [
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
    ];

    let mut lines = DATA.lines();

    while let Some(line) = lines.next() {
        if line == "" {
            break;
        }

        for (index, ch) in line.char_indices() {
            match ch {
                'A'..='Z' => stacks[(index - 1) / 4].insert(0, ch),
                _ => continue,
            }
        }
    }

    let moves = lines
        .map(|line| {
            let mut parts = line.split_ascii_whitespace();
            parts.next();
            let count: i32 = parts.next().unwrap().parse().unwrap();
            parts.next();
            let src: usize = parts.next().unwrap().parse().unwrap();
            parts.next();
            let dest: usize = parts.next().unwrap().parse().unwrap();

            Move {
                count,
                src: src - 1,
                dest: dest - 1,
            }
        })
        .into_iter();

    (stacks, moves)
}

fn get_result(stacks: [Vec<char>; 9]) -> String {
    stacks
        .map(|stack| *stack.last().unwrap())
        .into_iter()
        .collect()
}

fn part_one() -> String {
    let (mut stacks, moves) = parse_stacks();

    for Move { count, src, dest } in moves {
        for _ in 0..count {
            let ch = stacks[src].pop().unwrap();
            stacks[dest].push(ch);
        }
    }

    get_result(stacks)
}

fn part_two() -> String {
    let (mut stacks, moves) = parse_stacks();
    let mut temp_stack = Vec::new();

    for Move { count, src, dest } in moves {
        let src = &mut stacks[src];
        for _ in 0..count {
            let ch = src.pop().unwrap();
            temp_stack.push(ch);
        }

        let dest = &mut stacks[dest];
        while let Some(ch) = temp_stack.pop() {
            dest.push(ch);
        }
    }

    get_result(stacks)
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

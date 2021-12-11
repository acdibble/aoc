use std::env;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

fn find_illegal_char(line: &str, stack: &mut Vec<char>) -> Option<char> {
    for ch in line.chars() {
        match ch {
            '(' | '<' | '[' | '{' => stack.push(ch),
            ')' | '>' | ']' | '}' => match stack.pop() {
                Some(opener) => match (opener, ch) {
                    ('(', ')') => (),
                    ('{', '}') => (),
                    ('[', ']') => (),
                    ('<', '>') => (),
                    _ => return Some(ch),
                },
                None => return Some(ch),
            },
            _ => unreachable!(),
        }
    }

    None
}

fn score_stack(stack: &mut Vec<char>) -> u64 {
    let mut score = 0;

    while let Some(ch) = stack.pop() {
        score *= 5;
        score += match ch {
            '(' => 1,
            '[' => 2,
            '{' => 3,
            '<' => 4,
            _ => unreachable!(),
        }
    }

    score
}

fn solve(input: &str) -> (i32, u64) {
    let mut stack = Vec::new();

    let mut syntax_error_score = 0;
    let mut autocomplete_scores = Vec::new();
    for line in input.lines() {
        if let Some(ch) = find_illegal_char(line, &mut stack) {
            syntax_error_score += match ch {
                ')' => 3,
                ']' => 57,
                '}' => 1197,
                '>' => 25137,
                _ => unreachable!(),
            };
            stack.clear()
        } else {
            autocomplete_scores.push(score_stack(&mut stack));
        }
    }

    autocomplete_scores.sort();
    (
        syntax_error_score,
        autocomplete_scores[autocomplete_scores.len() / 2],
    )
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

    time_it(|| println!("parts (1, 2): {:?}", solve(&input)));

    Ok(())
}

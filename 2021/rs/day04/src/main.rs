use std::collections::VecDeque;
use std::env;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

#[derive(Debug)]
enum State {
    Marked,
    Unmarked,
}

#[derive(Debug)]
struct Square {
    value: i32,
    state: State,
}

impl Square {
    fn mark(&mut self) {
        self.state = State::Marked;
    }

    fn is_marked(&self) -> bool {
        matches!(self.state, State::Marked)
    }

    fn set_value(&mut self, value: i32) {
        self.value = value
    }

    fn has_value(&self, value: i32) -> bool {
        self.value == value
    }
}

impl Default for Square {
    fn default() -> Self {
        Self {
            value: 0,
            state: State::Unmarked,
        }
    }
}

#[derive(Debug, Default)]
struct Board([[Square; 5]; 5]);

impl Board {
    fn is_winner(&self) -> bool {
        self.0.iter().any(|row| row.iter().all(|sq| sq.is_marked()))
            || (0..5).any(|col| self.0.iter().all(|row| row[col].is_marked()))
    }

    fn mark(&mut self, value: i32) -> bool {
        for row in &mut self.0 {
            for sq in row {
                if sq.has_value(value) {
                    sq.mark();
                    return true;
                }
            }
        }

        false
    }

    fn sum_unmarked(&self) -> i32 {
        self.0.iter().fold(0, |row_acc, row| {
            row_acc
                + row.iter().fold(
                    0,
                    |acc, sq| if sq.is_marked() { acc } else { acc + sq.value },
                )
        })
    }

    #[cfg(debug_assertions)]
    fn print(&self) {
        println!("==================================================");
        for row in &self.0 {
            for sq in row {
                if sq.is_marked() {
                    print!("({:^2}) ", sq.value)
                } else {
                    print!("{:^4} ", sq.value)
                }
            }
            println!();
        }
        println!("==================================================");
    }
}

fn parse_boards(mut lines: std::str::Lines) -> Vec<Board> {
    let mut boards = Vec::new();

    while let Some(_) = lines.next() {
        let mut board: Board = Default::default();

        for row in 0..5 {
            for (col, num) in lines
                .next()
                .unwrap()
                .split_ascii_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                .enumerate()
            {
                board.0[row][col].set_value(num);
            }
        }

        boards.push(board)
    }

    boards
}

struct Simulator {
    numbers: Vec<i32>,
    boards: Vec<Board>,
    should_purge: bool,
    winners: VecDeque<i32>,
}

impl Simulator {
    fn new(numbers: Vec<i32>, boards: Vec<Board>) -> Self {
        Self {
            numbers,
            boards,
            should_purge: false,
            winners: VecDeque::new(),
        }
    }
}

impl Iterator for Simulator {
    type Item = i32;

    fn next(&mut self) -> std::option::Option<<Self as std::iter::Iterator>::Item> {
        if !self.winners.is_empty() {
            return self.winners.pop_front();
        }

        while let Some(num) = self.numbers.pop() {
            if self.should_purge {
                self.boards.retain(|b| !b.is_winner());
                self.should_purge = false
            }
            #[cfg(debug_assertions)]
            println!("number drawn: {}", num);
            for board in self.boards.iter_mut() {
                if board.mark(num) && board.is_winner() {
                    self.should_purge = true;
                    self.winners.push_back(board.sum_unmarked() * num)
                }

                #[cfg(debug_assertions)]
                board.print();
            }

            if !self.winners.is_empty() {
                return self.winners.pop_front();
            }
        }

        None
    }
}

fn main() -> std::io::Result<()> {
    let start = SystemTime::now();
    let file_path = env::current_dir()?.join(Path::new("data.txt"));
    let input = fs::read_to_string(file_path)?;

    let mut lines = input.lines();

    let numbers: Vec<_> = lines
        .next()
        .unwrap()
        .split(',')
        .map(|l| l.parse::<i32>().unwrap())
        .rev()
        .collect();
    let boards = parse_boards(lines);
    let mut sim = Simulator::new(numbers, boards);
    println!("Set up time: {} µs", start.elapsed().unwrap().as_micros());

    {
        let start = SystemTime::now();
        let result = sim.next();
        println!("Time elapsed: {} µs", start.elapsed().unwrap().as_micros());
        println!("part 1: {:?}", result);
    }
    {
        let start = SystemTime::now();
        let mut result = 0;
        while let Some(next) = sim.next() {
            result = next
        }
        println!("Time elapsed: {} µs", start.elapsed().unwrap().as_micros());
        println!("part 2: {}", result);
    }

    Ok(())
}

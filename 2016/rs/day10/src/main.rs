use std::collections::{HashMap, VecDeque};
use std::env;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

#[derive(Clone, Copy, Debug)]
enum Receiver {
    Bot(usize),
    Output(usize),
}

#[derive(Clone, Copy, Debug)]
enum Instruction {
    Direct(usize, i32),
    Pass(usize, Receiver, Receiver),
}

#[derive(Debug)]
struct Bot {
    high: Option<i32>,
    low: Option<i32>,
    instruction: Instruction,
}

impl Bot {
    fn new(instruction: Instruction) -> Self {
        Self {
            instruction,
            high: None,
            low: None,
        }
    }

    fn is_full(&self) -> bool {
        self.high.is_some() && self.low.is_some()
    }

    fn take(&mut self, value: i32) {
        if self.high.is_none() {
            self.high = Some(value);
        } else if self.high.unwrap() < value {
            self.low = self.high;
            self.high = Some(value);
        } else {
            self.low = Some(value);
        }
    }
}

fn part_one(input: &String) -> (usize, HashMap<usize, i32>) {
    let mut instruction_queue = VecDeque::new();
    let mut bots = HashMap::new();

    for line in input.lines() {
        let mut words = line.split_ascii_whitespace();
        match words.next().unwrap() {
            "value" => {
                let value = words.next().unwrap().parse().unwrap();

                words.next();
                words.next();
                words.next();

                let bot = words.next().unwrap().parse().unwrap();

                instruction_queue.push_back(Instruction::Direct(bot, value))
            }
            "bot" => {
                let giver = words.next().unwrap().parse().unwrap();

                words.next();
                words.next();
                words.next();

                let bot_or_output = words.next().unwrap();
                let bot_or_output_id = words.next().unwrap().parse().unwrap();

                let receiver_one = match bot_or_output {
                    "bot" => Receiver::Bot(bot_or_output_id),
                    "output" => Receiver::Output(bot_or_output_id),
                    _ => unreachable!(),
                };

                words.next();
                words.next();
                words.next();

                let bot_or_output = words.next().unwrap();
                let bot_or_output_id = words.next().unwrap().parse().unwrap();

                let receiver_two = match bot_or_output {
                    "bot" => Receiver::Bot(bot_or_output_id),
                    "output" => Receiver::Output(bot_or_output_id),
                    _ => unreachable!(),
                };

                bots.insert(
                    giver,
                    Bot::new(Instruction::Pass(giver, receiver_one, receiver_two)),
                );
            }
            _ => unreachable!(),
        }
    }

    let mut outputs = HashMap::new();

    while let Some(instruction) = instruction_queue.pop_front() {
        match instruction {
            Instruction::Direct(bot_id, value) => {
                let bot = bots.get_mut(&bot_id).unwrap();
                bot.take(value);

                if bot.is_full() {
                    instruction_queue.push_back(bot.instruction);
                }
            }
            Instruction::Pass(giver_id, receiver_one, receiver_two) => {
                let bot = bots.get(&giver_id).unwrap();

                for (receiver, value) in [
                    (receiver_one, bot.low.unwrap()),
                    (receiver_two, bot.high.unwrap()),
                ] {
                    match receiver {
                        Receiver::Bot(bot_id) => {
                            let receiver = bots.get_mut(&bot_id).unwrap();
                            receiver.take(value);

                            if receiver.is_full() {
                                instruction_queue.push_back(receiver.instruction);
                            }
                        }
                        Receiver::Output(output_id) => {
                            outputs.insert(output_id, value);
                        }
                    }
                }
            }
        }
    }

    let bot_id = *bots
        .iter()
        .find(|(_, bot)| bot.low == Some(17) && bot.high == Some(61))
        .unwrap()
        .0;

    (bot_id, outputs)
}

fn part_two(input: &HashMap<usize, i32>) -> i32 {
    input.get(&0).unwrap() * input.get(&1).unwrap() * input.get(&2).unwrap()
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

    let outputs = time_it(|| {
        let result = part_one(&input);
        println!("part 1: {}", result.0);
        result.1
    });
    time_it(|| println!("part 2: {}", part_two(&outputs)));

    Ok(())
}

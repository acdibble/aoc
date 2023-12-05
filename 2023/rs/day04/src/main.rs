use std::{
    collections::{BTreeMap, VecDeque},
    time::SystemTime,
};

const DATA: &'static str = include_str!("../data.txt");

#[derive(Debug)]
struct Card {
    id: usize,
    winners: i32,
}

impl Card {
    fn parse<const W: usize, const M: usize>(line: &str) -> Self {
        let mut it = line.split_ascii_whitespace();

        it.next(); // "Card"

        let id: usize = it.next().unwrap().trim_end_matches(':').parse().unwrap();

        let mut winning_numbers = [0; W];

        for n in &mut winning_numbers {
            *n = it.next().unwrap().parse().unwrap();
        }

        it.next(); // "|"

        let mut winners = 0;

        for _ in 0..M {
            let num = it.next().unwrap().parse().unwrap();

            if winning_numbers.contains(&num) {
                winners += 1;
            }
        }

        Self { id, winners }
    }
}

impl Card {
    fn score(&self) -> i32 {
        match self.winners {
            0 => 0,
            score => 2i32.pow(score as u32 - 1),
        }
    }
}

fn part_one() -> i32 {
    DATA.trim()
        .lines()
        .map(|l| Card::parse::<10, 25>(l).score())
        .sum()
}

fn part_two() -> i32 {
    let card_map = DATA
        .trim()
        .lines()
        .map(|l| {
            let card = Card::parse::<10, 25>(l);
            (card.id, card)
        })
        .collect::<BTreeMap<_, _>>();

    let mut cards = card_map.values().collect::<VecDeque<_>>();

    let mut count = 0;

    while let Some(card) = cards.pop_front() {
        count += 1;
        for offset in 1..=card.winners {
            cards.push_back(card_map.get(&(card.id + offset as usize)).unwrap())
        }
    }

    count
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

use std::{cmp::Ordering, fmt::Write, time::SystemTime};

const DATA: &'static str = include_str!("../data.txt");

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::Ten,
            'J' => Card::Jack,
            'Q' => Card::Queen,
            'K' => Card::King,
            'A' => Card::Ace,
            _ => unreachable!("Invalid card value: '{value}'"),
        }
    }
}

impl std::fmt::Debug for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ch = match self {
            Card::Joker => 'Ɉ',
            Card::Two => '2',
            Card::Three => '3',
            Card::Four => '4',
            Card::Five => '5',
            Card::Six => '6',
            Card::Seven => '7',
            Card::Eight => '8',
            Card::Nine => '9',
            Card::Ten => 'T',
            Card::Jack => 'J',
            Card::Queen => 'Q',
            Card::King => 'K',
            Card::Ace => 'A',
        };

        f.write_char(ch)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Rank {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl From<&[Card; 5]> for Rank {
    fn from(cards: &[Card; 5]) -> Self {
        let mut counts = [None; 5];

        let mut joker_count = 0;

        for card in cards {
            if *card == Card::Joker {
                joker_count += 1;
                continue;
            }

            for count in &mut counts {
                match count {
                    Some((c, count)) if *c == card => *count += 1,
                    Some(_) => continue,
                    None => *count = Some((card, 1)),
                }

                break;
            }
        }

        counts.sort_by(|a, b| {
            match (a, b) {
                (Some(a), Some(b)) => a.1.cmp(&b.1),
                (None, Some(_)) => Ordering::Less,
                (Some(_), None) => Ordering::Greater,
                (None, None) => Ordering::Equal,
            }
            .reverse()
        });

        counts[0].as_mut().map(|c| c.1 += joker_count);

        match counts[0] {
            Some((_, 5)) | None => Self::FiveOfAKind,
            Some((_, 4)) => Self::FourOfAKind,
            Some((_, 3)) if matches!(counts[1], Some((_, 2))) => Self::FullHouse,
            Some((_, 3)) => Self::ThreeOfAKind,
            Some((_, 2)) if matches!(counts[1], Some((_, 2))) => Self::TwoPair,
            Some((_, 2)) => Self::OnePair,
            Some((_, 1)) => Self::HighCard,
            _ => unreachable!("cards: {cards:?}"),
        }
    }
}

#[derive(Debug)]
struct Hand {
    cards: [Card; 5],
    bid: usize,
    rank: Rank,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.rank.cmp(&other.rank) {
            Ordering::Equal => {
                for (a, b) in self.cards.iter().zip(&other.cards) {
                    match a.cmp(b) {
                        Ordering::Equal => {}
                        ordering => return ordering,
                    }
                }

                unreachable!()
            }
            ordering => ordering,
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.rank == other.rank && self.cards.iter().zip(&other.cards).all(|(a, b)| a == b)
    }
}

impl Eq for Hand {}

impl From<&str> for Hand {
    fn from(line: &str) -> Self {
        let mut cards = [Card::Joker; 5];

        let mut it = line.split_ascii_whitespace();

        for (index, ch) in it.next().unwrap().char_indices() {
            cards[index] = ch.into();
        }

        let rank = Rank::from(&cards);

        let bid = it.next().unwrap().parse().unwrap();

        Hand { cards, bid, rank }
    }
}

fn part_one() -> usize {
    let mut cards: Vec<_> = DATA.trim().lines().map(Hand::from).collect();

    cards.sort();

    cards
        .into_iter()
        .enumerate()
        .map(|(index, card)| card.bid * (index + 1))
        .sum()
}

fn part_two() -> usize {
    let mut cards: Vec<_> = DATA
        .trim()
        .lines()
        .map(|line| {
            let mut hand = Hand::from(line);
            for card in &mut hand.cards {
                if *card == Card::Jack {
                    *card = Card::Joker;
                }
            }
            hand
        })
        .collect();

    cards.sort();

    cards
        .into_iter()
        .enumerate()
        .map(|(index, card)| card.bid * (index + 1))
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

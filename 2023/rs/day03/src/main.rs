use std::{collections::BTreeMap, iter, time::SystemTime};
use utils::Point;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Span {
    start: Point,
    end: Point,
    circumference_points: Vec<Point>,
}

impl Span {
    fn circumambulate<'a>(&'a mut self) -> impl Iterator<Item = Point> + 'a {
        if self.circumference_points.is_empty() {
            let start_x = self.start.x - 1;
            let end_x = self.end.x + 1;

            let start_y = self.start.y - 1;
            let end_y = self.end.y + 1;

            self.circumference_points = (start_x..=end_x)
                .into_iter()
                .map(move |x| Point::from((x, start_y)))
                .chain(
                    (start_x..=end_x)
                        .into_iter()
                        .map(move |x| Point::from((x, end_y))),
                )
                .chain(iter::once(Point::from((start_x, self.start.y))))
                .chain(iter::once(Point::from((end_x, self.start.y))))
                .collect()
        }

        self.circumference_points.iter().copied()
    }
}

const DATA: &'static str = include_str!("../data.txt");

fn parse_buffer(chars: &mut Vec<char>) -> i32 {
    assert!(chars.len() > 0);
    let mut init = chars[0].to_digit(10).unwrap() as i32;

    if chars.len() == 1 {
        chars.clear();
        return init;
    }

    for ch in chars.iter().skip(1) {
        init *= 10;
        init += ch.to_digit(10).unwrap() as i32;
    }

    chars.clear();

    init
}

fn parse_schematic() -> (BTreeMap<Span, i32>, BTreeMap<Point, char>) {
    let mut spans = BTreeMap::<Span, i32>::new();
    let mut symbols = BTreeMap::<Point, char>::new();

    let mut span_start = None;
    let mut buffer = Vec::new();
    for (y, line) in DATA.trim().lines().enumerate() {
        for (x, ch) in line.char_indices() {
            if ch.is_ascii_digit() && span_start.is_none() {
                span_start = Some(Point::from((x, y)));
                buffer.push(ch);
                continue;
            }

            if ch.is_ascii_digit() {
                buffer.push(ch);
                continue;
            }

            if let Some(start) = span_start.take() {
                let span = Span {
                    start,
                    end: Point::from((x - 1, y)),
                    circumference_points: Vec::new(),
                };

                spans.insert(span, parse_buffer(&mut buffer));
            }

            if ch != '.' {
                symbols.insert(Point::from((x, y)), ch);
            }
        }

        if let Some(start) = span_start.take() {
            let span = Span {
                start,
                end: Point::from((line.len() - 1, y)),
                circumference_points: Vec::new(),
            };

            spans.insert(span, parse_buffer(&mut buffer));
        }
    }

    (spans, symbols)
}

fn part_one() -> i32 {
    let mut sum = 0;
    let (spans, symbols) = parse_schematic();

    for (mut span, value) in spans {
        let has_symbol = span
            .circumambulate()
            .any(|point| symbols.contains_key(&point));

        if has_symbol {
            sum += value;
        }
    }

    sum
}

fn part_two() -> i32 {
    let mut sum = 0;
    let (spans, symbols) = parse_schematic();
    let mut spans = spans.into_iter().collect::<Vec<_>>();

    'outer: for (symbol, ch) in symbols {
        if ch != '*' {
            continue;
        }

        let mut a = None;
        let mut b = None;

        for (span, value) in spans.iter_mut() {
            if span.circumambulate().any(|point| point == symbol) {
                if a.is_none() {
                    a = Some(*value);
                } else if b.is_none() {
                    b = Some(*value);
                } else {
                    continue 'outer;
                }
            }
        }

        sum += a.unwrap_or_default() * b.unwrap_or_default()
    }

    sum
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

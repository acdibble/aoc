use std::{collections::HashMap, time::SystemTime};
use utils::Graph;

const DATA: &'static str = include_str!("../data.txt");

fn parse_line(line: &str) -> (&str, i32, Vec<&str>) {
    let mut parts = line.split_ascii_whitespace();
    parts.next();
    let tunnel = parts.next().unwrap();
    parts.next();
    parts.next();
    let rate = parts.next().unwrap().split('=').last().unwrap();
    let rate = rate[0..rate.len() - 1].parse().unwrap();
    let mut tunnels = vec![];
    let mut rev = parts.rev();

    tunnels.push(rev.next().unwrap());

    loop {
        let dest = rev.next().unwrap();

        if !dest.ends_with(",") {
            break;
        }

        tunnels.push(&dest[0..dest.len() - 1]);
    }

    (tunnel, rate, tunnels)
}

#[derive(Debug)]
struct State<'a> {
    location: &'a str,
    time_remaining: i32,
    rate: i32,
    visited: i32,
    total: i32,
}

fn calculate_flows(time: i32) -> HashMap<i32, i32> {
    let mut graph = Graph::<&str>::new();
    let mut rates_map = HashMap::new();

    for line in DATA.lines() {
        let (tunnel, rate, tunnels) = parse_line(line);
        rates_map.insert(tunnel, rate);

        for dest in tunnels {
            graph.add_edge(tunnel, dest);
        }
    }

    let valves: Vec<_> = rates_map
        .iter()
        .filter_map(|(key, value)| if *value == 0 { None } else { Some(*key) })
        .collect();

    let state_map: HashMap<_, _> = valves
        .iter()
        .enumerate()
        .map(|(index, name)| (*name, 1 << index))
        .collect();

    let mut queue = Vec::from([State {
        location: "AA",
        time_remaining: time,
        rate: 0,
        visited: 0,
        total: 0,
    }]);
    let mut flows = HashMap::new();
    let all_visited = state_map.values().fold(0, |a, b| a | b);

    while let Some(State {
        location,
        time_remaining,
        rate,
        visited,
        total,
    }) = queue.pop()
    {
        if visited == all_visited {
            let entry = flows.entry(visited).or_default();
            let new_total = total + rate * time_remaining;
            if *entry < new_total {
                *entry = new_total;
            }
            continue;
        }

        for next in &valves {
            if (visited & state_map[next]) != 0 {
                continue;
            }

            let time_elapsed = graph.distance_between(location, next) + 1;
            let entry = flows.entry(visited).or_default();
            let new_total = total + rate * time_remaining;
            if *entry < new_total {
                *entry = new_total;
            }

            if time_elapsed >= time_remaining {
                continue;
            }

            queue.push(State {
                location: next,
                time_remaining: time_remaining - time_elapsed,
                rate: rate + rates_map[next],
                visited: visited | state_map[next],
                total: total + rate * time_elapsed,
            })
        }
    }

    flows
}

fn part_one() -> i32 {
    *calculate_flows(30).values().max().unwrap()
}

fn part_two() -> i32 {
    let flows = calculate_flows(26);

    let mut result = 0;

    for (index, (a, amount_a)) in flows.iter().enumerate() {
        for (b, amount_b) in flows.iter().skip(index + 1) {
            if a & b == 0 {
                result = result.max(amount_a + amount_b);
            }
        }
    }

    result
}

fn time_it<F, T>(fun: F) -> T
where
    F: Fn() -> T,
{
    let start = SystemTime::now();
    let result = fun();
    println!("Time elapsed: {} ms", start.elapsed().unwrap().as_millis());
    result
}

fn main() {
    time_it(|| println!("part 1: {}", part_one()));
    time_it(|| println!("part 2: {}", part_two()));
}

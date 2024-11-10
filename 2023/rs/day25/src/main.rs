use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    time::SystemTime,
};

const DATA: &'static str = include_str!("../data.txt");

fn parse_graph() -> HashMap<&'static str, Vec<&'static str>> {
    let mut graph = HashMap::new();

    for line in DATA.lines() {
        let from = &line[0..3];

        for to in line.split(": ").last().unwrap().split_ascii_whitespace() {
            let entry = graph.entry(from).or_insert(Vec::new());
            entry.push(to);
            let entry = graph.entry(to).or_insert(Vec::new());
            entry.push(from);
        }
    }

    graph
}

#[derive(Debug, Clone)]
struct State {
    target: &'static str,
    curr: &'static str,
    prev: &'static str,
    seen: Vec<&'static str>,
}

impl State {
    fn new(target: &'static str, next: &'static str) -> Self {
        Self {
            target,
            curr: next,
            prev: target,
            seen: vec![next],
        }
    }

    fn visit(mut self, next: &'static str) -> Self {
        self.seen.push(next);

        Self {
            target: self.target,
            curr: next,
            prev: self.curr,
            seen: self.seen,
        }
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.seen.len().partial_cmp(&self.seen.len())
        // self.seen.len().partial_cmp(&other.seen.len())
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.seen.len().cmp(&self.seen.len())
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.seen.len() == other.seen.len()
    }
}

impl Eq for State {}

fn part_one() -> i32 {
    let mut graph = parse_graph();

    let mut path_map = HashMap::new();
    let mut heap = BinaryHeap::new();
    let mut walked = HashSet::new();
    for (&start, neighbors) in &graph {
        for &neighbor in neighbors {
            if !walked.insert((start, neighbor)) || !walked.insert((neighbor, start)) {
                continue;
            }

            let mut shortest = None;
            heap.clear();
            heap.push(State::new(start, neighbor));

            'outer: while let Some(state) = heap.pop() {
                for &next in graph.get(state.curr).unwrap() {
                    if next == state.prev || state.seen.contains(&next) {
                        continue;
                    }

                    if next == state.target {
                        shortest = Some(state);
                        break 'outer;
                    }
                    heap.push(state.clone().visit(next));
                }
            }

            let shortest = shortest.unwrap();
            match path_map.get_mut(&start) {
                None => {
                    path_map.insert(start, shortest);
                }
                Some(s) if s.seen.len() < shortest.seen.len() => *s = shortest,
                _ => {}
            }
        }
    }

    let mut longest_paths = path_map.into_values().collect::<Vec<_>>();
    longest_paths.sort_by(|a, b| b.cmp(a));

    let a = longest_paths.pop().unwrap();
    let b = longest_paths.pop().unwrap();
    let c = longest_paths.pop().unwrap();

    let one = a.target;
    let two = a.seen.first().copied().unwrap();

    for State { target, seen, .. } in [a, b, c] {
        let &first = seen.first().unwrap();

        for (start, end) in [(target, first), (first, target)] {
            graph.get_mut(start).unwrap().retain(|&n| n != end);
        }
    }

    let mut result = 1;

    for pos in [one, two] {
        let mut stack = vec![pos];
        let mut seen = HashSet::from([pos]);

        while let Some(node) = stack.pop() {
            for &neighbor in graph.get(node).unwrap() {
                if seen.insert(neighbor) {
                    stack.push(neighbor);
                }
            }
        }

        result *= seen.len() as i32;
    }

    result
}

fn part_two() -> i32 {
    0
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

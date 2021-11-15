use std::collections::{HashMap, HashSet, VecDeque};
use std::env;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

struct Graph<'a> {
    map: HashMap<&'a str, Vec<&'a str>>,
}

impl<'a> Graph<'a> {
    fn from(string: &'a str) -> Self {
        let mut map = HashMap::new();

        for line in string.lines() {
            let mut parts = line.split(" <-> ");
            let id = parts.next().unwrap();
            let connections = parts.next().unwrap();

            let vec = connections.split(", ").collect();

            map.insert(id, vec);
        }

        Self { map }
    }
}

fn part_one(graph: &Graph) -> usize {
    let mut seen = HashSet::new();
    let mut queue = VecDeque::from(["0"]);

    while let Some(current) = queue.pop_front() {
        seen.insert(current);
        match graph.map.get(current) {
            Some(vec) => {
                for conn in vec {
                    if !seen.contains(conn) {
                        queue.push_back(conn);
                    }
                }
            }
            None => (),
        }
    }

    seen.len()
}

fn part_two(graph: &Graph) -> usize {
    let mut group_count = 0;
    let mut seen = HashSet::new();
    let mut queue = VecDeque::new();

    for node in graph.map.keys() {
        if seen.contains(node) {
            continue;
        }

        queue.push_back(node);

        while let Some(current) = queue.pop_front() {
            seen.insert(current);
            match graph.map.get(current) {
                Some(vec) => {
                    for conn in vec {
                        if !seen.contains(conn) {
                            queue.push_back(conn);
                        }
                    }
                }
                None => (),
            }
        }

        group_count += 1;
    }

    group_count
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
    let graph = Graph::from(&input);

    time_it(|| println!("part 1: {}", part_one(&graph)));
    time_it(|| println!("part 2: {}", part_two(&graph)));

    Ok(())
}

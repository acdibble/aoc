use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Cave<'a>(&'a str);

impl Cave<'_> {
    fn is_small(&self) -> bool {
        match self.0.chars().nth(0) {
            Some('A'..='Z') => false,
            _ => true,
        }
    }
}

struct CaveNetwork<'a> {
    edges: HashMap<Cave<'a>, Vec<Cave<'a>>>,
}

impl<'a> CaveNetwork<'a> {
    fn new() -> Self {
        Self {
            edges: HashMap::new(),
        }
    }

    fn insert(&mut self, a: Cave<'a>, b: Cave<'a>) {
        for (from, to) in [(a, b), (b, a)] {
            if from.0 == "end" || to.0 == "start" {
                continue;
            }

            let entry_from = self.edges.entry(from).or_default();
            entry_from.push(to);
        }
    }

    fn count_paths_to_end(
        &self,
        current: &Cave<'a>,
        visited: &mut HashSet<Cave<'a>>,
        can_double_visit: bool,
    ) -> i32 {
        let mut paths_found = 0;

        for cave in self.edges.get(&current).unwrap() {
            if cave.0 == "end" {
                paths_found += 1;
                continue;
            }

            let is_small = cave.is_small();
            let mut remove_visited = true;
            let mut can_double_visit = can_double_visit;
            if is_small {
                if visited.contains(cave) {
                    if !can_double_visit {
                        continue;
                    }

                    remove_visited = false;
                    can_double_visit = false;
                }

                visited.insert(*cave);
            }
            paths_found += self.count_paths_to_end(cave, visited, can_double_visit);
            if is_small && remove_visited {
                visited.remove(cave);
            }
        }

        paths_found
    }

    fn count_all_routes(&mut self, can_double_visit: bool) -> i32 {
        let start = Cave("start");

        let mut visited = HashSet::new();
        self.count_paths_to_end(&start, &mut visited, can_double_visit)
    }
}

fn solve(input: &str) -> (i32, i32) {
    let mut network = CaveNetwork::new();

    for line in input.lines() {
        let mut split = line.split('-');
        let a = Cave(split.next().unwrap());
        let b = Cave(split.next().unwrap());
        network.insert(a, b);
    }

    (
        network.count_all_routes(false),
        network.count_all_routes(true),
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

use std::collections::HashSet;
use std::env;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Component(i32, i32);

impl Component {
    fn has_port(&self, value: i32) -> bool {
        self.0 == value || self.1 == value
    }

    fn free_port(&self, used_port: i32) -> i32 {
        if self.0 == used_port {
            self.1
        } else {
            self.0
        }
    }
}

impl std::str::FromStr for Component {
    type Err = String;

    fn from_str(string: &str) -> std::result::Result<Self, <Self as std::str::FromStr>::Err> {
        let mut parts = string.split('/');

        Ok(Component(
            parts
                .next()
                .ok_or_else(|| string.to_owned())?
                .parse::<i32>()
                .or_else(|_| Err(string.to_owned()))?,
            parts
                .next()
                .ok_or_else(|| string.to_owned())?
                .parse::<i32>()
                .or_else(|_| Err(string.to_owned()))?,
        ))
    }
}

#[derive(Debug)]
struct Node {
    used_port: i32,
    free_port: i32,
    children: Vec<Node>,
}

impl Node {
    fn new(component: &Component, used_port: i32) -> Self {
        Self {
            used_port,
            free_port: component.free_port(used_port),
            children: Vec::new(),
        }
    }

    fn add_children(&mut self, components: &Vec<Component>, seen: HashSet<&Component>) {
        for component in components {
            if !seen.contains(component) && component.has_port(self.free_port) {
                let mut node = Node::new(component, self.free_port);
                let mut clone = seen.clone();
                clone.insert(component);
                node.add_children(components, clone);
                self.children.push(node);
            }
        }
    }

    fn find_strengths(&self, running_total: i32, strengths: &mut Vec<i32>) {
        let strength = self.free_port + self.used_port;

        if self.children.len() == 0 {
            strengths.push(running_total + strength)
        } else {
            for child in &self.children {
                child.find_strengths(running_total + strength, strengths)
            }
        }
    }

    fn find_depths_and_strengths(
        &self,
        depth: i32,
        running_total: i32,
        strengths: &mut Vec<(i32, i32)>,
    ) {
        let strength = self.free_port + self.used_port;

        if self.children.len() == 0 {
            strengths.push((depth, running_total + strength))
        } else {
            for child in &self.children {
                child.find_depths_and_strengths(depth + 1, running_total + strength, strengths)
            }
        }
    }
}

fn part_one(root: &Node) -> i32 {
    let mut strengths = Vec::new();

    root.find_strengths(0, &mut strengths);
    strengths.sort();

    *strengths.last().unwrap()
}

fn part_two(root: &Node) -> i32 {
    let mut depths_and_strengths = Vec::new();

    root.find_depths_and_strengths(0, 0, &mut depths_and_strengths);

    depths_and_strengths.sort_by(|a, b| match a.0.cmp(&b.0) {
        std::cmp::Ordering::Equal => a.1.cmp(&b.1),
        ordering => ordering,
    });

    depths_and_strengths.last().unwrap().1
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

    let components: Vec<_> = input
        .lines()
        .map(|l| l.parse::<Component>().unwrap())
        .collect();
    let mut root = Node::new(&Component(0, 0), 0);
    root.add_children(&components, HashSet::new());

    time_it(|| println!("part 1: {}", part_one(&root)));
    time_it(|| println!("part 2: {}", part_two(&root)));

    Ok(())
}

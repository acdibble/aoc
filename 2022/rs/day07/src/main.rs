use std::{str::Lines, time::SystemTime};

const DATA: &'static str = include_str!("../data.txt");

#[derive(Debug)]
enum Dirent {
    File(&'static str, usize),
    Dir(&'static str, usize, Vec<Dirent>),
}

impl Dirent {
    fn add_child(&mut self, child: Dirent) {
        match self {
            Dirent::File(..) => unreachable!(),
            Dirent::Dir(_, size, children) => {
                *size += child.size();
                children.push(child)
            }
        }
    }

    fn size(&self) -> usize {
        *match self {
            Dirent::File(_, size) => size,
            Dirent::Dir(_, size, _) => size,
        }
    }
}

fn build_tree(cd: &'static str, lines: &mut Lines<'static>) -> Option<Dirent> {
    let name = cd.split_ascii_whitespace().last().unwrap();
    if name == ".." {
        return None;
    }

    lines.next();

    let mut dirent = Dirent::Dir(name, 0, vec![]);

    while let Some(line) = lines.next() {
        if line.starts_with("$") {
            match build_tree(line, lines) {
                Some(child) => dirent.add_child(child),
                None => return Some(dirent),
            }
        } else if !line.starts_with("dir") {
            let mut parts = line.split_ascii_whitespace();
            let size: usize = parts.next().unwrap().parse().unwrap();
            let name = parts.next().unwrap();
            dirent.add_child(Dirent::File(name, size));
        }
    }

    Some(dirent)
}

fn parse_structure() -> Dirent {
    let mut lines = DATA.lines();

    let cd = lines.next().unwrap();

    build_tree(cd, &mut lines).unwrap()
}

fn part_one() -> usize {
    let root = parse_structure();
    let mut queue = vec![root];
    let mut total = 0;

    while let Some(dirent) = queue.pop() {
        match dirent {
            Dirent::Dir(_, size, children) => {
                for child in children {
                    queue.push(child)
                }

                if size <= 100000 {
                    total += size;
                }
            }
            _ => continue,
        }
    }

    total
}

fn part_two() -> usize {
    let space_required = 70000000 - 30000000;
    let root = parse_structure();
    let target_increase = root.size() - space_required;

    let mut queue = vec![root];
    let mut result = usize::MAX;

    while let Some(dirent) = queue.pop() {
        match dirent {
            Dirent::Dir(_, size, children) => {
                for child in children {
                    queue.push(child)
                }

                if size > target_increase {
                    result = result.min(size)
                }
            }
            _ => continue,
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
    println!("Time elapsed: {} µs", start.elapsed().unwrap().as_micros());
    result
}

fn main() {
    time_it(|| println!("part 1: {}", part_one()));
    time_it(|| println!("part 2: {}", part_two()));
}

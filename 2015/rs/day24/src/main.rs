use std::env;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

#[derive(Debug)]
struct GroupGenerator {
    stack: Vec<(usize, i32)>,
    running_total: i32,
    target: i32,
    index: usize,
    packages: Vec<i32>,
}

impl GroupGenerator {
    fn new(packages: Vec<i32>, target: i32) -> Self {
        Self {
            index: 0,
            running_total: 0,
            stack: Vec::new(),
            packages,
            target,
        }
    }
}

impl Iterator for GroupGenerator {
    type Item = Vec<i32>;

    fn next(&mut self) -> Option<Self::Item> {
        let package_count = self.packages.len();

        if self.running_total == self.target {
            if let Some((old_index, old_value)) = self.stack.pop() {
                self.running_total -= old_value;
                self.index = old_index + 1;
            } else {
                return None;
            }
        }

        loop {
            if self.index == package_count {
                if let Some((old_index, old_value)) = self.stack.pop() {
                    self.running_total -= old_value;
                    self.index = old_index + 1;
                    continue;
                } else {
                    return None;
                }
            }

            let current = self.packages[self.index];
            let mut return_value = None;

            if self.running_total + current <= self.target {
                self.running_total += current;

                self.stack.push((self.index, current));

                if self.running_total == self.target {
                    return_value = Some(
                        self.stack
                            .iter()
                            .map(|&(_, value)| value)
                            .collect::<Vec<i32>>(),
                    );
                    self.running_total -= current;
                    self.stack.pop();
                }
            }

            self.index += 1;
            if return_value.is_some() {
                return return_value;
            }
        }
    }
}

fn find_entanglement(packages: &Vec<i32>, group_count: usize) -> i128 {
    let mut min_entanglement = std::i128::MAX;
    let mut min_length = std::usize::MAX;
    let target = packages.iter().sum::<i32>() / group_count as i32;
    let init = GroupGenerator::new(packages.clone(), target);
    let mut iterators = Vec::from([init]);
    let mut groups: Vec<Vec<i32>> = Vec::new();

    let mut current_entanglement = 0;

    while !iterators.is_empty() {
        if iterators.len() != group_count - 1 {
            let current_iterator = iterators.last_mut().unwrap();
            if let Some(next_group) = current_iterator.next() {
                if groups.is_empty() {
                    if next_group.len() > min_length {
                        continue;
                    }

                    current_entanglement = next_group.iter().fold(1, |acc, v| acc * *v as i128);

                    if current_entanglement > min_entanglement {
                        continue;
                    }
                }

                groups.push(next_group);
                let filtered = packages
                    .iter()
                    .filter_map(|p| {
                        for group in &groups {
                            if group.contains(p) {
                                return None;
                            }
                        }

                        Some(*p)
                    })
                    .collect::<Vec<i32>>();
                iterators.push(GroupGenerator::new(filtered, target));
            } else {
                groups.pop();
                iterators.pop();
            }
            continue;
        }

        if let Some(mut it) = iterators.pop() {
            while let Some(group) = it.next() {
                let last_group_length: usize = packages
                    .iter()
                    .map(|p| {
                        for group in &groups {
                            if group.contains(p) {
                                return 0;
                            }
                        }

                        if group.contains(p) {
                            return 0;
                        }

                        1
                    })
                    .sum();

                let groups_len =
                    groups.iter().fold(0, |acc, g| acc + g.len()) + group.len() + last_group_length;
                if groups_len == packages.len() {
                    min_length = groups[0].len();
                    min_entanglement = current_entanglement;
                }
            }
        }
        groups.pop();
    }

    min_entanglement
}

fn part_one(packages: &Vec<i32>) -> i128 {
    find_entanglement(packages, 3)
}

fn part_two(packages: &Vec<i32>) -> i128 {
    find_entanglement(packages, 4)
}

fn time_it<F>(fun: F)
where
    F: Fn() -> (),
{
    let start = SystemTime::now();
    fun();
    println!("Time elapsed: {} ms", start.elapsed().unwrap().as_millis())
}

fn main() -> std::io::Result<()> {
    let file_path = env::current_dir()?.join(Path::new("data.txt"));
    let input = fs::read_to_string(file_path)?;

    let mut packages: Vec<i32> = input.lines().map(|line| line.parse().unwrap()).collect();
    packages.sort_by(|a, b| b.cmp(a));

    time_it(|| println!("part 1: {}", part_one(&packages)));
    time_it(|| println!("part 2: {}", part_two(&packages)));

    Ok(())
}

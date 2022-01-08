use std::collections::{HashMap, VecDeque};
use std::env;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

const ITEMS_PER_FLOOR: usize = 14;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
enum Item<'a> {
    Microchip(&'a str),
    Generator(&'a str),
    None,
}

impl Item<'_> {
    fn is_none(&self) -> bool {
        *self == Item::None
    }
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
struct Floor<'a>([Item<'a>; ITEMS_PER_FLOOR]);

impl<'a> Floor<'a> {
    fn insert(&mut self, item: Item<'a>) {
        for slot in &mut self.0 {
            if slot.is_none() {
                *slot = item;
                return;
            }
        }

        println!("{:?}, {:?}", self, item);
        unreachable!("unable to insert")
    }

    fn remove_index(&mut self, index: usize) {
        self.0[index] = Item::None;
    }

    fn sort(&mut self) {
        self.0.sort();
    }

    fn is_valid(&self) -> bool {
        if self.0.iter().any(|item| matches!(item, Item::Generator(_))) {
            for item in self.0.iter() {
                match item {
                    Item::Microchip(kind) => {
                        if !self.0.contains(&Item::Generator(kind)) {
                            return false;
                        }
                    }
                    _ => continue,
                }
            }
        }

        true
    }
}

fn parse_floors(input: &String) -> [Floor; 4] {
    let mut lines = input.lines();

    let mut floors = [Floor([Item::None; ITEMS_PER_FLOOR]); 4];

    for floor in &mut floors {
        let mut words = lines.next().unwrap().split_ascii_whitespace();

        words.next(); // the
        words.next(); // nth
        words.next(); // floor
        words.next(); // contains

        match words.next().unwrap() {
            "nothing" => continue,
            _ => (),
        }

        for i in 0..ITEMS_PER_FLOOR {
            let identifier = words.next();
            if identifier.is_none() {
                break;
            }

            let kind = words.next().unwrap();
            floor.0[i] = match kind {
                "microchip" | "microchip," | "microchip." => {
                    let identifier = identifier.unwrap().split('-').next().unwrap();
                    Item::Microchip(identifier)
                }
                "generator" | "generator," | "generator." => Item::Generator(identifier.unwrap()),
                _ => unreachable!(),
            };

            if kind.ends_with('.') {
                break;
            }

            if words.next() == Some(&"and") {
                words.next();
            }
        }
    }

    floors
}

struct HashQueue<'a> {
    queue: VecDeque<([Floor<'a>; 4], usize, i32)>,
    hash_map: HashMap<([Floor<'a>; 4], usize), i32>,
}

impl<'a> HashQueue<'a> {
    fn new(init: ([Floor<'a>; 4], usize, i32)) -> Self {
        Self {
            hash_map: HashMap::from([((init.0, init.1), init.2)]),
            queue: VecDeque::from([init]),
        }
    }

    fn pop(&mut self) -> Option<([Floor<'a>; 4], usize, i32)> {
        self.queue.pop_front()
    }

    fn push(&mut self, mut item: ([Floor<'a>; 4], usize, i32)) {
        for floor in &mut item.0 {
            if !floor.is_valid() {
                return;
            }
            floor.sort()
        }

        let entry = self.hash_map.entry((item.0, item.1)).or_insert(i32::MAX);

        if item.2 < *entry {
            *entry = item.2;
            self.queue.push_back(item);
        }
    }
}

fn make_moves<'a>(
    hash_queue: &mut HashQueue<'a>,
    floors: [Floor<'a>; 4],
    steps: i32,
    current_floor: usize,
    next_floor: usize,
) {
    let floor = &floors[current_floor];

    for (index, item) in floor.0.iter().enumerate() {
        if item.is_none() {
            continue;
        }

        let mut to_push = floors.clone();
        to_push[current_floor].remove_index(index);
        to_push[next_floor].insert(*item);
        hash_queue.push((to_push, next_floor, steps));

        for (index2, item2) in floor.0.iter().enumerate().skip(index + 1) {
            if item2.is_none() {
                continue;
            }

            let mut to_push = to_push.clone();
            to_push[current_floor].remove_index(index2);
            to_push[next_floor].insert(*item2);
            hash_queue.push((to_push, next_floor, steps));
        }
    }
}

fn find_solution(input: &String) -> i32 {
    let mut floors = parse_floors(input);

    floors[0].insert(Item::Generator("elerium"));
    floors[0].insert(Item::Microchip("elerium"));
    floors[0].insert(Item::Generator("dilithium"));
    floors[0].insert(Item::Microchip("dilithium"));
    floors[0].sort();

    let mut hash_queue = HashQueue::new((floors, 0, 0));

    let mut min_steps = i32::MAX;

    while let Some((floors, elevator_floor, steps)) = hash_queue.pop() {
        if steps > min_steps {
            continue;
        }

        if floors[3].0.iter().rev().all(|item| !item.is_none()) {
            min_steps = min_steps.min(steps)
        }

        let new_steps = steps + 1;

        if elevator_floor == 0 || elevator_floor == 1 || elevator_floor == 2 {
            make_moves(
                &mut hash_queue,
                floors,
                new_steps,
                elevator_floor,
                elevator_floor + 1,
            )
        }

        if elevator_floor == 1 || elevator_floor == 2 || elevator_floor == 3 {
            make_moves(
                &mut hash_queue,
                floors,
                new_steps,
                elevator_floor,
                elevator_floor - 1,
            )
        }
    }

    min_steps
}

fn time_it<F, T>(fun: F) -> T
where
    F: Fn() -> T,
{
    let start = SystemTime::now();
    let result = fun();
    println!("Time elapsed: {} s", start.elapsed().unwrap().as_secs_f64());
    result
}

fn main() -> std::io::Result<()> {
    let file_path = env::current_dir()?.join(Path::new("data.txt"));
    let input = fs::read_to_string(file_path)?;

    time_it(|| println!("part 2: {}", find_solution(&input)));

    Ok(())
}

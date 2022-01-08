use std::collections::{BinaryHeap, HashMap};
use std::env;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
enum Amphipod {
    Amber,
    Bronze,
    Copper,
    Desert,
}

impl Amphipod {
    fn from_char(ch: char) -> Option<Self> {
        match ch {
            'A' => Some(Self::Amber),
            'B' => Some(Self::Bronze),
            'C' => Some(Self::Copper),
            'D' => Some(Self::Desert),
            _ => None,
        }
    }

    fn energy_used(&self, steps: i32) -> i32 {
        steps
            * match self {
                Self::Amber => 1,
                Self::Bronze => 10,
                Self::Copper => 100,
                Self::Desert => 1000,
            }
    }

    fn room_index(&self) -> usize {
        match self {
            Self::Amber => 2,
            Self::Bronze => 4,
            Self::Copper => 6,
            Self::Desert => 8,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct Room<const N: usize> {
    target: Amphipod,
    spaces: [Option<Amphipod>; N],
}

impl<const N: usize> Room<N> {
    const fn new(target: Amphipod) -> Self {
        Self {
            target,
            spaces: [None; N],
        }
    }

    fn all_spaces_are_correct_or_empty(&self) -> bool {
        self.spaces.iter().all(|a| match a {
            Some(amphipod) if *amphipod == self.target => true,
            None => true,
            _ => false,
        })
    }

    fn can_accept(&self, amphipod: &Amphipod) -> bool {
        if self.target != *amphipod {
            return false;
        }

        if self.spaces.iter().all(Option::is_some) {
            return false;
        }

        if self.all_spaces_are_correct_or_empty() {
            return true;
        }

        false
    }

    fn eject(&mut self) -> Option<(Amphipod, i32)> {
        if self.all_spaces_are_correct_or_empty() {
            return None;
        }

        let mut index = N - 1;

        while let Some(None) = self.spaces.get(index) {
            index -= 1;
        }

        self.spaces
            .get_mut(index)
            .unwrap()
            .take()
            .map(|a| (a, (self.spaces.len() - index) as i32))
    }

    fn load_next(&mut self, amphipod: Amphipod) -> Option<i32> {
        let mut index = 0;

        while let Some(Some(_)) = self.spaces.get(index) {
            index += 1;
        }

        let space = self.spaces.get_mut(index).unwrap();

        debug_assert!(space.is_none());

        *space = Some(amphipod);

        Some((self.spaces.len() - index) as i32)
    }

    fn accept(&mut self, amphipod: Amphipod) -> Option<i32> {
        if !self.can_accept(&amphipod) {
            return None;
        }

        self.load_next(amphipod)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
enum Location<const N: usize> {
    Room(Room<N>),
    Hall(Option<Amphipod>),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct Burrow<const N: usize>([Location<N>; 11]);

impl<const N: usize> Default for Burrow<N> {
    fn default() -> Self {
        Burrow([
            Location::Hall(None),
            Location::Hall(None),
            Location::Room(Room::new(Amphipod::Amber)),
            Location::Hall(None),
            Location::Room(Room::new(Amphipod::Bronze)),
            Location::Hall(None),
            Location::Room(Room::new(Amphipod::Copper)),
            Location::Hall(None),
            Location::Room(Room::new(Amphipod::Desert)),
            Location::Hall(None),
            Location::Hall(None),
        ])
    }
}

impl<const N: usize> Burrow<N> {
    fn from_str(input: &str) -> Self {
        let mut lines = input.lines();
        let mut hallway_chars = lines.nth(1).unwrap().chars();
        hallway_chars.next();

        let location_0 = Location::Hall(Amphipod::from_char(hallway_chars.next().unwrap()));
        let location_1 = Location::Hall(Amphipod::from_char(hallway_chars.next().unwrap()));
        hallway_chars.next();
        let location_3 = Location::Hall(Amphipod::from_char(hallway_chars.next().unwrap()));
        hallway_chars.next();
        let location_5 = Location::Hall(Amphipod::from_char(hallway_chars.next().unwrap()));
        hallway_chars.next();
        let location_7 = Location::Hall(Amphipod::from_char(hallway_chars.next().unwrap()));
        hallway_chars.next();
        let location_9 = Location::Hall(Amphipod::from_char(hallway_chars.next().unwrap()));
        let location_10 = Location::Hall(Amphipod::from_char(hallway_chars.next().unwrap()));

        let mut room_a = Room::new(Amphipod::Amber);
        let mut room_b = Room::new(Amphipod::Bronze);
        let mut room_c = Room::new(Amphipod::Copper);
        let mut room_d = Room::new(Amphipod::Desert);

        let mut reversed = lines.rev();
        reversed.next();

        while let Some(line) = reversed.next() {
            let mut chars = line.chars();
            if let Some(amphipod) = Amphipod::from_char(chars.nth(3).unwrap()) {
                room_a.load_next(amphipod);
            }
            if let Some(amphipod) = Amphipod::from_char(chars.nth(1).unwrap()) {
                room_b.load_next(amphipod);
            }
            if let Some(amphipod) = Amphipod::from_char(chars.nth(1).unwrap()) {
                room_c.load_next(amphipod);
            }
            if let Some(amphipod) = Amphipod::from_char(chars.nth(1).unwrap()) {
                room_d.load_next(amphipod);
            }
        }

        Self([
            location_0,
            location_1,
            Location::Room(room_a),
            location_3,
            Location::Room(room_b),
            location_5,
            Location::Room(room_c),
            location_7,
            Location::Room(room_d),
            location_9,
            location_10,
        ])
    }

    fn is_path_clear(&self, start: usize, end: usize) -> bool {
        let a;
        let b;
        if start < end {
            a = start + 1;
            b = end;
        } else {
            a = end;
            b = start - 1;
        }

        for index in a..=b {
            if !matches!(self.0[index], Location::Hall(None) | Location::Room(_)) {
                return false;
            }
        }

        true
    }

    fn create_clone(
        &self,
        index1: usize,
        mut loc1: Location<N>,
        index2: usize,
        mut loc2: Location<N>,
    ) -> Self {
        let mut clone = *self;

        std::mem::swap(&mut clone.0[index1], &mut loc1);
        std::mem::swap(&mut clone.0[index2], &mut loc2);

        clone
    }
}

struct HeapElement<const N: usize>(Burrow<N>, i32);

impl<const N: usize> PartialEq for HeapElement<N> {
    fn eq(&self, other: &Self) -> bool {
        self.1 == other.1
    }
}

impl<const N: usize> PartialOrd for HeapElement<N> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.1.cmp(&self.1))
    }
}

impl<const N: usize> Eq for HeapElement<N> {}

impl<const N: usize> Ord for HeapElement<N> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.1.cmp(&self.1)
    }
}

struct CacheHeap<const N: usize> {
    heap: BinaryHeap<HeapElement<N>>,
    cache: HashMap<Burrow<N>, i32>,
    min_energy: i32,
    solved: Burrow<N>,
}

impl<const N: usize> CacheHeap<N> {
    fn new(burrow: Burrow<N>) -> Self {
        let solved;
        if N == 2 {
            solved = Burrow::from_str(
                "#############
#...........#
###A#B#C#D###
  #A#B#C#D#
  #########",
            );
        } else {
            solved = Burrow::from_str(
                "#############
#...........#
###A#B#C#D###
  #A#B#C#D#
  #A#B#C#D#
  #A#B#C#D#
  #########",
            );
        }

        let mut heap = BinaryHeap::new();
        heap.push(HeapElement(burrow, 0));

        Self {
            solved,
            heap,
            cache: HashMap::new(),
            min_energy: i32::MAX,
        }
    }

    fn pop(&mut self) -> Option<HeapElement<N>> {
        self.heap.pop()
    }

    fn push(&mut self, value: HeapElement<N>) {
        if value.1 >= self.min_energy {
            return;
        }

        let entry = self.cache.entry(value.0).or_insert(i32::MAX);

        if *entry <= value.1 {
            return;
        }

        *entry = value.1;

        if value.0 == self.solved {
            self.min_energy = value.1.min(self.min_energy);
            return;
        }

        self.heap.push(value)
    }
}

fn solve<const N: usize>(input: &str) -> i32 {
    let burrow = Burrow::<N>::from_str(input);

    let mut heap = CacheHeap::new(burrow);

    while let Some(HeapElement(burrow, energy)) = heap.pop() {
        for (starting_index, location) in burrow.0.iter().enumerate() {
            match location {
                &Location::Room(mut room) => {
                    if let Some((ejected, steps_into_hall)) = room.eject() {
                        for i in 0..11 {
                            if !burrow.is_path_clear(starting_index, i) {
                                continue;
                            }
                            match burrow.0[i] {
                                Location::Room(mut incoming_room) => {
                                    if let Some(steps_into_room) = incoming_room.accept(ejected) {
                                        let mut injected = incoming_room;
                                        injected.accept(ejected);
                                        let new_burrow = burrow.create_clone(
                                            i,
                                            Location::Room(injected),
                                            starting_index,
                                            Location::Room(room),
                                        );
                                        heap.push(HeapElement(
                                            new_burrow,
                                            energy
                                                + ejected.energy_used(
                                                    steps_into_hall
                                                        + steps_into_room
                                                        + (starting_index as i32 - i as i32).abs(),
                                                ),
                                        ));
                                    }
                                }
                                Location::Hall(None) => {
                                    let new_burrow = burrow.create_clone(
                                        i,
                                        Location::Hall(Some(ejected)),
                                        starting_index,
                                        Location::Room(room),
                                    );
                                    heap.push(HeapElement(
                                        new_burrow,
                                        energy
                                            + ejected.energy_used(
                                                steps_into_hall
                                                    + (starting_index as i32 - i as i32).abs(),
                                            ),
                                    ));
                                }
                                Location::Hall(_) => {}
                            }
                        }
                    }
                }
                Location::Hall(Some(amphipod)) => {
                    let room_index = amphipod.room_index();
                    if let Location::Room(mut room) = burrow.0[room_index] {
                        if burrow.is_path_clear(starting_index, room_index) {
                            if let Some(steps) = room.accept(*amphipod) {
                                let new_burrow = burrow.create_clone(
                                    room_index,
                                    Location::Room(room),
                                    starting_index,
                                    Location::Hall(None),
                                );
                                heap.push(HeapElement(
                                    new_burrow,
                                    energy
                                        + amphipod.energy_used(
                                            steps
                                                + (starting_index as i32 - room_index as i32).abs(),
                                        ),
                                ));
                            }
                        }
                    } else {
                        unreachable!()
                    }
                }
                Location::Hall(None) => {}
            }
        }
    }

    heap.min_energy
}

fn part_two(input: &str) -> i32 {
    let mut new_input = String::new();
    let mut lines = input.lines();
    new_input.push_str(lines.next().unwrap());
    new_input.push('\n');
    new_input.push_str(lines.next().unwrap());
    new_input.push('\n');
    new_input.push_str(lines.next().unwrap());
    new_input.push('\n');
    new_input.push_str("  #D#C#B#A#\n");
    new_input.push_str("  #D#B#A#C#\n");
    new_input.push_str(lines.next().unwrap());
    new_input.push('\n');
    new_input.push_str(lines.next().unwrap());
    new_input.push('\n');

    solve::<4>(&new_input)
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

    time_it(|| println!("part 1: {}", solve::<2>(&input)));
    time_it(|| println!("part 2: {}", part_two(&input)));

    Ok(())
}

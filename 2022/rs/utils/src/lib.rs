use std::{
    collections::{HashMap, VecDeque},
    fmt::{Debug, Display},
    hash::Hash,
};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Coord(pub i64, pub i64);

impl Coord {
    pub fn distance_to(&self, other: &Self) -> i64 {
        (self.0 - other.0).abs() + (self.1 - other.1).abs()
    }

    pub fn translate_y(&self, distance: i64) -> Self {
        Self(self.0, self.1 + distance)
    }

    pub fn translate_x(&self, distance: i64) -> Self {
        Self(self.0 + distance, self.1)
    }
}

impl std::ops::Add<Coord> for Coord {
    type Output = Self;

    fn add(self, rhs: Coord) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Debug for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

impl Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub struct Graph<T>
where
    T: PartialEq + Eq + Hash + Debug + Copy,
{
    edges: HashMap<T, Vec<T>>,
    distance_cache: HashMap<(T, T), i32>,
}

impl<T> Graph<T>
where
    T: PartialEq + Eq + Hash + Debug + Copy,
{
    pub fn new() -> Self {
        Self {
            edges: HashMap::new(),
            distance_cache: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, value: T) {
        self.edges.insert(value, vec![]);
    }

    pub fn add_edge(&mut self, from: T, to: T) {
        let entry = self.edges.entry(from).or_default();
        entry.push(to);
    }

    pub fn get_edges(&mut self, from: T) -> &Vec<T> {
        self.edges.get(&from).unwrap()
    }

    pub fn distance_between(&mut self, from: T, to: T) -> i32 {
        if let Some(distance) = self.distance_cache.get(&(from, to)) {
            return *distance;
        }

        let mut queue: VecDeque<_> = self
            .edges
            .get(&from)
            .unwrap()
            .iter()
            .map(|loc| (loc, 1))
            .collect();

        while let Some((location, steps)) = queue.pop_front() {
            let entry = self
                .distance_cache
                .entry((from, *location))
                .or_insert(i32::MAX);
            *entry = steps.min(*entry);

            if *location == to {
                self.distance_cache.insert((from, to), steps);
                self.distance_cache.insert((to, from), steps);
                return steps;
            }

            for next in self.edges.get(location).unwrap() {
                queue.push_back((next, steps + 1))
            }
        }

        unreachable!()
    }
}

// pub struct Segment {
//     start: Coord,
//     end: Coord,
// }

// impl Segment {
//     pub fn new(start: Coord, end: Coord) -> Self {
//         Self { start, end }
//     }

//     pub fn len(&self) -> i64 {
//         self.start.distance_to(&self.end)
//     }

//     pub fn iter(&self) -> impl Iterator<Item = Coord> + '_ {
//         let distance = self.len();
//         let dy = self.start.1 - self.end.1;
//         let dx = self.start.0 - self.end.0;

//         (0..=distance)
//             .map(move |i| Coord(self.start.0 + dx * i, self.start.1 + dy * i))
//             .into_iter()
//     }
// }

// pub struct Chart<T>
// where
//     T: Default + Clone + Copy + Display + PartialEq + Eq,
// {
//     data: HashMap<Coord, T>,
//     min_x: i64,
//     max_x: i64,
//     min_y: i64,
//     max_y: i64,
// }

// impl<T> Display for Chart<T>
// where
//     T: Default + Clone + Copy + Display + PartialEq + Eq,
// {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         for y in self.min_y..=self.max_y {
//             for x in self.min_x..=self.max_x {
//                 write!(f, "{}", self.get(&Coord(x, y)).copied().unwrap_or_default())?;
//             }
//             write!(f, "\n")?;
//         }

//         Ok(())
//     }
// }

// impl<T> Chart<T>
// where
//     T: Default + Clone + Copy + Display + PartialEq + Eq,
// {
//     pub fn new() -> Self {
//         Self {
//             data: HashMap::new(),
//             min_x: 0,
//             min_y: 0,
//             max_x: 0,
//             max_y: 0,
//         }
//     }

//     pub fn get(&self, coord: &Coord) -> Option<&T> {
//         self.data.get(coord)
//     }

//     pub fn get_mut(&mut self, coord: &Coord) -> Option<&mut T> {
//         self.data.get_mut(coord)
//     }

//     pub fn overwrite(&mut self, coord: &Coord, value: T) -> Option<T> {
//         self.min_x = self.min_x.min(coord.0);
//         self.max_x = self.max_x.max(coord.0);
//         self.min_y = self.min_y.min(coord.1);
//         self.max_y = self.max_y.max(coord.1);
//         self.data.insert(*coord, value)
//     }

//     pub fn set(&mut self, coord: &Coord, mut value: T) -> Option<T> {
//         match self.get_mut(coord) {
//             Some(entry) if *entry == Default::default() => {
//                 std::mem::swap(entry, &mut value);
//                 Some(value)
//             }
//             None => self.overwrite(coord, value),
//             _ => None,
//         }
//     }

//     pub fn print(&self) {
//         println!("{}", self)
//     }
// }

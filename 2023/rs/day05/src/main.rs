use std::time::SystemTime;

const DATA: &'static str = include_str!("../data.txt");

#[derive(Debug)]
struct MapRange {
    source: usize,
    destination: usize,
    length: usize,
}

impl MapRange {
    fn try_map_value(&self, value: usize) -> Option<usize> {
        if value >= self.source && value < self.source + self.length {
            let diff = value - self.source;

            return Some(diff + self.destination);
        }

        None
    }

    fn try_reverse_map_value(&self, value: usize) -> Option<usize> {
        if value >= self.destination && value < self.destination + self.length {
            let diff = value - self.destination;

            return Some(diff + self.source);
        }

        None
    }
}

#[derive(Debug)]
struct Map {
    ranges: Vec<MapRange>,
}

impl Map {
    fn map_value(&self, value: usize) -> usize {
        self.ranges
            .iter()
            .find_map(|range| range.try_map_value(value))
            .unwrap_or(value)
    }

    fn reverse_map_value(&self, value: usize) -> usize {
        self.ranges
            .iter()
            .find_map(|range| range.try_reverse_map_value(value))
            .unwrap_or(value)
    }
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<usize>,
    maps: Vec<Map>,
}

impl Almanac {
    fn new() -> Self {
        let mut it = DATA.trim().lines();

        let seeds = it
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .skip(1) // skip "seeds:"
            .map(|seed| seed.parse::<usize>().unwrap())
            .collect();

        it.next(); // whitespace
        let mut maps = Vec::new();

        while let Some(_) = it.next() {
            let mut map = Map { ranges: Vec::new() };

            while let Some(range) = it.next() {
                if range.is_empty() {
                    break;
                }
                let mut range = range.split_ascii_whitespace();
                map.ranges.push(MapRange {
                    destination: range.next().unwrap().parse::<usize>().unwrap(),
                    source: range.next().unwrap().parse::<usize>().unwrap(),
                    length: range.next().unwrap().parse::<usize>().unwrap(),
                });
            }

            maps.push(map);
        }

        Self { seeds, maps }
    }

    fn find_lowest_location(&self) -> usize {
        self.seeds
            .iter()
            .map(|seed| self.maps.iter().fold(*seed, |acc, map| map.map_value(acc)))
            .min()
            .unwrap()
    }

    fn find_lowest_location_of_seed_ranges(&self) -> usize {
        let seed_ranges = self
            .seeds
            .chunks_exact(2)
            .map(|range| (range[0]..(range[0] + range[1])))
            .collect::<Vec<_>>();

        let mut lowest = usize::MAX;

        for i in 0..self.maps.len() {
            let (reverse_maps, forward_maps) = self.maps.split_at(i);

            let current_map = &forward_maps[0];
            let forward_maps = &forward_maps[1..];

            for range in &current_map.ranges {
                let seed = reverse_maps
                    .iter()
                    .rev()
                    .fold(range.source, |acc, map| map.reverse_map_value(acc));
                let output = forward_maps
                    .iter()
                    .fold(range.destination, |acc, map| map.map_value(acc));

                if seed_ranges.iter().any(|range| range.contains(&seed)) {
                    lowest = lowest.min(output)
                }
            }
        }

        lowest
    }
}

fn part_one() -> usize {
    Almanac::new().find_lowest_location()
}

fn part_two() -> usize {
    Almanac::new().find_lowest_location_of_seed_ranges()
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

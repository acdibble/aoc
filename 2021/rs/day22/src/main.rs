use std::env;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Cube {
    x1: i128,
    x2: i128,
    y1: i128,
    y2: i128,
    z1: i128,
    z2: i128,
    intersections: Vec<Cube>,
}

impl Cube {
    fn volume(&self) -> i128 {
        (self.x2 - self.x1 + 1) * (self.y2 - self.y1 + 1) * (self.z2 - self.z1 + 1)
            - self.intersections.iter().fold(0, |acc, c| acc + c.volume())
    }

    fn intersect(&mut self, other: &Self) {
        let x1 = self.x1.max(other.x1);
        let x2 = self.x2.min(other.x2);
        if x1 > x2 {
            return;
        }
        let y1 = self.y1.max(other.y1);
        let y2 = self.y2.min(other.y2);
        if y1 > y2 {
            return;
        }
        let z1 = self.z1.max(other.z1);
        let z2 = self.z2.min(other.z2);
        if z1 > z2 {
            return;
        }

        let mut new = Cube {
            x1,
            x2,
            y1,
            y2,
            z1,
            z2,
            intersections: vec![],
        };

        for intersection in &mut self.intersections {
            intersection.intersect(&mut new)
        }

        self.intersections.push(new);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_split() {
        let (_, mut a) = parse_cube("on x=10..12,y=10..12,z=10..12");
        let (_, mut b) = parse_cube("on x=11..13,y=11..13,z=11..13");

        println!("a: {}", a.volume());
        println!("b: {}", b.volume());

        a.intersect(&mut b);

        println!("a after: {}", a.volume());
        println!("b after: {}", b.volume());
        println!("a + b: {}", a.volume() + b.volume());

        let (_, mut c) = parse_cube("off x=9..11,y=9..11,z=9..11");
        a.intersect(&mut c);
        b.intersect(&mut c);

        println!("a + b again: {}", a.volume() + b.volume());

        let (_, mut d) = parse_cube("on x=10..10,y=10..10,z=10..10");

        a.intersect(&mut d);
        b.intersect(&mut d);
        c.intersect(&mut d);

        assert_eq!(39, a.volume() + b.volume() + d.volume());
    }
}

fn parse_range(range: &str) -> (i128, i128) {
    let mut parts = range[2..].split("..");

    (
        parts.next().unwrap().parse().unwrap(),
        parts.next().unwrap().parse().unwrap(),
    )
}

fn parse_cube(line: &str) -> (&str, Cube) {
    let mut parts = line.split_ascii_whitespace();

    let state = parts.next().unwrap();

    let mut ranges = parts.next().unwrap().split(',');
    let (x1, x2) = parse_range(ranges.next().unwrap());
    let (y1, y2) = parse_range(ranges.next().unwrap());
    let (z1, z2) = parse_range(ranges.next().unwrap());

    (
        state,
        Cube {
            x1,
            x2,
            y1,
            y2,
            z1,
            z2,
            intersections: vec![],
        },
    )
}

fn solve(input: &str, limits: Option<(i128, i128)>) -> i128 {
    let mut stack = Vec::<Cube>::new();

    for (state, mut cube) in input.lines().map(parse_cube) {
        if let Some((lower_limit, upper_limit)) = limits {
            if cube.x1 < lower_limit
                || cube.x2 > upper_limit
                || cube.y1 < lower_limit
                || cube.y2 > upper_limit
                || cube.z1 < lower_limit
                || cube.z2 > upper_limit
            {
                continue;
            }
        }

        for other in &mut stack {
            other.intersect(&mut cube);
        }

        if state == "on" {
            stack.push(cube)
        }
    }

    stack.into_iter().fold(0, |acc, c| acc + c.volume())
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

    time_it(|| println!("part 1: {}", solve(&input, Some((-50, 50)))));
    time_it(|| println!("part 2: {}", solve(&input, None)));

    Ok(())
}

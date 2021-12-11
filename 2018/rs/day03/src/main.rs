use std::collections::HashSet;
use std::env;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

const GRID_SIZE: usize = 1000;

type Grid = Vec<Vec<Vec<u16>>>;

#[derive(Debug, Default)]
struct Claim {
    pub x: usize,
    pub y: usize,
    pub rise: usize,
    pub run: usize,
    pub holder: u16,
}

fn parse_string(string: &str) -> Claim {
    let mut claim: Claim = Default::default();
    let mut iter = string.split_whitespace();

    claim.holder = iter.next().unwrap()[1..].parse::<u16>().unwrap();
    iter.next();

    let coords = iter.next().unwrap();
    for (i, v) in coords[..coords.len() - 1].split(",").enumerate() {
        let parsed = v.parse().unwrap();
        if i == 0 {
            claim.x = parsed
        } else {
            claim.y = parsed
        }
    }

    let rise_run = iter.next().unwrap();
    for (i, v) in rise_run.split("x").enumerate() {
        let parsed = v.parse().unwrap();
        if i == 0 {
            claim.run = parsed
        } else {
            claim.rise = parsed
        }
    }

    claim
}

fn solve(claims: &Vec<Claim>) -> (i32, u16) {
    let mut grid: Grid = (0..GRID_SIZE)
        .map(|_| (0..GRID_SIZE).map(|_| Vec::new()).collect())
        .collect();
    let mut overlaps = 0;
    let mut unique_claimants: HashSet<u16> = claims.iter().map(|claim| claim.holder).collect();

    for claim in claims {
        for x in claim.x..(claim.x + claim.run) {
            for y in claim.y..(claim.y + claim.rise) {
                let claimants = &mut grid[x][y];
                claimants.push(claim.holder);
                let len = claimants.len();
                if len > 1 {
                    if len == 2 {
                        overlaps += 1;
                    }
                    for claimant in claimants {
                        unique_claimants.remove(claimant);
                    }
                }
            }
        }
    }

    if unique_claimants.len() != 1 {
        panic!("at the disco");
    }

    (overlaps, *unique_claimants.iter().nth(0).unwrap())
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

    let claims: Vec<_> = input.lines().map(parse_string).collect();

    time_it(|| println!("parts (1, 2): {:?}", solve(&claims)));

    Ok(())
}

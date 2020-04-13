use std::collections::HashSet;
use std::fs::File;
use std::io::{BufReader, BufRead};

#[derive(Debug)]
#[derive(Default)]
struct Claim {
    pub x: u16,
    pub y: u16,
    pub rise: u16,
    pub run: u16,
    pub holder: u16
}

fn parse_string(string: String) -> Claim {
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

fn main() {
    let mut matrix: Vec<Vec<Vec<u16>>> = Vec::with_capacity(1000);;
    for i in 0..1000 {
        matrix.push(Vec::with_capacity(1000););
        for _ in 0..1000 {
            matrix[i].push(Vec::<u16>::new());
        }
    }

    let claims: Vec<Claim> = BufReader::new(File::open("day03/input.txt").unwrap())
        .lines()
        .map(|l| parse_string(l.unwrap()))
        .collect();

    for claim in &claims {
        for x in claim.x..(claim.x + claim.run) {
            for y in claim.y..(claim.y + claim.rise) {
                matrix[x as usize][y as usize].push(claim.holder);
            }
        }
    }

    let mut holders: HashSet<u16> = claims.into_iter().map(|claim| claim.holder).collect();

    for column in matrix.iter() {
        for row in column.iter() {
            if row.len() > 1 {
                for holder in row.iter() {
                    holders.remove(holder);
                }
            }
        }
    }

    println!("holders: {:?}", holders);
}

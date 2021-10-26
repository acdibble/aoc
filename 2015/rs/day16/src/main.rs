use std::env;
use std::fs;
use std::path::Path;

#[derive(Debug, Default)]
struct Sue {
    children: Option<usize>,
    cats: Option<usize>,
    samoyeds: Option<usize>,
    pomeranians: Option<usize>,
    akitas: Option<usize>,
    vizslas: Option<usize>,
    goldfish: Option<usize>,
    trees: Option<usize>,
    cars: Option<usize>,
    perfumes: Option<usize>,
}

fn parse_sue(line: &str) -> Sue {
    let mut it = line.split_ascii_whitespace();

    it.next();
    it.next();

    let mut sue: Sue = Default::default();

    for _ in 0..3 {
        let field = it.next().unwrap();

        let value = it.next().unwrap();
        let end = if value.ends_with(",") {
            value.len() - 1
        } else {
            value.len()
        };
        let value: usize = value[0..end].parse().unwrap();

        match field {
            "children:" => sue.children = Some(value),
            "cats:" => sue.cats = Some(value),
            "samoyeds:" => sue.samoyeds = Some(value),
            "pomeranians:" => sue.pomeranians = Some(value),
            "akitas:" => sue.akitas = Some(value),
            "vizslas:" => sue.vizslas = Some(value),
            "goldfish:" => sue.goldfish = Some(value),
            "trees:" => sue.trees = Some(value),
            "cars:" => sue.cars = Some(value),
            "perfumes:" => sue.perfumes = Some(value),
            _ => unreachable!(),
        }
    }

    sue
}

fn parse_sues(input: &String) -> Vec<Sue> {
    let mut sues = Vec::with_capacity(500);

    for line in input.lines() {
        sues.push(parse_sue(line))
    }

    sues
}

fn part_one(sues: &Vec<Sue>, sought: &Sue) -> usize {
    sues.iter()
        .position(|sue| {
            (sue.children.is_none() || sue.children == sought.children)
                && (sue.cats.is_none() || sue.cats == sought.cats)
                && (sue.samoyeds.is_none() || sue.samoyeds == sought.samoyeds)
                && (sue.pomeranians.is_none() || sue.pomeranians == sought.pomeranians)
                && (sue.akitas.is_none() || sue.akitas == sought.akitas)
                && (sue.vizslas.is_none() || sue.vizslas == sought.vizslas)
                && (sue.goldfish.is_none() || sue.goldfish == sought.goldfish)
                && (sue.trees.is_none() || sue.trees == sought.trees)
                && (sue.cars.is_none() || sue.cars == sought.cars)
                && (sue.perfumes.is_none() || sue.perfumes == sought.perfumes)
        })
        .unwrap()
        + 1
}

fn part_two(sues: &Vec<Sue>, sought: &Sue) -> usize {
    sues.iter()
        .position(|sue| {
            (sue.children.is_none() || sue.children == sought.children)
                && (sue.cats.is_none() || sue.cats > sought.cats)
                && (sue.samoyeds.is_none() || sue.samoyeds == sought.samoyeds)
                && (sue.pomeranians.is_none() || sue.pomeranians < sought.pomeranians)
                && (sue.akitas.is_none() || sue.akitas == sought.akitas)
                && (sue.vizslas.is_none() || sue.vizslas == sought.vizslas)
                && (sue.goldfish.is_none() || sue.goldfish < sought.goldfish)
                && (sue.trees.is_none() || sue.trees > sought.trees)
                && (sue.cars.is_none() || sue.cars == sought.cars)
                && (sue.perfumes.is_none() || sue.perfumes == sought.perfumes)
        })
        .unwrap()
        + 1
}

fn main() -> std::io::Result<()> {
    let file_path = env::current_dir()?.join(Path::new("data.txt"));
    let input = fs::read_to_string(file_path)?;
    let sues = parse_sues(&input);

    let sought_sue = Sue {
        children: Some(3),
        cats: Some(7),
        samoyeds: Some(2),
        pomeranians: Some(3),
        akitas: Some(0),
        vizslas: Some(0),
        goldfish: Some(5),
        trees: Some(3),
        cars: Some(2),
        perfumes: Some(1),
    };

    println!("{:?}", &sues);

    println!("part 1: {}", part_one(&sues, &sought_sue));
    println!("part 2: {}", part_two(&sues, &sought_sue));

    Ok(())
}

use std::cmp;
use std::env;
use std::fs;
use std::path::Path;

#[derive(Debug, Copy, Clone)]
struct Ingredient {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

fn parse_with_comma(string: Option<&str>) -> i32 {
    let value = string.unwrap();
    let value = &value[0..value.len() - 1];
    value.parse().unwrap()
}

impl Ingredient {
    fn from(string: &str) -> Ingredient {
        let mut it = string.split_ascii_whitespace();

        it.next().unwrap();
        // let name = name[0..name.len() - 1].to_owned();

        it.next();
        let capacity = parse_with_comma(it.next());

        it.next();
        let durability = parse_with_comma(it.next());

        it.next();
        let flavor = parse_with_comma(it.next());

        it.next();
        let texture = parse_with_comma(it.next());

        it.next();
        let calories = it.next().unwrap().parse().unwrap();

        Ingredient {
            capacity,
            durability,
            flavor,
            texture,
            calories,
        }
    }
}

struct PermutationGenerator {
    total: i32,
    amounts: Vec<i32>,
}

impl PermutationGenerator {
    fn new(total: i32, size: usize) -> PermutationGenerator {
        PermutationGenerator {
            total,
            amounts: vec![0; size],
        }
    }
}

impl Iterator for PermutationGenerator {
    type Item = Vec<i32>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut i = 0;
        while i < self.amounts.len() {
            if self.amounts[i] < self.total {
                self.amounts[i] += 1;
                i = 0;
            } else {
                self.amounts[i] = 0;
                i += 1;
                continue;
            }

            if self.amounts.iter().sum::<i32>() == self.total {
                return Some(self.amounts.clone());
            }
        }

        None
    }
}

fn find_high_score(ingredients: &[Ingredient], caloric_target: Option<i32>) -> i32 {
    let mut max_score = 0;

    for perm in PermutationGenerator::new(100, ingredients.len()) {
        if let Some(target) = caloric_target {
            let calories = perm.iter().enumerate().fold(0, |acc, (index, amount)| {
                acc + ingredients[index].calories * amount
            });

            if calories != target {
                continue;
            }
        }

        let capacity = perm.iter().enumerate().fold(0, |acc, (index, amount)| {
            acc + ingredients[index].capacity * amount
        });
        let durability = perm.iter().enumerate().fold(0, |acc, (index, amount)| {
            acc + ingredients[index].durability * amount
        });
        let flavor = perm.iter().enumerate().fold(0, |acc, (index, amount)| {
            acc + ingredients[index].flavor * amount
        });
        let texture = perm.iter().enumerate().fold(0, |acc, (index, amount)| {
            acc + ingredients[index].texture * amount
        });

        let score = cmp::max(capacity, 0)
            * cmp::max(durability, 0)
            * cmp::max(flavor, 0)
            * cmp::max(texture, 0);

        max_score = cmp::max(score, max_score);
    }

    max_score
}

fn part_one(ingredients: &[Ingredient]) -> i32 {
    find_high_score(ingredients, None)
}

fn part_two(ingredients: &[Ingredient]) -> i32 {
    find_high_score(ingredients, Some(500))
}

fn main() -> std::io::Result<()> {
    let file_path = env::current_dir()?.join(Path::new("data.txt"));
    let input = fs::read_to_string(file_path)?;

    let ingredients: Vec<_> = input.lines().map(Ingredient::from).collect();

    println!("part 1: {}", part_one(&ingredients));
    println!("part 2: {}", part_two(&ingredients));

    Ok(())
}

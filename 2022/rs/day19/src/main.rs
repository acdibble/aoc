use regex::Regex;
use std::{collections::HashMap, time::SystemTime};

const DATA: &'static str = include_str!("../data.txt");

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Resource {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[derive(Debug)]
struct Cost {
    robot: Resource,
    ore_cost: i32,
    other_cost: Option<(Resource, i32)>,
}

#[derive(Debug)]
struct Blueprint {
    id: i32,
    costs: [Cost; 4],
}

fn parse_blueprints() -> Vec<Blueprint> {
    let id_regex = Regex::new(r"Blueprint (\d+)").unwrap();
    let ore_robot_regex = Regex::new(r"ore robot costs (\d+) ore").unwrap();
    let clay_robot_regex = Regex::new(r"clay robot costs (\d+) ore").unwrap();
    let obsidian_robot_regex =
        Regex::new(r"obsidian robot costs (\d+) ore and (\d+) clay").unwrap();
    let geode_robot_regex = Regex::new(r"geode robot costs (\d+) ore and (\d+) obsidian").unwrap();

    DATA.lines()
        .map(|line| {
            let id = id_regex.captures(line).unwrap();
            let ore_robot = ore_robot_regex.captures(line).unwrap();
            let clay_robot = clay_robot_regex.captures(line).unwrap();
            let obsidian_robot = obsidian_robot_regex.captures(line).unwrap();
            let geode_robot = geode_robot_regex.captures(line).unwrap();

            Blueprint {
                id: id[1].parse().unwrap(),
                costs: [
                    Cost {
                        robot: Resource::Geode,
                        ore_cost: geode_robot[1].parse().unwrap(),
                        other_cost: Some((Resource::Obsidian, geode_robot[2].parse().unwrap())),
                    },
                    Cost {
                        robot: Resource::Obsidian,
                        ore_cost: obsidian_robot[1].parse().unwrap(),
                        other_cost: Some((Resource::Clay, obsidian_robot[2].parse().unwrap())),
                    },
                    Cost {
                        robot: Resource::Clay,
                        ore_cost: clay_robot[1].parse().unwrap(),
                        other_cost: None,
                    },
                    Cost {
                        robot: Resource::Ore,
                        ore_cost: ore_robot[1].parse().unwrap(),
                        other_cost: None,
                    },
                ],
            }
        })
        .collect()
}

fn get_required_time(
    robots: &mut HashMap<Resource, i32>,
    resources: &mut HashMap<Resource, i32>,
    resource: Resource,
    amount_required: i32,
) -> Option<i32> {
    let robot_count = *robots.entry(resource).or_default();
    if robot_count == 0 {
        return None;
    }
    let existing_amount = *resources.entry(resource).or_default();
    let required_ore = (amount_required - existing_amount).max(0);
    Some(((required_ore + robot_count - 1) / robot_count) + 1)
}

fn max_geodes(
    robots: &HashMap<Resource, i32>,
    resources: &HashMap<Resource, i32>,
    time_remaining: i32,
) -> i32 {
    let mut geode_robot_count = robots.get(&Resource::Geode).copied().unwrap_or_default();
    let current_count = resources.get(&Resource::Geode).copied().unwrap_or_default();

    (0..time_remaining).fold(current_count, |acc, _| {
        let new_amount = geode_robot_count;
        geode_robot_count += 1;
        acc + new_amount
    })
}

fn simulate(blueprints: &Vec<Blueprint>, initial_time: i32) -> Vec<i32> {
    let mut result = Vec::with_capacity(blueprints.len());

    for blueprint in blueprints.iter() {
        let mut stack: Vec<_> = blueprint
            .costs
            .iter()
            .map(|cost| {
                (
                    HashMap::from([(Resource::Ore, 1)]),
                    HashMap::from([(Resource::Ore, 0)]),
                    initial_time,
                    cost,
                )
            })
            .collect();
        let mut max = 0;

        while let Some((mut robots, mut resources, time_remaining, costs)) = stack.pop() {
            if max_geodes(&robots, &resources, time_remaining) <= max {
                continue;
            }

            let mut time_to_spend =
                get_required_time(&mut robots, &mut resources, Resource::Ore, costs.ore_cost)
                    .unwrap();
            if let Some((resource, amount)) = costs.other_cost {
                if let Some(time_for_resource) =
                    get_required_time(&mut robots, &mut resources, resource, amount)
                {
                    time_to_spend = time_to_spend.max(time_for_resource);
                } else {
                    continue;
                }
            }

            if time_to_spend >= time_remaining {
                let geode_count = resources.get(&Resource::Geode).copied().unwrap_or_default();
                let geode_robots = robots.get(&Resource::Geode).copied().unwrap_or_default();
                max = max.max(geode_count + geode_robots * time_remaining);
                continue;
            }

            for (robot, count) in &robots {
                let entry = resources.entry(*robot).or_default();
                *entry += count * time_to_spend;
            }

            let entry = robots.entry(costs.robot).or_default();
            *entry += 1;

            let entry = resources.entry(Resource::Ore).or_default();
            *entry -= costs.ore_cost;
            if let Some((resource, amount)) = costs.other_cost {
                let entry = resources.entry(resource).or_default();
                *entry -= amount;
            }

            for costs in &blueprint.costs {
                stack.push((
                    robots.clone(),
                    resources.clone(),
                    time_remaining - time_to_spend,
                    costs,
                ))
            }
        }

        result.push(max);
    }

    result
}

fn part_one() -> i32 {
    let blueprints = parse_blueprints();

    let results = simulate(&blueprints, 24);

    blueprints
        .iter()
        .zip(results.iter())
        .fold(0, |acc, (blueprint, result)| acc + blueprint.id * result)
}

fn part_two() -> i32 {
    let mut blueprints = parse_blueprints();
    blueprints.truncate(3);

    let results = simulate(&blueprints, 32);

    results.into_iter().fold(1, |a, b| a * b)
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

use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;

type Graph<'a> = HashMap<&'a str, Vec<(&'a str, i32)>>;

fn parse_to_graph(input: &String) -> Graph {
    let mut output: Graph = HashMap::new();

    for line in input.lines() {
        let mut it = line.split_ascii_whitespace();

        let name = it.next().unwrap();

        it.next(); // consume 'would'

        let multiplier = if matches!(it.next(), Some("gain")) {
            1
        } else {
            -1
        };

        let amount = it.next().unwrap().parse::<i32>().unwrap() * multiplier;

        it.next(); // consume 'happiness'
        it.next(); // consume 'units'
        it.next(); // consume 'by'
        it.next(); // consume 'sitting'
        it.next(); // consume 'next'
        it.next(); // consume 'to'

        let neighbor = it.next().unwrap();
        let neighbor = &neighbor[0..neighbor.len() - 1]; // remove period

        let entry = output.entry(name).or_default();
        entry.push((neighbor, amount))
    }

    output
}

fn permute(size: usize) -> Vec<Vec<usize>> {
    let mut base_array: Vec<usize> = (0..size).into_iter().collect();
    let mut permutations = vec![base_array.clone()];
    let mut c = vec![0; size];
    let mut i = 0;

    while i < size {
        if c[i] < i {
            match i % 2 {
                0 => base_array.swap(0, i),
                _ => base_array.swap(c[i], i),
            }

            permutations.push(base_array.clone());
            c[i] += 1;
            i = 0;
        } else {
            c[i] = 0;
            i += 1;
        }
    }

    permutations
}

fn calculate_happiness(graph: &Graph, order: Vec<usize>, people: &Vec<&str>) -> Option<i32> {
    let mut total = 0;
    let len = order.len();
    for (first, second) in (0..len).zip(1..len + 1) {
        let a = people.get(*order.get(first)?)?;
        let b = people.get(*order.get(second % len)?)?;

        total += graph.get(a)?.iter().find(|(name, _)| name == b)?.1;
        total += graph.get(b)?.iter().find(|(name, _)| name == a)?.1;
    }

    Some(total)
}

fn find_optimal_happiness(graph: &Graph, input: Vec<Vec<usize>>) -> i32 {
    let people: Vec<&str> = graph.keys().map(|k| *k).collect();

    input
        .into_iter()
        .map(|order| calculate_happiness(graph, order, &people).unwrap())
        .max()
        .unwrap()
}

fn main() -> std::io::Result<()> {
    let file_path = env::current_dir()?.join(Path::new("data.txt"));
    let input = fs::read_to_string(file_path)?;
    let mut graph = parse_to_graph(&input);

    let possibilities = permute(graph.keys().len());

    println!("part 1: {}", find_optimal_happiness(&graph, possibilities));

    let people: Vec<&str> = graph.keys().map(|name| *name).collect();
    for person in &people {
        graph.entry(person).and_modify(|vec| vec.push(("me", 0)));
        graph.entry("me").or_default().push((person, 0));
    }

    let possibilities = permute(graph.keys().len());

    println!("part 2: {}", find_optimal_happiness(&graph, possibilities));

    Ok(())
}

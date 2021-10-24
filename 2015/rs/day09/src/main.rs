use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;

type Graph<'a> = HashMap<&'a str, Vec<(&'a str, usize)>>;

fn parse_into_graph(input: &String) -> Graph {
    let mut graph: Graph = HashMap::new();

    for line in input.lines() {
        let mut it = line.split_ascii_whitespace();
        let src = it.next().unwrap();
        it.next();
        let dest = it.next().unwrap();
        it.next();
        let dist = it.next().unwrap().parse().unwrap();

        let edges = graph.entry(src).or_default();
        edges.push((dest, dist));
        let edges = graph.entry(dest).or_default();
        edges.push((src, dist));
    }

    graph
}

fn permute<'a>(mut locations: Vec<&'a str>) -> Vec<Vec<&str>> {
    let mut permutations = vec![locations.clone()];
    let len = locations.len();
    let mut c = vec![0; len];
    let mut i = 0;

    while i < len {
        if c[i] < i {
            match i % 2 {
                0 => locations.swap(0, i),
                _ => locations.swap(c[i], i),
            }

            permutations.push(locations.clone());
            c[i] += 1;
            i = 0;
        } else {
            c[i] = 0;
            i += 1;
        }
    }

    permutations
}

fn walk(graph: &Graph, locations: Vec<&str>) -> usize {
    let mut distance = 0;

    for i in 0..locations.len() - 1 {
        let edges = graph.get(locations[i]).unwrap();
        distance += edges
            .iter()
            .find(|&&edge| edge.0 == locations[i + 1])
            .unwrap()
            .1;
    }

    distance
}

fn calculate_lengths(graph: Graph) -> Vec<usize> {
    let locations: Vec<&str> = graph.keys().map(|k| *k).collect();

    let permutations = permute(locations);

    let mut lengths = Vec::new();

    for perm in permutations {
        lengths.push(walk(&graph, perm));
    }

    lengths.sort();
    lengths
}

fn part_one(input: &Vec<usize>) -> usize {
    *input.first().unwrap()
}

fn part_two(input: &Vec<usize>) -> usize {
    *input.last().unwrap()
}

fn main() -> std::io::Result<()> {
    let file_path = env::current_dir()?.join(Path::new("data.txt"));
    let input = fs::read_to_string(file_path)?;
    let graph = parse_into_graph(&input);

    let lengths = calculate_lengths(graph);

    println!("part 1: {}", part_one(&lengths));
    println!("part 2: {}", part_two(&lengths));

    Ok(())
}

use std::{
    collections::HashMap, env, fs, iter::Peekable, path::Path, str::Chars, time::SystemTime,
};

type Node = (i32, i32);

type Graph = HashMap<Node, Vec<Node>>;

#[cfg(debug_assertions)]
fn print_graph(graph: &Graph) {
    let mut buffer = Vec::new();

    let mut min_y = i32::MAX;
    let mut min_x = i32::MAX;
    let mut max_y = i32::MIN;
    let mut max_x = i32::MIN;

    for &(x, y) in graph.keys() {
        max_x = x.max(max_x);
        min_x = x.min(min_x);
        max_y = y.max(max_y);
        min_y = y.min(min_y);
    }

    let x_offset = min_x.abs();
    max_x += x_offset;

    let y_range = max_y - min_y;

    while buffer.len() <= y_range as usize {
        buffer.push(Vec::new());
    }

    for row in &mut buffer {
        while row.len() <= max_x as usize {
            row.push('#');
        }
    }

    for (&(x, y), neighbors) in graph {
        let mut neighbor_left = false;
        let mut neighbor_right = false;
        let mut neighbor_up = false;
        let mut neighbor_down = false;

        for &neighbor in neighbors {
            neighbor_left = neighbor_left || ((x - 1, y) == neighbor);
            neighbor_right = neighbor_right || ((x + 1, y) == neighbor);
            neighbor_up = neighbor_up || ((x, y + 1) == neighbor);
            neighbor_down = neighbor_down || ((x, y - 1) == neighbor);
        }

        let ch = match (neighbor_left, neighbor_up, neighbor_right, neighbor_down) {
            (true, false, false, false) => '╡',
            (false, true, false, false) => '╨',
            (false, false, true, false) => '╞',
            (false, false, false, true) => '╥',
            (true, true, true, true) => '╬',
            (true, true, true, false) => '╩',
            (true, true, false, false) => '╝',
            (false, true, true, true) => '╠',
            (false, false, true, true) => '╔',
            (false, true, false, true) => '║',
            (true, false, true, false) => '═',
            (false, true, true, false) => '╚',
            (true, false, false, true) => '╗',
            (true, false, true, true) => '╦',
            (true, true, false, true) => '╣',
            (false, false, false, false) => unreachable!(),
        };

        buffer[(y - max_y).abs() as usize][(x + x_offset) as usize] = ch;
    }

    for row in buffer {
        for ch in row {
            print!("{}", ch);
        }
        println!();
    }
}

fn build_graph(chars: &mut Peekable<Chars>, current: Node, graph: &mut Graph) -> Node {
    let mut current = current;

    while let Some(ch) = chars.peek() {
        let next = match ch {
            'E' => (current.0 + 1, current.1),
            'W' => (current.0 - 1, current.1),
            'N' => (current.0, current.1 + 1),
            'S' => (current.0, current.1 - 1),
            '(' => {
                loop {
                    chars.next();
                    build_graph(chars, current, graph);

                    if !matches!(chars.peek(), Some('|')) {
                        break;
                    }
                }

                debug_assert!(matches!(chars.peek(), Some(')')));

                chars.next();
                continue;
            }
            '|' | ')' => return current,
            '$' => return current,
            _ => unreachable!(),
        };

        chars.next();

        debug_assert!(((current.1 - next.1).abs() + (current.0 - next.0).abs()) == 1);

        let entry = graph.entry(current).or_default();
        entry.push(next);
        let entry = graph.entry(next).or_default();
        entry.push(current);
        current = next;
    }

    current
}

fn walk_graph(graph: &Graph, distance: &mut HashMap<Node, i32>, current: Node, steps: i32) {
    let entry = distance.entry(current).or_default();
    if *entry <= steps {
        return;
    }
    *entry = steps;

    for neighbor in graph.get(&current).unwrap() {
        walk_graph(graph, distance, *neighbor, steps + 1);
    }
}

fn solve(input: &str) -> (i32, usize) {
    let mut graph: Graph = HashMap::new();
    let mut chars = input.chars().peekable();
    chars.next();

    build_graph(&mut chars, (0, 0), &mut graph);

    debug_assert!(matches!(chars.next(), Some('$')));

    let mut distances: HashMap<_, _> = graph.keys().map(|&key| (key, i32::MAX)).collect();

    #[cfg(debug_assertions)]
    print_graph(&graph);

    walk_graph(&graph, &mut distances, (0, 0), 0);

    (
        distances.values().cloned().max().unwrap(),
        distances.into_values().filter(|&v| v >= 1000).count(),
    )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!((3, 0), solve("^WNE$"));
        assert_eq!((10, 0), solve("^ENWWW(NEEE|SSE(EE|N))$"));
        assert_eq!((18, 0), solve("^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$"));
        assert_eq!(
            (23, 0),
            solve("^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$")
        );
        assert_eq!(
            (31, 0),
            solve("^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$")
        );
    }
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

    time_it(|| println!("parts (1, 2): {:?}", solve(&input)));

    Ok(())
}

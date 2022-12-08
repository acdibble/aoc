use std::time::SystemTime;

const DATA: &'static str = include_str!("../data.txt");

#[derive(Debug, Clone, Copy)]
struct Tree {
    height: i32,
    visible: bool,
}

fn height_to_i32(height: char) -> i32 {
    match height {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        _ => unreachable!(),
    }
}

impl Tree {
    fn new(height: char) -> Self {
        Tree {
            height: height_to_i32(height),
            visible: false,
        }
    }
}

fn rotate<T: Copy + Clone>(matrix: &mut Vec<Vec<T>>) {
    let clone = matrix.clone();

    for i in 0..matrix.len() {
        for j in 0..matrix.len() {
            matrix[i][j] = clone[j][i];
        }
    }

    for row in matrix {
        row.reverse();
    }
}

fn part_one() -> i32 {
    let mut forest: Vec<Vec<_>> = DATA
        .lines()
        .map(|l| l.chars().map(Tree::new).collect())
        .collect();

    for _ in 0..4 {
        for row in &mut forest {
            let mut current_height = -1;
            for tree in row {
                if tree.height > current_height {
                    tree.visible = true;
                    current_height = tree.height;
                }
            }
        }
        rotate(&mut forest);
    }

    forest
        .into_iter()
        .flat_map(|row| row.into_iter().map(|tree| if tree.visible { 1 } else { 0 }))
        .sum()
}

const DIRS: [[i32; 2]; 4] = [[0, 1], [0, -1], [1, 0], [-1, 0]];

fn part_two() -> i32 {
    let forest: Vec<Vec<_>> = DATA
        .lines()
        .map(|l| l.chars().map(height_to_i32).collect())
        .collect();

    let mut highest_score = 0;

    for (i, row) in forest.iter().enumerate() {
        for (j, tree) in row.iter().enumerate() {
            let mut scores = [0; 4];

            for (dir, [mod_x, mod_y]) in DIRS.iter().enumerate() {
                let mut x = j;
                let mut y = i;

                let mut view_distance = 0;
                loop {
                    x = (x as i32 + mod_x) as usize;
                    y = (y as i32 + mod_y) as usize;

                    if let Some(neighbor) = forest.get(y).and_then(|row| row.get(x)) {
                        view_distance += 1;
                        if neighbor >= tree {
                            break;
                        }
                    } else {
                        break;
                    }
                }

                scores[dir] = view_distance;
            }

            let [a, b, c, d] = scores;
            highest_score = highest_score.max(a * b * c * d);
        }
    }

    highest_score
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

const SLOPES: [(usize, usize); 5] = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

fn main() -> std::io::Result<()> {
    let file = std::fs::read_to_string("data.txt")?;

    let values: Vec<Vec<u8>> = file
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '#' => 1,
                    _ => 0,
                })
                .collect::<Vec<u8>>()
        })
        .collect();

    let mut product: i64 = 1;
    for (run, rise) in SLOPES.iter() {
        let mut total_trees: i32 = 0;
        let mut x: usize = 0;
        for vec in values.iter().step_by(*rise) {
            total_trees += vec[x] as i32;
            x = (x + run) % vec.len();
        }
        product *= total_trees as i64;
    }

    println!("{}", product);
    Ok(())
}

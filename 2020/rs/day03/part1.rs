const RUN: usize = 3;

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

    let mut total_trees: i32 = 0;
    let mut x: usize = 0;
    for vec in values {
        total_trees += vec[x] as i32;
        x = (x + RUN) % vec.len();
    }

    println!("{}", total_trees);
    Ok(())
}

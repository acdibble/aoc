fn main() -> std::io::Result<()> {
    let file = std::fs::read_to_string("data.txt")?;

    let mut total = 0;

    let regex =
        regex::Regex::new(r"(?P<min>\d+)-(?P<max>\d+) (?P<letter>\w): (?P<password>\w+)").unwrap();

    for caps in regex.captures_iter(&file) {
        let min: i32 = caps["min"].parse().unwrap();
        let max: i32 = caps["max"].parse().unwrap();
        let letter = caps["letter"].chars().nth(0).unwrap();
        let password = &caps["password"];

        let count: i32 = password
            .chars()
            .fold(0, |acc, ch| acc + (ch == letter) as i32);

        total += (count <= max && count >= min) as i32;
    }

    println!("total: {}", total);

    Ok(())
}

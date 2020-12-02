fn main() {
    let file = std::fs::read_to_string("data.txt").unwrap();

    let mut total = 0;

    let regex =
        regex::Regex::new(r"(?P<min>\d+)-(?P<max>\d+) (?P<letter>\w): (?P<password>\w+)").unwrap();

    for caps in regex.captures_iter(&file) {
        let min: usize = caps["min"].parse().unwrap();
        let max: usize = caps["max"].parse().unwrap();
        let letter = caps["letter"].chars().nth(0).unwrap();
        let password = &caps["password"];

        let is_min_letter = password.chars().nth(min - 1).unwrap() == letter;
        let is_max_letter = password.chars().nth(max - 1).unwrap() == letter;

        total += (is_min_letter ^ is_max_letter) as i32
    }

    println!("total: {}", total);
}

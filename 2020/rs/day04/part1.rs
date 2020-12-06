fn main() -> std::io::Result<()> {
    let file = std::fs::read_to_string("data.txt")?;

    let passports: Vec<String> = file.split("\n\n").map(|s| s.to_string()).collect();

    let mut valid_passports = 0;

    for passport in passports {
        let elements: Vec<Vec<_>> = passport
            .split_ascii_whitespace()
            .map(|s| s.split(":").collect())
            .collect();

        let mut fields_found = 0;
        for element in elements {
            fields_found += match element[0] {
                "byr" => 1,
                "iyr" => 1,
                "eyr" => 1,
                "hgt" => 1,
                "hcl" => 1,
                "ecl" => 1,
                "pid" => 1,
                _ => 0,
            }
        }

        valid_passports += (fields_found == 7) as i32;
    }

    println!("{}", valid_passports);

    Ok(())
}

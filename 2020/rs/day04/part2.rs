fn is_in_range(value: &str, lower: i32, upper: i32) -> bool {
    match value.trim_end_matches(char::is_alphabetic).parse() {
        Ok(int_val) => lower <= int_val && int_val <= upper,
        _ => false,
    }
}

fn validate_hgt(value: &str) -> bool {
    if value.ends_with("in") {
        is_in_range(value, 59, 76)
    } else if value.ends_with("cm") {
        is_in_range(value, 150, 193)
    } else {
        false
    }
}

fn validate_hcl(value: &str) -> bool {
    let without_hash = value.trim_start_matches("#");

    if without_hash == value {
        return false;
    }

    match i32::from_str_radix(without_hash, 16) {
        Ok(_) => true,
        _ => false,
    }
}

fn validate_pid(value: &str) -> bool {
    value.len() == 9 && value.bytes().all(|c| (b'0'..=b'9').contains(&c))
}

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
            let value = element[1];
            fields_found += match element[0] {
                // byr (Birth Year) - four digits; at least 1920 and at most 2002.
                "byr" => is_in_range(value, 1920, 2002),
                // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
                "iyr" => is_in_range(value, 2010, 2020),
                // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
                "eyr" => is_in_range(value, 2020, 2030),
                // hgt (Height) - a number followed by either cm or in:
                // If cm, the number must be at least 150 and at most 193.
                // If in, the number must be at least 59 and at most 76.
                "hgt" => validate_hgt(value),
                // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
                "hcl" => validate_hcl(value),
                // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
                "ecl" => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&value),
                // pid (Passport ID) - a nine-digit number, including leading zeroes.
                "pid" => validate_pid(value),
                "cid" => false,
                _ => unreachable!(),
            } as i32;
        }

        valid_passports += (fields_found == 7) as i32;
    }

    println!("{}", valid_passports);

    Ok(())
}

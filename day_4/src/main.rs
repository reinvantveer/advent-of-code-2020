use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let passports: Vec<&str> = input
        // Assuming Unix-type line-feed line endings here!
        .split(&"\n\n".to_string())
        .collect();

    println!("{} passports found", passports.len());

    let mut valid_keys_passports = 0;
    for passport in &passports {
        let passport_joined = passport
            .replace("\n", " "); // Get rid of newlines

        let passport_items: Vec<&str> = passport_joined
            .split(" ")
            .collect();  // convert to iterator over passport keys

        if is_valid_passport_by_keys(&passport_items) {
            valid_keys_passports += 1;
        }
    }

    println!("{} passports with correct number of keys", valid_keys_passports);

    let mut valid_values_passports = 0;

    for passport in &passports {
        let passport_joined = passport
            .replace("\n", " "); // Get rid of newlines

        let passport_items: Vec<&str> = passport_joined
            .split(" ")
            .collect();  // convert to iterator over passport keys

        if is_valid_passport_by_keys(&passport_items)
            && is_valid_passport_by_properties(&passport_items) {
            valid_values_passports += 1;
            println!("{} is valid by keys and values\n", passport);
        }
    }
    println!("{} passports valid by correct fields and values", valid_values_passports);
}

fn is_valid_passport_by_keys(passport_items: &Vec<&str>) -> bool {
    let mut keys = Vec::new();

    for item in passport_items {
        let mut propsplit = item.split(":");
        let (key, _val): (&str, &str) = (
            propsplit.next().unwrap(),
            propsplit.next().unwrap(),
        );
        // println!("Key {}, val {}", key, val);

        if key != "cid" && !keys.contains(&key) {
            keys.push(key);
        }
    }

    // Return true only if all 7 required keys are present
    keys.len() == 7
}

fn is_valid_passport_by_properties(passport_items: &Vec<&str>) -> bool {
    for item in passport_items {
        let mut propsplit = item.split(":");
        let (key, val): (&str, &str) = (
            propsplit.next().unwrap(),
            propsplit.next().unwrap(),
        );

        let valid_value = match key {
            // (Birth Year) - four digits; at least 1920 and at most 2002
            "byr" => is_valid_year(val, 1920, 2002),
            // (Issue Year) - four digits; at least 2010 and at most 2020
            "iyr" => is_valid_year(val, 2010, 2020),
            // (Expiration Year) - four digits; at least 2020 and at most 2030
            "eyr" => is_valid_year(val, 2020, 2030),
            "hgt" => is_valid_height(val),
            "hcl" => is_valid_hair_color(val),
            "ecl" => is_valid_eye_color(val),
            "pid" => is_valid_passport_id(val),
            // (Country ID) - ignored, missing or not.
            "cid" => true,
            _ => false
        };

        if !valid_value {
            return false
        }
    }

    true
}

fn is_valid_year(year: &str, min: usize, max: usize) -> bool {
    if year.len() <= 3 {
        return false
    }

    let year_number = year.parse::<usize>().unwrap();
    year_number >= min && year_number <= max
}

fn is_valid_height(height: &str) -> bool {
    // (Height) - a number followed by either cm or in:
    // If cm, the number must be at least 150 and at most 193.
    // If in, the number must be at least 59 and at most 76.

    // There should be at least one digit and two chars for the unit of height
    if height.len() < 3 {
        return false
    }

    let units_start = height.len() - 2;
    let height_str_end = height.len();
    let units = &height[units_start..height_str_end];
    let amount: usize = height[0..units_start].parse().unwrap();

    // println!("{}", units);

    match units {
        // If cm, the number must be at least 150 and at most 193.
        "cm" => amount >= 150 && amount <= 193,
        // If in, the number must be at least 59 and at most 76.
        "in" => amount >= 59 && amount <= 76,
        _ => false
    }
}

fn is_valid_hair_color(color: &str) -> bool {
    // a # followed by exactly six characters 0-9 or a-f
    if !color.starts_with("#") || !color.len() == 7 {
        return false
    }

    let color_parse_result = usize::from_str_radix(&color[1..color.len()], 16);
    match color_parse_result {
        Ok(_t) => true,
        Err(_e) => false
    }
}

fn is_valid_eye_color(color: &str) -> bool {
    // (Eye Color) - exactly one of: amb blu brn gry grn hzl oth

    match color {
        "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
        _ => false
    }
}

fn is_valid_passport_id(identifier: &str) -> bool {
    // (Passport ID) - a nine-digit number, including leading zeroes.

    if identifier.len() != 9 {
        return false
    }

    let number_result = identifier.parse::<usize>();
    match number_result {
        Ok(_t) => true,
        Err(_e) => false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let some_string = "aa\n\ncc".to_string();
        let splits: Vec<&str> = some_string.split("\n\n").collect();
        assert_eq!(splits, vec!["aa", "cc"]);
    }

    // With some help of https://github.com/Lakret/aoc2020/blob/master/src/d4.rs#L168
    #[test]
    fn validators_work() {
        assert!(is_valid_year("2002", 1920, 2002));

        assert!(!is_valid_year("2003", 1920, 2002));

        assert!(is_valid_height("60in"));
        assert!(is_valid_height("190cm"));

        assert!(!is_valid_height("190in"));
        assert!(!is_valid_height("190"));

        assert!(is_valid_hair_color("#123abc"));

        assert!(!is_valid_hair_color("#123abz"));
        assert!(!is_valid_hair_color("123abc"));

        assert!(is_valid_eye_color("brn"));
        assert!(!is_valid_eye_color("wat"));

        assert!(is_valid_passport_id("000000001"));
        assert!(!is_valid_passport_id("0123456789"));
    }
}
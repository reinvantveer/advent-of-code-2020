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
        if is_valid_passport_by_keys(passport) {
            valid_keys_passports += 1;
        }
    }

    println!("{} passports with correct number of keys", valid_keys_passports);

    let mut valid_values_passports = 0;

    for passport in &passports {
        if is_valid_passport_by_properties(passport) {
            valid_values_passports += 1;
        }
    }
    println!("{} passports valid by correct values", valid_values_passports);
}

fn is_valid_passport_by_keys(passport: &&str) -> bool {
    let passport_joined = passport
        .replace("\n", " "); // Get rid of newlines

    let passport_props: Vec<&str> = passport_joined
        .split(" ")
        .collect();  // convert to iterator over passport keys

    // println!("{} properties in passport {}", passport_props.len(), passport_props.join(" "));

    let mut keys = Vec::new();

    for props in passport_props {
        let mut propsplit = props.split(":");
        let (key, _val): (&str, &str) = (
            propsplit.next().unwrap(),
            propsplit.next().unwrap(),
        );
        // println!("Key {}, val {}", key, val);

        if key != "cid" {
            keys.push(key);
        }
    }

    // Return true only if all 7 required keys are present
    keys.len() == 7
}

fn is_valid_passport_by_properties(passport: &&str) -> bool {
    let passport_joined = passport
        .replace("\n", " "); // Get rid of newlines

    let passport_props: Vec<&str> = passport_joined
        .split(" ")
        .collect();  // convert to iterator over passport keys

    // println!("{} properties in passport {}", passport_props.len(), passport_props.join(" "));

    for props in passport_props {
        let mut propsplit = props.split(":");
        let (key, val): (&str, &str) = (
            propsplit.next().unwrap(),
            propsplit.next().unwrap(),
        );
        let valid_value = match key {
            "byr" => val.parse::<usize>().unwrap() >= 1920 && val.parse::<usize>().unwrap() <= 2002,
            "iyr" => val.parse::<usize>().unwrap() >= 2010 && val.parse::<usize>().unwrap() <= 2020,
            "eyr" => val.parse::<usize>().unwrap() >= 2020 && val.parse::<usize>().unwrap() <= 2030,
            "hgt" => is_valid_height(val),
            "hcl" => is_valid_hair_color(val),

            _ => true
        };
        if !valid_value { return false };
    }

    true
}

fn is_valid_height(height: &str) -> bool {
    // There should be at least one digit and two chars for the unit for the height
    if height.len() <= 3 {
        return false;
    }

    let units_start = height.len() - 2;
    let height_str_end = height.len();
    let units = &height[units_start..height_str_end];
    let amount: usize = height[0..units_start].parse().unwrap();

    // println!("{}", units);

    match units {
        "cm" => amount >= 150 && amount <= 193,
        "in" => amount >= 59 && amount <= 76,
        _ => false
    }
}

fn is_valid_hair_color(color: &str) -> bool {
    // Hair color should start with a pound sign
    if !color.starts_with("#") {
        return false;
    }

    // Hair color string should consist of 7 chars
    if !color.len() == 7 {
        return false;
    }

    // TODO: all chars from [1..8] should be in hex

    true
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_add() {
        let some_string = "aa\n\ncc".to_string();
        let splits: Vec<&str> = some_string.split("\n\n").collect();
        assert_eq!(splits, vec!["aa", "cc"]);
    }
}
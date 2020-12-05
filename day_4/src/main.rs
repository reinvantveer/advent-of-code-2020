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
            _ => false
        };
        if !valid_value { return false };
    }

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
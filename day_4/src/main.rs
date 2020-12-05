use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let passports: Vec<&str> = input
        // Assuming Unix-type line-feed line endings here!
        .split(&"\n\n".to_string())
        .collect();

    println!("{} passports found", passports.len());

    for passport in &passports {
        validate_passport(passport);
        break;
    }
}

fn validate_passport(passport: &&str) {
    let passport_joined = passport
        .replace("\n", " "); // Get rid of newlines

    let passport_keys: Vec<&str> = passport_joined
        .split(" ")
        .collect();  // convert to iterator over passport keys

    println!("{} keys in passport {}", passport_keys.len(), passport_keys.join(" "));
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
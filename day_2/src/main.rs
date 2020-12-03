use std::fs::File;
use std::io::Read;

fn main() {
    let mut input_file = File::open("input.txt").unwrap();
    let mut contents = String::new();
    input_file.read_to_string(&mut contents).unwrap();

    let entries: Vec<(String, String, String)> = contents
        .split("\n")
        .filter(|entry| entry.to_string() != "")
        .map(|entry| entry_to_tuple(entry))
        .collect();

    let mut failed = 0;
    let mut succeeded = 0;

    occurrence_validation(entries.clone(), &mut failed, &mut succeeded);
    println!("There were {} succeeded occurrence-valid entries and {} failed", succeeded, failed);

    failed = 0;
    succeeded = 0;
    position_validation(entries, &mut failed, &mut succeeded);
    println!("There were {} succeeded position-valid entries and {} failed", succeeded, failed);
}

fn occurrence_validation(entries: Vec<(String, String, String)>, failed: &mut i32, succeeded: &mut i32) {
    for entry in entries {
        let occurrence_range: Vec<usize> = entry.0
            .split("-")
            .map(|min_or_max| min_or_max.parse().unwrap())
            .collect();

        let minimum = occurrence_range[0];
        let maximum = occurrence_range[1];

        let mut chars = entry.1.chars();
        let char_to_check: String = chars.next().unwrap().to_string();  // First character in second entry
        let char_occurrences: Vec<char> = entry.2
            .chars()
            .filter(|character| character.to_string() == char_to_check)
            .collect();

        if char_occurrences.len() < minimum || char_occurrences.len() > maximum {
            // println!("{} has less than {} required {}'s: {}",
            //          &entry.2, minimum, &char_to_check, char_occurrences.len())
            *failed += 1;
        } else {
            *succeeded += 1;
        }
    }
}

fn position_validation(entries: Vec<(String, String, String)>, failed: &mut i32, succeeded: &mut i32) {
    for entry in entries {
        let positions: Vec<usize> = entry.0
            .split("-")
            .map(|min_or_max| min_or_max.parse().unwrap())
            .collect();

        let index_1: usize = positions[0] - 1;  // We need to subtract one to make it a 0-indexed idx
        let index_2: usize = positions[1] - 1;

        let char_to_check = entry.1
            .chars()
            .next()
            .unwrap(); // First character in second entry

        let chars: Vec<char> = entry.2.chars().collect();

        if (chars[index_1] == char_to_check) ^ (chars[index_2] == char_to_check) {
            // println!("{} has less than {} required {}'s: {}",
            //          &entry.2, minimum, &char_to_check, char_occurrences.len())
            *succeeded += 1;
        } else {
            *failed += 1;
        }
    }
}

fn entry_to_tuple(entry: &str) -> (String, String, String) {
    let mut entry_parts = entry
        .split(" ")
        .map(|part| part.to_string());

    let parts: (String, String, String) = (
        entry_parts.next().unwrap(),
        entry_parts.next().unwrap(),
        entry_parts.next().unwrap(),
    );
    parts
}
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

        if char_occurrences.len() < minimum {
            println!("{} has less than {} required {}'s: {}",
                     &entry.2, minimum, &char_to_check, char_occurrences.len())
        }
    }

    println!("Hello, world!");
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
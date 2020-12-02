use std::fs::File;
use std::io::Read;


fn main() {
    let flag: usize = 2020;

    let mut input_file = File::open("input.txt").unwrap();
    let mut contents = String::new();
    input_file.read_to_string(&mut contents).unwrap();

    let entries: Vec<usize> = contents
        .split("\n")
        .map(|entry| entry.parse::<usize>().unwrap())
        .collect();

    for entry_idx in 1..entries.len() {
        for inner_entry_idx in 1..entries.len() {
            if entries[entry_idx] + entries[inner_entry_idx] == flag {
                println!("Flag found: {}", entries[entry_idx] * entries[inner_entry_idx]);
                println!("Entries {} {}", entries[entry_idx], entries[inner_entry_idx]);
            }
        }
    }
}

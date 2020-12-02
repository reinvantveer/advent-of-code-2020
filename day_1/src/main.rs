use std::fs::File;
use std::io::Read;


fn main() {
    let flag_hint: usize = 2020;

    let mut input_file = File::open("input.txt").unwrap();
    let mut contents = String::new();
    input_file.read_to_string(&mut contents).unwrap();

    let entries: Vec<usize> = contents
        .split("\n")
        .map(|entry| entry.parse::<usize>().unwrap())
        .collect();

    double_entry_flag(flag_hint, entries.clone());
    triple_entry_flag(flag_hint, entries)
}

fn double_entry_flag(flag_hint: usize, entries: Vec<usize>) {
    for outer_entry_idx in 1..entries.len() {
        for inner_entry_idx in outer_entry_idx..entries.len() {
            if entries[outer_entry_idx] + entries[inner_entry_idx] == flag_hint {
                println!("Double entry flag found: {}", entries[outer_entry_idx] * entries[inner_entry_idx]);
                println!("Entries {} {}", entries[outer_entry_idx], entries[inner_entry_idx]);
            }
        }
    }
}

fn triple_entry_flag(flag_hint: usize, entries: Vec<usize>) {
    for first_entry_idx in 1..entries.len() {
        for second_entry_idx in first_entry_idx..entries.len() {
            for third_entry_idx in second_entry_idx..entries.len() {
                if entries[first_entry_idx]
                    + entries[second_entry_idx]
                    + entries[third_entry_idx] == flag_hint {
                        println!("Triple entry flag found: {}", entries[first_entry_idx] * entries[second_entry_idx] * entries[third_entry_idx]);
                        println!("Entries {} {} {}", entries[first_entry_idx], entries[second_entry_idx], entries[third_entry_idx]);
                    }
            }
        }
    }
}

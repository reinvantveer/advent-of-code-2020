use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let lines: Vec<&str> = input
        .lines()
        .collect();

    println!("Hello, world!");
}

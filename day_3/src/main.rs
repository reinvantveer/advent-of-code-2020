use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let lines: Vec<&str> = input
        .lines()
        .collect();

    let mut column = 0;
    let mut encountered_trees = 0;

    for line in &lines {
        if column >= line.len() {
            println!("{}", column);
            column -= line.len();
        }

        println!("{}", line);
        let line_chars: Vec<char> = line.chars().collect();
        if line_chars[column] == "#".chars().next().unwrap() {
            encountered_trees += 1;
            println!("{}^ Tree at column {}", " ".repeat(column), column);
        }
        column += 3;
    }
    println!("Encountered {} trees", encountered_trees);
}

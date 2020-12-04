use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let lines: Vec<&str> = input
        .lines()
        .collect();

    let mut column_increment = 1;
    let mut line_increment = 1;
    let mut tree_multiples = slide_down(&lines, column_increment, line_increment);

    column_increment = 3;
    tree_multiples *= slide_down(&lines, column_increment, line_increment);

    column_increment = 5;
    tree_multiples *= slide_down(&lines, column_increment, line_increment);

    column_increment = 7;
    tree_multiples *= slide_down(&lines, column_increment, line_increment);

    column_increment = 1;
    line_increment = 2;
    tree_multiples *= slide_down(&lines, column_increment, line_increment);

    println!("{}", tree_multiples);
}

fn slide_down(
    lines: &Vec<&str>,
    right_increment: usize,
    down_increment: usize,
    ) -> usize {

    let mut column = 0;
    let mut encountered_trees = 0;

    for line in lines.iter().step_by(down_increment) {
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
        column += right_increment;
    }
    println!("Encountered {} trees", encountered_trees);
    encountered_trees
}

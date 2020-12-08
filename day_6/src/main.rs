use std::fs;

fn main() {
    read_lines("input.txt");

    println!("Hello, world!");
}

fn read_lines(path: &str) -> Vec<String> {
    let input = fs::read_to_string(path)
        .unwrap();

    let lines: Vec<String> = input.lines().map(String::from).collect();
    lines
}

fn get_group_answers(lines: Vec<String>) -> Vec<String> {
    let mut grouped: Vec<String> = Vec::new();

    for line in lines.clone() {
        let mut one_group: Vec<String> = Vec::new();
        if line == "" {
            grouped.append(&mut one_group);
        } else {
            one_group.append(&mut vec![line.clone()]);
        }
    }
    grouped
}

#[cfg(test)]
mod test {
    use crate::{read_lines, get_group_answers};

    #[test]
    fn figure_out_correct_example_answer() {
        let lines = read_lines("example.txt");
        let group_answers = get_group_answers(lines);
        assert_eq!(group_answers[0], "abc");
        assert_eq!(group_answers[1], "abc")
    }
}
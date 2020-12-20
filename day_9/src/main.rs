use std::fs;

fn main() {
    println!("Hello, world!");
}

fn read_lines(path: &str) -> Vec<String> {
    let input = fs::read_to_string(path)
        .unwrap();

    input
        .lines()
        .map(String::from)
        .collect()
}

fn lines_to_numbers(lines: &Vec<String>) -> Vec<usize> {
    lines
        .iter()
        .map(|line| line.parse().unwrap())
        .collect()
}

fn find_first_wrong_number(numbers: Vec<usize>) -> usize {
    5
}

#[cfg(test)]
mod test {
    use crate::{read_lines, lines_to_numbers, find_first_wrong_number};

    #[test]
    fn test_first_wrong_number() {
        let lines = read_lines("example.txt");
        let numbers = lines_to_numbers(&lines);
        let wrong = find_first_wrong_number(numbers);
        assert_eq!(wrong, 127)
    }
}
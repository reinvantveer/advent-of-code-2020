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

fn get_device_rating(ratings: &Vec<usize>) -> usize {
    ratings.iter().max().unwrap() + 3
}

fn get_adapter_chain(ratings: &mut Vec<usize>) -> Vec<usize> {
    let ordered = *ratings.sort();
    ordered
}

fn get_joltage_differences(ratings: &Vec<usize>) -> (usize, usize) {
    (0, 0)
}

#[cfg(test)]
mod test {
    use crate::{read_lines, lines_to_numbers, get_device_rating, get_joltage_differences};

    #[test]
    fn test_simple_device_joltage_rating() {
        let lines = read_lines("example1_1.txt");
        let ratings = lines_to_numbers(&lines);
        let device_rating = get_device_rating(&ratings);

        assert_eq!(device_rating, 22);
    }

    #[test]
    fn test_adapter_chain() {
        let lines = read_lines("example1_1.txt");
        let ratings = lines_to_numbers(&lines);

    }

    #[test]
    fn test_jolt_differences() {
        let lines = read_lines("example1_1.txt");
        let ratings = lines_to_numbers(&lines);

        let (diffs_1_jolt, diffs_3_jolt) = get_joltage_differences(&ratings);
        assert_eq!(diffs_1_jolt, 7);
        assert_eq!(diffs_3_jolt, 5);
    }

    #[test]
    fn test_slightly_less_simple_joltage_rating() {
        let lines = read_lines("example1_2.txt");
        let ratings = lines_to_numbers(&lines);
        let device_rating = get_device_rating(&ratings);

        assert_eq!(device_rating, 22);
    }


}
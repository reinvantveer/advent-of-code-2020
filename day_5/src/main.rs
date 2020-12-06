use std::fs;

fn main() {
    let lines = read_lines("input.txt");

    println!("Hello, world!");
}

fn read_lines(path: &str) -> Vec<String> {
    let input = fs::read_to_string(path)
        .unwrap();

    let lines: Vec<String> = input.lines().map(String::from).collect();
    lines
}

fn parse_seat_id(seat: String) -> usize {
    for (idx, element) in seat.chars().enumerate() {

    }
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn read_works() {
        let lines = read_lines("example.txt");
        assert_eq!(lines.len(), 3);
    }

    #[test]
    fn correct_seat_id(){
        assert_eq!(parse_seat_id("BFFFBBFRRR".to_string()), 567)
    }

}
use std::fs;

fn main() {
    println!("Hello, world!");
}

struct Instruction {
    operation: String,
    amount: isize
}

fn read_lines(path: &str) -> Vec<String> {
    let input = fs::read_to_string(path)
        .unwrap();

    input
        .lines()
        .map(String::from)
        .collect()
}

fn parse_instructions(lines: &Vec<String>) -> Vec<Instruction> {
    lines
        .iter()
        .map(parse_instruction)
        .collect()
}

fn parse_instruction(line: &String) -> Instruction {
    let op_parts: Vec<_> = line.split(" ").collect();
    Instruction {
        operation: op_parts[0].to_string(),
        amount: op_parts[1].parse().unwrap()
    }
}


#[cfg(test)]
mod test {
    use crate::{read_lines, parse_instructions};

    #[test]
    fn test_example_code() {
        let lines = read_lines("example.txt");
        let instructions = parse_instructions(&lines);

        assert_eq!(instructions[0].operation, "nop");
        assert_eq!(instructions[5].amount, -99);
    }
}
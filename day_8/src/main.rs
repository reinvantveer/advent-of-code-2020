use std::fs;
use std::collections::HashSet;

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

fn run_code_until_already_executed(instructions: Vec<Instruction>) -> isize {
    let mut already_executed = HashSet::new();
    let mut program_counter = 0;
    let mut accumulator = 0 as isize;

    loop {
        if already_executed.contains(&program_counter) {
            println!("End of loop detected: {} already visited", &program_counter);
            break;
        }
        let instruction = &instructions[program_counter];
        already_executed.insert(program_counter);
        program_counter += 1;
    }

    accumulator
}

#[cfg(test)]
mod test {
    use crate::{read_lines, parse_instructions, run_code_until_already_executed};

    #[test]
    fn test_example_code_parser() {
        let lines = read_lines("example.txt");
        let instructions = parse_instructions(&lines);

        assert_eq!(instructions[0].operation, "nop");
        assert_eq!(instructions[5].amount, -99);
    }

    #[test]
    fn test_once_execution() {
        let lines = read_lines("example.txt");
        let instructions = parse_instructions(&lines);

        let accumulator_value = run_code_until_already_executed(instructions);
        assert_eq!(accumulator_value, 5)
    }

}
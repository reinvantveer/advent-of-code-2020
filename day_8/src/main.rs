use std::fs;
use std::collections::HashSet;

fn main() {
    let lines = read_lines("input.txt");
    let instructions = parse_instructions(&lines);
    let (acc, _) = run_code_until_already_executed(instructions);

    println!("Accumulator after broken loop: {}", acc);

    let instructions = parse_instructions(&lines);

    let acc = run_to_completion(&lines, instructions).unwrap();

    println!("Accumulator: {}", acc);
}

fn run_to_completion(lines: &Vec<String>, instructions: Vec<Instruction>) -> Option<isize> {
    for (idx, instruction) in instructions.iter().enumerate() {
        // No need to tinker if it's not either a jmp or nop instruction
        if !["jmp", "nop"].contains(&&*instruction.operation) { continue; };

        let mut new_instructions = parse_instructions(&lines);  // Reset
        let swapped = match instruction.operation.as_str() {
            "jmp" => "nop",  // Switch instruction type
            "nop" => "jmp",
            _ => unreachable!()
        }.to_string();

        assert_ne!(swapped, instruction.operation);
        new_instructions[idx].operation = swapped;

        let (acc, terminated_normally) = run_code_until_already_executed(new_instructions);
        if terminated_normally {
            println!("Program terminated successfully, bug fixed! Accumulator: {}", acc);
            return Some(acc);
        }
    }
    return None;
}

#[derive(Clone)]
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

fn run_code_until_already_executed(instructions: Vec<Instruction>) -> (isize, bool) {
    let mut already_executed = HashSet::new();
    let mut program_counter = 0 as isize;
    let mut accumulator = 0 as isize;

    loop {
        if already_executed.contains(&program_counter) {
            println!("Infinite loop detected: {} already visited", &program_counter);
            println!("Program terminated after re-visited instruction. Accumulator: {}", accumulator);
            return (accumulator, false);
        }

        // Update the set of already visited instructions with the current one
        already_executed.insert(program_counter);

        let instruction = &instructions[program_counter as usize];

        match instruction.operation.as_str() {
            "nop" => (),
            "acc" => accumulator += instruction.amount,
            "jmp" => program_counter += instruction.amount - 1, // We auto-increment the pc after
            _ => unreachable!(format!("Unknown opcode {}", instruction.operation))
        }

        program_counter += 1;

        if program_counter as usize == instructions.len() {
            println!("Program terminated normally with accumulator {}", accumulator);
            return (accumulator, true)
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{read_lines, parse_instructions, run_code_until_already_executed, run_to_completion};

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

        let (accumulator_value, normal_termination) = run_code_until_already_executed(instructions);
        assert_eq!(accumulator_value, 5);
        assert_eq!(normal_termination, false)
    }

    #[test]
    fn test_run_to_completion() {
        let lines = read_lines("example.txt");
        let instructions = parse_instructions(&lines);

        let accumulator_value = run_to_completion(&lines, instructions).unwrap();
        assert_eq!(accumulator_value, 8)
    }
}
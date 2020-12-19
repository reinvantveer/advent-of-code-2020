use std::fs;
use std::collections::HashSet;

fn main() {
    let lines = read_lines("input.txt");
    let instructions = parse_instructions(&lines);
    let (acc, _) = run_code_until_already_executed(instructions);

    println!("Accumulator after broken loop: {}", acc);

    let instructions = parse_instructions(&lines);

    for (idx, instruction) in instructions.iter().enumerate() {
        if instruction.operation != "jmp" || instruction.operation != "nop" { continue };

        let mut new_instructions = parse_instructions(&lines);  // Reset
        new_instructions[idx].operation = match instruction.operation.as_str() {
            "jmp" => "nop",  // Switch instruction type
            "nop" => "jmp",
            _ => unreachable!()
        }.to_string();

        let (acc, terminated_normally) = run_code_until_already_executed(new_instructions);
        if terminated_normally {
            println!("Program terminated successfully, bug fixed! Accumulator: {}", acc);
            return;
        }
    }

    println!("Accumulator: {}", acc);
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
            println!("End of loop detected: {} already visited", &program_counter);
            break;
        }
        let instruction = &instructions[program_counter as usize];

        match instruction.operation.as_str() {
            "nop" => (),
            "acc" => accumulator += instruction.amount,
            "jmp" => program_counter += instruction.amount - 1, // We auto-increment the pc after
            _ => unreachable!(format!("Unknown opcode {}", instruction.operation))
        }

        already_executed.insert(program_counter);
        program_counter += 1;

        if program_counter as usize == instructions.len() {
            println!("Program terminated normally with accumulator {}", accumulator);
            return (accumulator, true)
        }
    }

    println!("Program terminated after re-visited instruction. Accumulator: {}", accumulator);
    (accumulator, false)
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
use std::{num::ParseIntError, path::PathBuf, str::FromStr};


use crate::utils::read_lines;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum Instruction {
    Noop,
    AddX(isize),
}

#[derive(Debug, Clone)]
enum InstructionError {
    UnknownInstruction(String),
    IntegerParseError,
}

impl From<ParseIntError> for InstructionError {
    fn from(_: ParseIntError) -> Self {
        Self::IntegerParseError
    }
}

impl FromStr for Instruction {
    type Err = InstructionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "noop" {
            Ok(Self::Noop)
        } else if s.starts_with("addx") {
            let val = s.trim_start_matches("addx ").parse::<isize>()?;
            Ok(Self::AddX(val))
        } else {
            Err(InstructionError::UnknownInstruction(format!(
                "Unknown input {}",
                s
            )))
        }
    }
}

fn run_instruction(state: &mut Vec<isize>, instruction: &Instruction) {
    let current_value: isize = *state.last().unwrap_or(&(-1));
    match instruction {
        Instruction::Noop => state.push(current_value),
        Instruction::AddX(adder) => {
            state.push(current_value);
            state.push(current_value + adder);
        }
    }
}

fn signal_strength(state: &Vec<isize>, index: usize) -> isize {
    let value = *state.get(index - 1).unwrap_or(&1);
    index as isize * value
}

fn calculate_signal_strength(state: &Vec<isize>, cycles: &Vec<usize>) -> isize {
    cycles.iter().map(|i| signal_strength(state, *i)).sum()
}

fn draw_screen(state: &Vec<isize>) -> String{
    let mut output = String::with_capacity(40 * 6 + 5);
    for index in 1..241 {

        if index != 1 && index % 40 == 1 {
            output.push_str("\n");
        }
        if let Some(register) = state.get(index - 1) {
            let sprite_position = register + 1 % 40;
            let column: isize = (index % 40) as isize;
            if (sprite_position - column).abs() <= 1 {
                output.push_str("#");
            } else {
                output.push_str(".");
            }
        } else {
            output.push_str(".");
        }
    }
    output

}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test_example_case() {
        let mut states = vec![1];
        run_instruction(&mut states, &Instruction::AddX(15));
        run_instruction(&mut states, &Instruction::AddX(-11));
        run_instruction(&mut states, &Instruction::AddX(6));
        run_instruction(&mut states, &Instruction::AddX(-3));
        run_instruction(&mut states, &Instruction::AddX(5));
        run_instruction(&mut states, &Instruction::AddX(-1));
        run_instruction(&mut states, &Instruction::AddX(-8));
        run_instruction(&mut states, &Instruction::AddX(13));
        run_instruction(&mut states, &Instruction::AddX(4));
        run_instruction(&mut states, &Instruction::Noop);
        assert_eq!(420, calculate_signal_strength(&states, &vec![20]));
    }

    #[test]
    pub fn test_crt() {
        let mut states = vec![1];
        run_instruction(&mut states, &Instruction::AddX(15));
        run_instruction(&mut states, &Instruction::AddX(-11));
        run_instruction(&mut states, &Instruction::AddX(6));
        run_instruction(&mut states, &Instruction::AddX(-3));
        run_instruction(&mut states, &Instruction::AddX(5));
        run_instruction(&mut states, &Instruction::AddX(-1));
        run_instruction(&mut states, &Instruction::AddX(-8));
        run_instruction(&mut states, &Instruction::AddX(13));
        run_instruction(&mut states, &Instruction::AddX(4));
        run_instruction(&mut states, &Instruction::Noop);
        run_instruction(&mut states, &Instruction::AddX(-1));
        run_instruction(&mut states, &Instruction::AddX(5));
        run_instruction(&mut states, &Instruction::AddX(-1));
        run_instruction(&mut states, &Instruction::AddX(5));
        run_instruction(&mut states, &Instruction::AddX(-1));
        run_instruction(&mut states, &Instruction::AddX(5));
        run_instruction(&mut states, &Instruction::AddX(-1));
        run_instruction(&mut states, &Instruction::AddX(5));
        run_instruction(&mut states, &Instruction::AddX(-1));
        run_instruction(&mut states, &Instruction::AddX(-35));
        run_instruction(&mut states, &Instruction::AddX(1));
        run_instruction(&mut states, &Instruction::AddX(24));
        run_instruction(&mut states, &Instruction::AddX(-19));
        run_instruction(&mut states, &Instruction::AddX(1));
        run_instruction(&mut states, &Instruction::AddX(16));
        run_instruction(&mut states, &Instruction::AddX(-11));
        assert_eq!("##..##..##..##..##..##..##..##..##..##..", draw_screen(&states).lines().next().unwrap());
    }
}

pub fn run(path: &PathBuf, bonus: bool) -> String {
    let mut states = vec![1];

    if let Ok(lines) = read_lines(path) {
        for maybe_line in lines {
            if let Ok(line) = maybe_line {
                if let Ok(instruction) = Instruction::from_str(line.as_str()) {
                    run_instruction(&mut states, &instruction);
                }
            }
        }
    }

    if bonus {
        draw_screen(&states)
    } else {
        calculate_signal_strength(&states, &vec![20, 60, 100, 140, 180, 220]).to_string()
    }
}

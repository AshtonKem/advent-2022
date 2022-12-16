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
}

pub fn run(path: &PathBuf, _bonus_: bool) -> isize {
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

    calculate_signal_strength(&states, &vec![20, 60, 100, 140, 180, 220])
}

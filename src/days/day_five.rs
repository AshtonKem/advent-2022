use std::{
    borrow::Borrow, cmp, collections::HashMap, num::ParseIntError, ops::RangeFrom, path::PathBuf,
    str::FromStr,
};

use crate::utils::read_lines;

#[derive(Debug)]
struct State {
    stacks: Vec<Vec<char>>,
}

impl State {
    fn new() -> State {
        State { stacks: Vec::new() }
    }

    fn build(input: &mut Vec<String>) -> State {
        let mut state = State::new();
        let mut indices: HashMap<usize, usize> = HashMap::new();
        for (i, character) in input
            .pop()
            .expect("Expected column line")
            .chars()
            .enumerate()
        {
            if character != ' ' {
                let column: usize = character
                    .to_string()
                    .parse()
                    .expect("Invalid column number");
                indices.insert(i, column - 1);
            }
        }
        for line in input {
            for (i, character) in line.chars().enumerate() {
                if indices.contains_key(&i) && character != ' ' {
                    if let Some(column) = indices.get(&i) {
                        state.add_item(*column, character);
                    }
                }
            }
        }
        state.finish_construction();
        state
    }

    fn add_stack(&mut self) {
        self.stacks.push(Vec::new());
    }

    fn finish_construction(&mut self) {
        for stack in self.stacks.iter_mut() {
            stack.reverse();
        }
    }

    fn add_item(&mut self, stack: usize, item: char) {
        while self.stacks.len() <= stack {
            self.add_stack();
        }
        self.stacks
            .get_mut(stack)
            .expect("Out of bounds stack")
            .push(item);
    }

    fn implement_move(&mut self, move_struct: &Move, bonus: bool) {
        let from = self.stacks.get(move_struct.from).expect("Out of range");
        let range: RangeFrom<usize> = RangeFrom {
            start: from.len() - move_struct.count,
        };
        let mut intermediate: Vec<char> = self
            .stacks
            .get_mut(move_struct.from)
            .expect("Out of range")
            .drain(range)
            .collect();
        if !bonus {
            intermediate.reverse();
        }
        let target = self.stacks.get_mut(move_struct.to).expect("Out of range");
        for val in intermediate {
            target.push(val);
        }
    }

    fn final_state(&self) -> String {
        let mut result: String = "".to_string();
        for stack in &self.stacks {
            if !stack.is_empty() {
                result += &stack.last().unwrap().to_string();
            }
        }
        result
    }
}

#[derive(Debug)]
struct Move {
    from: usize,
    to: usize,
    count: usize,
}

impl FromStr for Move {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b, c) = s
            .strip_prefix("move ")
            .and_then(|s| s.split_once(" from "))
            .and_then(|(a, s)| s.split_once(" to ").map(|(b, c)| (a, b, c)))
            .unwrap();
        Ok(Move {
            from: b.parse::<usize>()? - 1,
            to: c.parse::<usize>()? - 1,
            count: a.parse::<usize>()?,
        })
    }
}

pub fn run(path: &PathBuf, bonus: bool) -> String {
    let mut state = State::new();
    if let Ok(lines) = read_lines(path) {
        let mut initial_state: Vec<String> = Vec::new();
        let mut finalized = false;
        for maybe_line in lines {
            if let Ok(line) = maybe_line {
                if line == "" {
                    finalized = true;
                    state = State::build(&mut initial_state);
                } else if finalized {
                    state.implement_move(
                        Move::from_str(line.as_str())
                            .expect("Invalid input")
                            .borrow(),
                        bonus,
                    );
                } else {
                    initial_state.push(line);
                }
            }
        }
    }
    state.final_state()
}

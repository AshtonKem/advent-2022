use std::{cmp, collections::HashSet, num::ParseIntError, path::PathBuf, str::FromStr};

use crate::utils::read_lines;

#[derive(PartialEq, PartialOrd, Eq, Hash, Clone, Debug)]
struct Coordinate {
    x: isize,
    y: isize,
}

impl Coordinate {
    fn new(x: isize, y: isize) -> Coordinate {
        Coordinate { x, y }
    }

    fn shift(&self, x: isize, y: isize) -> Coordinate {
        Coordinate::new(self.x + x, self.y + y)
    }

    fn delta_x(&self, other: &Coordinate) -> isize {
        self.x - other.x
    }

    fn delta_y(&self, other: &Coordinate) -> isize {
        self.y - other.y
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, Clone, Copy)]
struct Move {
    direction: Direction,
    times: usize,
}

impl Move {
    fn right(times: usize) -> Move {
        Move {
            direction: Direction::Right,
            times,
        }
    }

    fn left(times: usize) -> Move {
        Move {
            direction: Direction::Left,
            times,
        }
    }

    fn up(times: usize) -> Move {
        Move {
            direction: Direction::Up,
            times,
        }
    }

    fn down(times: usize) -> Move {
        Move {
            direction: Direction::Down,
            times,
        }
    }
}

impl FromStr for Move {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (d, n) = s.split_once(" ").expect("Should have space");
        let times = n.parse::<usize>()?;
        match d {
            "R" => Ok(Move::right(times)),
            "L" => Ok(Move::left(times)),
            "U" => Ok(Move::up(times)),
            "D" => Ok(Move::down(times)),
            _ => panic!("Unexpected input"),
        }
    }
}

struct Bridge {
    head_position: Coordinate,
    tail_position: Coordinate,
    past_tail_positions: HashSet<Coordinate>,
    max_x: isize,
    max_y: isize,
}

fn calculate_shift(head: isize, tail: isize) -> isize {
    if head == tail {
        0
    } else if head > tail {
        1
    } else {
        -1
    }
}

impl Bridge {
    fn new() -> Bridge {
        Bridge {
            head_position: Coordinate::new(0, 0),
            tail_position: Coordinate::new(0, 0),
            past_tail_positions: HashSet::from_iter(vec![Coordinate::new(0, 0)]),
            max_x: 0,
            max_y: 0,
        }
    }

    fn max_positions(&self) -> usize {
        self.past_tail_positions.len()
    }

    fn move_tail(&mut self) {
        let delta_x = self.head_position.delta_x(&self.tail_position);
        let delta_y = self.head_position.delta_y(&self.tail_position);
        if self.head_position == self.tail_position {
            return;
        } else if delta_x.abs() == 1 && delta_y == 0 {
            return;
        } else if delta_y.abs() == 1 && delta_x == 0 {
            return;
        } else if delta_x.abs() == 1 && delta_y.abs() == 1 {
            return;
        }
        else {
            let x_shift = calculate_shift(self.head_position.x, self.tail_position.x);
            let y_shift = calculate_shift(self.head_position.y, self.tail_position.y);
            self.tail_position = self.tail_position.shift(x_shift, y_shift);
            self.past_tail_positions.insert(self.tail_position.clone());
        }
    }

    fn update_maxes(&mut self) {
        self.max_x = cmp::max(self.max_x, self.head_position.x);
        self.max_y = cmp::max(self.max_y, self.head_position.y);
    }

    fn process_move(&mut self, move_struct: &Move) {
        for _ in 0..(move_struct.times) {
            match move_struct.direction {
                Direction::Left => self.head_position = self.head_position.shift(-1, 0),
                Direction::Right => self.head_position = self.head_position.shift(1, 0),
                Direction::Up => self.head_position = self.head_position.shift(0, 1),
                Direction::Down => self.head_position = self.head_position.shift(0, -1),
            };
            self.move_tail();
            self.update_maxes();
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test_move_simple() {
        let mut bridge = Bridge::new();
        assert_eq!(1, bridge.max_positions());
        bridge.process_move(&Move {
            direction: Direction::Up,
            times: 1,
        });
        // Tail should not have moved yet
        assert_eq!(1, bridge.max_positions());
        assert_eq!(Coordinate::new(0, 1), bridge.head_position);
        assert_eq!(Coordinate::new(0, 0), bridge.tail_position);

        bridge.process_move(&Move {
            direction: Direction::Up,
            times: 1,
        });
        assert_eq!(2, bridge.max_positions());
        assert_eq!(Coordinate::new(0, 2), bridge.head_position);
        assert_eq!(Coordinate::new(0, 1), bridge.tail_position);
    }

    #[test]
    pub fn test_diagonal() {
        let mut bridge = Bridge::new();
        assert_eq!(1, bridge.max_positions());
        bridge.process_move(&Move {
            direction: Direction::Up,
            times: 1,
        });
        bridge.process_move(&Move {
            direction: Direction::Right,
            times: 1,
        });
        // Tail should not have moved yet
        assert_eq!(1, bridge.max_positions());
        assert_eq!(Coordinate::new(1, 1), bridge.head_position);
        assert_eq!(Coordinate::new(0, 0), bridge.tail_position);

        bridge.process_move(&Move {
            direction: Direction::Up,
            times: 1,
        });
        assert_eq!(2, bridge.max_positions());
        assert_eq!(Coordinate::new(1, 2), bridge.head_position);
        assert_eq!(Coordinate::new(1, 1), bridge.tail_position);
    }
}

pub fn run(path: &PathBuf, _bonus_: bool) -> usize {
    let mut bridge = Bridge::new();
    if let Ok(lines) = read_lines(path) {
        for maybe_line in lines {
            if let Ok(line) = maybe_line {
                if let Ok(planned_move) = Move::from_str(line.as_str()) {
                    bridge.process_move(&planned_move);
                }
            }
        }
    }
    bridge.max_positions()
}

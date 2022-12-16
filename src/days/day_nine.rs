use std::{collections::HashSet, num::ParseIntError, path::PathBuf, str::FromStr};

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
    knots: Vec<Coordinate>,
    past_tail_positions: HashSet<Coordinate>,
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

fn calculate_new_position(knot: &Coordinate, following: &Coordinate) -> Coordinate {
    let delta_x = knot.delta_x(following);
    let delta_y = knot.delta_y(following);
    if knot == following {
        return following.clone();
    } else if delta_x.abs() == 1 && delta_y == 0 {
        return following.clone();
    } else if delta_y.abs() == 1 && delta_x == 0 {
        return following.clone();
    } else if delta_x.abs() == 1 && delta_y.abs() == 1 {
        return following.clone();
    } else {
        let x_shift = calculate_shift(knot.x, following.x);
        let y_shift = calculate_shift(knot.y, following.y);
        return following.shift(x_shift, y_shift);
    }
}

impl Bridge {
    fn new(knot_count: usize) -> Bridge {
        let mut knots = Vec::new();
        for _ in 0..knot_count {
            knots.push(Coordinate::new(0, 0));
        }
        Bridge {
            knots,
            past_tail_positions: HashSet::from_iter(vec![Coordinate::new(0, 0)]),
        }
    }

    fn max_positions(&self) -> usize {
        self.past_tail_positions.len()
    }

    fn move_head(&mut self, x: isize, y: isize) {
        let old_position = self.knots.first().expect("Non empty knots expected");
        let new_position = old_position.shift(x, y);
        self.knots[0] = new_position;
    }

    fn process_move(&mut self, move_struct: &Move) {
        for _ in 0..(move_struct.times) {
            match move_struct.direction {
                Direction::Left => self.move_head(-1, 0),
                Direction::Right => self.move_head(1, 0),
                Direction::Up => self.move_head(0, 1),
                Direction::Down => self.move_head(0, -1),
            };
            for index in 1..(self.knots.len()) {
                let previous = &self.knots[index - 1];
                let following = &self.knots[index];
                self.knots[index] = calculate_new_position(previous, following);
            }

            self.past_tail_positions
                .insert(self.tail_position().clone());
        }
    }

    #[allow(dead_code)]
    fn head_position(&self) -> &Coordinate {
        self.knots.first().expect("Should have a first knot")
    }

    fn tail_position(&self) -> &Coordinate {
        self.knots.last().expect("Should have more than 0 knots")
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test_move_simple() {
        let mut bridge = Bridge::new(2);
        assert_eq!(1, bridge.max_positions());
        bridge.process_move(&Move {
            direction: Direction::Up,
            times: 1,
        });
        // Tail should not have moved yet
        assert_eq!(1, bridge.max_positions());
        assert_eq!(&Coordinate::new(0, 1), bridge.head_position());
        assert_eq!(&Coordinate::new(0, 0), bridge.tail_position());

        bridge.process_move(&Move {
            direction: Direction::Up,
            times: 1,
        });
        assert_eq!(2, bridge.max_positions());
        assert_eq!(&Coordinate::new(0, 2), bridge.head_position());
        assert_eq!(&Coordinate::new(0, 1), bridge.tail_position());
    }

    #[test]
    pub fn test_diagonal() {
        let mut bridge = Bridge::new(2);
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
        assert_eq!(&Coordinate::new(1, 1), bridge.head_position());
        assert_eq!(&Coordinate::new(0, 0), bridge.tail_position());

        bridge.process_move(&Move {
            direction: Direction::Up,
            times: 1,
        });
        assert_eq!(2, bridge.max_positions());
        assert_eq!(&Coordinate::new(1, 2), bridge.head_position());
        assert_eq!(&Coordinate::new(1, 1), bridge.tail_position());
    }
}

pub fn run(path: &PathBuf, bonus: bool) -> usize {
    let mut bridge = if bonus {
        Bridge::new(10)
    } else {
        Bridge::new(2)
    };
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

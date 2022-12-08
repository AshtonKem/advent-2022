use std::{path::PathBuf, str::FromStr, num::ParseIntError};

use crate::utils::read_lines;

struct Assignment {
    start: u8,
    end: u8
}

impl FromStr for Assignment {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.split_once('-').expect("should contain one dash");

        Ok(Assignment {
            start: start.parse::<u8>()?,
            end: end.parse::<u8>()?
        })
    }
}

impl Assignment {
    fn overlaps(&self, other: &Assignment) -> bool {
        self.start <= other.start && self.end >= other.end
    }
}

struct Pair {
    first: Assignment,
    second: Assignment
}

impl FromStr for Pair {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (first, second) = s.split_once(",").expect("Should contain one comma");
        Ok(Pair{
            first: Assignment::from_str(first)?,
            second: Assignment::from_str(second)?
        })
    }
}

impl Pair {
    fn duplicated(&self) -> bool {
        self.first.overlaps(&self.second) || self.second.overlaps(&self.first)
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Borrow;

    use super::*;

    #[test]
    fn test_overlap() {
        assert_eq!(false, Assignment::from_str("3-7").unwrap().overlaps(Assignment::from_str("2-8").unwrap().borrow()));
        assert_eq!(true, Assignment::from_str("2-8").unwrap().overlaps(Assignment::from_str("3-7").unwrap().borrow()));
    }

    #[test]
    fn test_overlap_edge_cases() {
        assert!(Assignment::from_str("6-6").unwrap().overlaps(Assignment::from_str("6-6").unwrap().borrow()));
        assert!(Assignment::from_str("4-6").unwrap().overlaps(Assignment::from_str("6-6").unwrap().borrow()));
        assert!(Assignment::from_str("6-7").unwrap().overlaps(Assignment::from_str("6-6").unwrap().borrow()));
        assert!(Assignment::from_str("6-8").unwrap().overlaps(Assignment::from_str("6-6").unwrap().borrow()));
    }

    #[test]
    fn test_duplicated() {
        assert_eq!(true, Pair::from_str("2-8,3-7").unwrap().duplicated());
        assert_eq!(true, Pair::from_str("6-6,4-6").unwrap().duplicated());
        assert_eq!(false, Pair::from_str("2-6,4-8").unwrap().duplicated());
    }
}


pub fn run(path: &PathBuf, _bonus_: bool) -> u32 {
    let mut count = 0;
    if let Ok(lines) = read_lines(path) {
        for maybe_line in lines {
            if let Ok(line) = maybe_line {
                if Pair::from_str(line.as_str()).expect("Invalid input").duplicated() {
                    count += 1
                }
            }
        }
    }
    count
}

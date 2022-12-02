use std::{
    fs::File,
    io::{self, BufRead},
    path::{Path, PathBuf},
};

trait Scorable {
    fn get_score(&self) -> u32;
}

#[derive(PartialEq, Debug)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl Scorable for Hand {
    fn get_score(&self) -> u32 {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        }
    }
}

impl From<String> for Hand {
    fn from(str: String) -> Self {
        match str.as_str() {
            "X" => Hand::Rock,
            "A" => Hand::Rock,
            "Y" => Hand::Paper,
            "B" => Hand::Paper,
            "Z" => Hand::Scissors,
            "C" => Hand::Scissors,
            _ => panic!("Unexpected string"),
        }
    }
}

#[derive(Debug)]
enum Outcome {
    Win,
    Loss,
    Draw,
}

impl Scorable for Outcome {
    fn get_score(&self) -> u32 {
        match self {
            Outcome::Win => 6,
            Outcome::Loss => 0,
            Outcome::Draw => 3,
        }
    }
}

fn outcome(our_hand: &Hand, their_hand: &Hand) -> Outcome{
    match (our_hand, their_hand) {
        (Hand::Rock, Hand::Paper) => Outcome::Loss,
        (Hand::Rock, Hand::Scissors) => Outcome::Win,
        (Hand::Paper, Hand::Rock) => Outcome::Win,
        (Hand::Paper, Hand::Scissors) => Outcome::Loss,
        (Hand::Scissors, Hand::Rock) => Outcome::Loss,
        (Hand::Scissors, Hand::Paper) => Outcome::Win,
        _ => Outcome::Draw
    }
}

pub fn run(path: &PathBuf) -> u32 {
    let mut score = 0;
    if let Ok(lines) = read_lines(path) {
        for maybe_line in lines {
            if let Ok(line) = maybe_line {
                let hands: Vec<&str> = line.split(" ").collect();
                assert!(hands.len() == 2, "Expected only 2 hands per line");
                let their_hand: Hand = hands.first().unwrap().to_string().into();
                let our_hand: Hand = hands.last().unwrap().to_string().into();
                let outcome = outcome(&our_hand, &their_hand);
                score += our_hand.get_score();
                score += outcome.get_score();
            }
        }
    }
    score
}

fn read_lines(filename: &PathBuf) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

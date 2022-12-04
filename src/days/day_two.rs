use std::path::PathBuf;

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

impl From<String> for Outcome {
    fn from(str: String) -> Self {
        match str.as_str() {
            "X" => Outcome::Loss,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            _ => panic!("Unexpected input"),
        }
    }
}

fn calculate_hand(outcome: &Outcome, their_hand: &Hand) -> Hand {
    match (outcome, their_hand) {
        (Outcome::Win, Hand::Rock) => Hand::Paper,
        (Outcome::Win, Hand::Paper) => Hand::Scissors,
        (Outcome::Win, Hand::Scissors) => Hand::Rock,
        (Outcome::Loss, Hand::Rock) => Hand::Scissors,
        (Outcome::Loss, Hand::Paper) => Hand::Rock,
        (Outcome::Loss, Hand::Scissors) => Hand::Paper,
        (Outcome::Draw, Hand::Rock) => Hand::Rock,
        (Outcome::Draw, Hand::Paper) => Hand::Paper,
        (Outcome::Draw, Hand::Scissors) => Hand::Scissors,
    }
}

fn outcome(our_hand: &Hand, their_hand: &Hand) -> Outcome {
    match (our_hand, their_hand) {
        (Hand::Rock, Hand::Paper) => Outcome::Loss,
        (Hand::Rock, Hand::Scissors) => Outcome::Win,
        (Hand::Paper, Hand::Rock) => Outcome::Win,
        (Hand::Paper, Hand::Scissors) => Outcome::Loss,
        (Hand::Scissors, Hand::Rock) => Outcome::Loss,
        (Hand::Scissors, Hand::Paper) => Outcome::Win,
        _ => Outcome::Draw,
    }
}

pub fn run(path: &PathBuf, bonus: bool) -> u32 {
    let mut score = 0;
    if let Ok(lines) = crate::utils::read_lines(path) {
        for maybe_line in lines {
            if let Ok(line) = maybe_line {
                let hands: Vec<&str> = line.split(" ").collect();
                assert!(hands.len() == 2, "Expected only 2 hands per line");
                let their_hand: Hand = hands.first().unwrap().to_string().into();
                if bonus {
                    let outcome: Outcome = hands.last().unwrap().to_string().into();
                    let our_hand = calculate_hand(&outcome, &their_hand);
                    score += our_hand.get_score();
                    score += outcome.get_score();
                } else {
                    let our_hand: Hand = hands.last().unwrap().to_string().into();
                    let outcome = outcome(&our_hand, &their_hand);
                    score += our_hand.get_score();
                    score += outcome.get_score();
                }
            }
        }
    }
    score
}

use std::fs::File;
use std::io::{self, BufRead};
use std::path::PathBuf;

struct Elf {
    food: Vec<u32>,
}

impl Elf {
    fn new() -> Elf {
        Elf { food: vec![] }
    }

    fn add_food(&mut self, food: u32) {
        self.food.push(food);
    }

    fn total_calories(&self) -> u32 {
        self.food.iter().sum()
    }
}

pub fn run(path: &PathBuf) -> u32 {
    let mut elves: Vec<Elf> = vec![];
    let mut current_elf = Elf::new();
    if let Ok(lines) = read_lines(path) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                if ip == "" {
                    elves.push(current_elf);
                    current_elf = Elf::new();
                } else {
                    // Panic is ok here
                    let calories = ip.parse::<u32>().unwrap();
                    current_elf.add_food(calories);
                }
            }
        }
        elves
            .iter()
            .max_by_key(|elf| elf.total_calories())
            .expect("Expected some elves to be provided")
            .total_calories()
    } else {
        0
    }
}

fn read_lines(filename: &PathBuf) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

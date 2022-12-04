use std::{
    fs::File,
    io::{self, BufRead},
    path::PathBuf,
};

pub fn read_lines(filename: &PathBuf) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

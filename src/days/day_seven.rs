use std::{
    fs,
    io::{self, BufReader, Lines},
    path::PathBuf,
};

use crate::utils::read_lines;

trait FileSize {
    fn get_size(&self) -> usize;
}

#[derive(Debug)]
struct File {
    name: String,
    size: usize,
}

impl File {
    fn new(name: String, size: usize) -> File {
        File { name, size }
    }
}

impl FileSize for File {
    fn get_size(&self) -> usize {
        self.size
    }
}

#[derive(Debug)]
struct Directory {
    name: String,
    files: Vec<File>,
    descendants: Vec<Directory>,
}

impl Directory {
    fn new(name: String) -> Directory {
        Directory {
            name,
            files: Vec::new(),
            descendants: Vec::new(),
        }
    }

    fn add_file(&mut self, file: File) {
        self.files.push(file);
    }

    fn add_directory(&mut self, directory: Directory) {
        self.descendants.push(directory);
    }
}

impl FileSize for Directory {
    fn get_size(&self) -> usize {
        let file_sizes: usize = self.files.iter().map(FileSize::get_size).sum();
        let dir_sizes: usize = self.descendants.iter().map(|f| f.get_size()).sum();
        file_sizes + dir_sizes
    }
}

fn handle_file(directory: &mut Directory, line: String) {
    if let Some((size_str, filename)) = line.split_once(" ") {
        if let Ok(size) = size_str.parse::<usize>() {
            let file = File::new(filename.to_string(), size);
            directory.add_file(file);
        } else {
            panic!("Invalid file size {}", line);
        }
    } else {
        panic!("Invalid file string {}", line);
    }
}

fn build_directory(name: String, lines: &mut Lines<BufReader<fs::File>>) -> Directory {
    let mut directory = Directory::new(name);
    loop {
        if let Some(Ok(line)) = lines.next() {
            if line == "$ cd .." {
                break;
            } else if line == "$ ls" {
                continue;
            } else if line.starts_with("$ cd") {
                let dir_name = line.clone().remove("$ cd ".len()).to_string();
                directory.add_directory(build_directory(dir_name, lines));
            } else if line.starts_with("dir") {
                continue;
            } else {
                handle_file(&mut directory, line);
            }
        } else {
            break;
        }
    }
    directory
}

fn handle_line(parent: &mut Directory, mut lines: Lines<BufReader<fs::File>>) {
    loop {
        if let Some(Ok(line)) = lines.next() {
            if line == "$ cd .." {
                // Shouldn't happen here
                panic!("Found a unexpected call to 'cd ..")
            } else if line == "$ cd /" {
                continue;
            } else if line.starts_with("$ cd ") {
                let dir_name = line.clone().remove("$ cd ".len()).to_string();
                parent.add_directory(build_directory(dir_name, &mut lines));
            }
        } else {
            break;
        }
    }
}

fn sum_directories(directory: &Directory, limit: usize) -> usize {
    let mut sum = 0;
    if directory.get_size() < limit {
        sum += directory.get_size();
    }
    let descendants: &Vec<Directory> = &directory.descendants;
    sum += descendants
        .into_iter()
        .map(|d| sum_directories(&d, limit))
        .sum::<usize>();
    sum
}

pub fn run(path: &PathBuf, _bonus_: bool) -> usize {
    let mut count = 0;
    let mut root = Directory::new("/".to_string());

    if let Ok(lines) = read_lines(path) {
        handle_line(&mut root, lines);
        count = sum_directories(&root, 100_000);
    }
    count
}

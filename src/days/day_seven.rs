use std::{
    fs,
    io::{BufReader, Lines},
    path::PathBuf,
};

use crate::utils::read_lines;

trait FileSize {
    fn get_size(&self) -> usize;
}

#[derive(Debug)]
struct File {
    #[allow(dead_code)]
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
struct Directory{
    #[allow(dead_code)]
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

    fn get_sizes(&self) -> Vec<usize> {
        let mut results = Vec::new();
        results.push(self.get_size());
        for dir in self.descendants.iter() {
            results.append(&mut dir.get_sizes());
        }
        results
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

fn handle_line(mut lines: Lines<BufReader<fs::File>>) -> Directory {
    if let Some(Ok(line)) = lines.next() {
        if line != "$ cd /" {
            panic!("Unexpected first cd command '{}'", line);
        } else {
            return build_directory("/".to_string(), &mut lines);
        }
    } else {
        panic!("Could not read file");
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

fn smallest_directory(directory: &Directory, limit: usize) -> usize {
    let mut sizes = directory.get_sizes();
    sizes.sort();
    sizes
        .into_iter()
        .filter(|s| s.clone().clone() > limit)
        .next()
        .unwrap_or(0)
}

pub fn run(path: &PathBuf, bonus: bool) -> usize {
    if let Ok(lines) = read_lines(path) {
        let root = handle_line(lines);
        if bonus {
            let disk_size = 70_000_000;
            let unused = disk_size - root.get_size();
            let target_size = 30_000_000;
            let to_delete = target_size - unused;
            return smallest_directory(&root, to_delete);
        } else {
            return sum_directories(&root, 100_000);
        }
    }
    0
}

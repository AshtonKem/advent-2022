use std::{cmp, path::PathBuf};

use array2d::Array2D;

use crate::utils::read_lines;

fn build_state(input: &Vec<Vec<usize>>) -> Array2D<bool> {
    let rows = input.len();
    let cols = input
        .iter()
        .map(Vec::len)
        .max()
        .expect("Expected non empty rows");
    Array2D::filled_with(false, rows, cols)
}

fn build_input(input: &Vec<Vec<usize>>) -> Array2D<usize> {
    Array2D::from_rows(input).expect("Expected square input")
}

fn visibility(input: &Array2D<usize>, x: usize, y: usize) -> usize {
    let left = visibility_left(input, x, y);
    let right = visibility_right(input, x, y);
    let up = visibility_up(input, x, y);
    let down = visibility_down(input, x, y);
    let total = left * right * up * down;

    total
}

fn visibility_down(input: &Array2D<usize>, x: usize, y: usize) -> usize {
    let max_val = input.get(y, x).expect("Invalid X & Y");
    if y >= (input.column_len() - 1) {
        0
    } else {
        let mut count = 0;
        let iter = input.column_iter(x).expect("Invalid column").skip(y + 1);
        for item in iter {
            count += 1;
            if item >= max_val {
                break;
            }
        }
        count
    }
}

fn visibility_up(input: &Array2D<usize>, x: usize, y: usize) -> usize {
    let max_val = input.get(y, x).expect("Invalid X & Y");
    if y <= 0 {
        0
    } else {
        let mut count = 0;
        let iter = input
            .column_iter(x)
            .expect("Invalid column")
            .rev()
            .skip(input.num_rows() - y);
        for item in iter {
            count += 1;
            if item >= max_val {
                break;
            }
        }
        count
    }
}

fn visibility_right(input: &Array2D<usize>, x: usize, y: usize) -> usize {
    let max_val = input.get(y, x).expect("Invalid X & Y");
    if x >= (input.num_columns() - 1) {
        0
    } else {
        let mut count = 0;
        let iter = input.row_iter(y).expect("Invalid row").skip(x + 1);
        for item in iter {
            count += 1;
            if item >= max_val {
                break;
            }
        }
        count
    }
}

fn visibility_left(input: &Array2D<usize>, x: usize, y: usize) -> usize {
    let max_val = input.get(y, x).expect("Invalid X & Y");
    if x <= 0 {
        0
    } else {
        let mut count = 0;
        let iter = input
            .row_iter(y)
            .expect("Invalid row")
            .rev()
            .skip(input.num_columns() - x);
        for item in iter {
            count += 1;
            if item >= max_val {
                break;
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_visibility_up() {
        let input: Array2D<usize> = Array2D::from_rows(&vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ])
        .unwrap();
        assert_eq!(1, visibility_up(&input, 3, 1));
        assert_eq!(0, visibility_up(&input, 1, 0));
        assert_eq!(1, visibility_up(&input, 2, 4));
        assert_eq!(4, visibility_up(&input, 3, 4));
        assert_eq!(3, visibility_up(&input, 3, 3));
        assert_eq!(2, visibility_up(&input, 2, 3));
    }

    #[test]
    pub fn test_visibility_down() {
        let input: Array2D<usize> = Array2D::from_rows(&vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ])
        .unwrap();
        assert_eq!(0, visibility_down(&input, 0, 4));
        assert_eq!(0, visibility_down(&input, 4, 4));
        assert_eq!(2, visibility_down(&input, 0, 0));
        assert_eq!(1, visibility_down(&input, 1, 0));
        assert_eq!(3, visibility_down(&input, 4, 0));
        assert_eq!(1, visibility_down(&input, 1, 1));
        assert_eq!(1, visibility_down(&input, 2, 3));
    }

    #[test]
    pub fn test_visibility_left() {
        let input: Array2D<usize> = Array2D::from_rows(&vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ])
        .unwrap();
        assert_eq!(0, visibility_left(&input, 0, 0));
        assert_eq!(1, visibility_left(&input, 1, 0));
        assert_eq!(1, visibility_left(&input, 1, 1));
        assert_eq!(1, visibility_left(&input, 3, 2));
        assert_eq!(4, visibility_left(&input, 4, 3));
        assert_eq!(2, visibility_left(&input, 2, 3));
    }

    #[test]
    pub fn test_visibility_right() {
        let input: Array2D<usize> = Array2D::from_rows(&vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ])
        .unwrap();
        assert_eq!(0, visibility_right(&input, 4, 0));
        assert_eq!(1, visibility_right(&input, 3, 1));
        assert_eq!(1, visibility_right(&input, 3, 2));
        assert_eq!(1, visibility_right(&input, 2, 2));
        assert_eq!(1, visibility_right(&input, 2, 2));
        assert_eq!(2, visibility_right(&input, 2, 3));
    }
}

fn max_visibility(input: &Vec<Vec<usize>>) -> usize {
    let input_array = Array2D::from_rows(input).expect("Could not build 2D Array()");

    input_array
        .enumerate_row_major()
        .map(|((x, y), _value_)| visibility(&input_array, x, y))
        .max()
        .expect("Could not determine any visibility")
}

fn iterate(state: &mut Array2D<bool>, input: &Array2D<usize>) {
    iterate_normal(state, input);
    iterate_reverse(state, input);
}

fn iterate_normal(state: &mut Array2D<bool>, input: &Array2D<usize>) {
    let cols = input.column_len();
    let rows = input.row_len();
    for (y, row) in input.rows_iter().enumerate() {
        let mut x_max = 0;
        for (x, val) in row.enumerate() {
            if x == 0 || x == (cols - 1) {
                state.set(x, y, true).expect("Expected set to work");
            } else if *val > x_max {
                state.set(x, y, true).expect("Expected set to work");
            }
            x_max = cmp::max(x_max, *val);
        }
    }
    for (x, column) in input.columns_iter().enumerate() {
        let mut y_max = 0;
        for (y, val) in column.enumerate() {
            if y == 0 || y == (rows - 1) {
                state.set(x, y, true).expect("Expected set to work");
            } else if *val > y_max {
                state.set(x, y, true).expect("Expected set to work");
            }
            y_max = cmp::max(y_max, *val);
        }
    }
}

fn iterate_reverse(state: &mut Array2D<bool>, input: &Array2D<usize>) {
    let cols = input.column_len();
    let rows = input.row_len();
    for (y, row) in input.rows_iter().enumerate() {
        let mut x_max = 0;
        for (x_rev, val) in row.rev().enumerate() {
            let x = cols - x_rev - 1;
            if x == 0 || x == (cols - 1) {
                state.set(x, y, true).expect("Expected set to work");
            } else if *val > x_max {
                state.set(x, y, true).expect("Expected set to work");
            }
            x_max = cmp::max(x_max, *val);
        }
    }
    for (x, column) in input.columns_iter().enumerate() {
        let mut y_max = 0;
        for (y_rev, val) in column.rev().enumerate() {
            let y = rows - y_rev - 1;
            if y == 0 || y == (rows - 1) {
                state.set(x, y, true).expect("Expected set to work");
            } else if *val > y_max {
                state.set(x, y, true).expect("Expected set to work");
            }
            y_max = cmp::max(y_max, *val);
        }
    }
}

fn count(state: Array2D<bool>) -> usize {
    state
        .enumerate_column_major()
        .map(|(_index, value)| *value)
        .filter(|x| *x)
        .count()
}

pub fn run(path: &PathBuf, bonus: bool) -> usize {
    if let Ok(lines) = read_lines(path) {
        let mut input_vec: Vec<Vec<usize>> = Vec::new();
        for maybe_line in lines {
            let mut current_line: Vec<usize> = Vec::new();
            if let Ok(line) = maybe_line {
                for character in line.chars() {
                    current_line
                        .push(character.to_digit(10).expect("Expected valid ASCII digit") as usize);
                }
                input_vec.push(current_line);
            }
        }
        if bonus {
            return max_visibility(&input_vec);
        } else {
            let input = build_input(&input_vec);
            let mut state = build_state(&input_vec);
            iterate(&mut state, &input);
            return count(state);
        }
    }
    0
}

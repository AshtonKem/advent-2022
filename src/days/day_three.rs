use std::{collections::HashSet, iter::FromIterator, path::PathBuf};

fn get_index(character: &char) -> u32 {
    let point = *character as u32;
    if character.is_uppercase() {
        point - 38
    } else {
        point - 96
    }
}

fn split(line: &str) -> (&str, &str) {
    line.split_at(line.len() / 2)
}

fn overlap<'a>(first: &'a str, second: &'a str) -> HashSet<char> {
    let first_chars: HashSet<char> = HashSet::from_iter(first.chars());
    let second_chars: HashSet<char> = HashSet::from_iter(second.chars());
    first_chars.intersection(&second_chars).copied().collect()
}

fn process(input: &str) -> u32 {
    let mut score = 0;
    let (first_half, second_half) = split(input);
    let repeats = overlap(&first_half, &second_half);
    for character in repeats {
        score += get_index(&character);
    }
    score
}

pub fn run(path: &PathBuf, bonus: bool) -> u32 {
    let mut score = 0;
    if let Ok(lines) = crate::utils::read_lines(path) {
        for maybe_line in lines {
            if let Ok(line) = maybe_line {
                score += process(&line);
            }
        }
    }
    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index() {
        let lower_a = 'a';
        let lower_z = 'z';
        let upper_a = 'A';
        let upper_z = 'Z';
        assert_eq!(1, get_index(&lower_a));
        assert_eq!(26, get_index(&lower_z));
        assert_eq!(27, get_index(&upper_a));
        assert_eq!(52, get_index(&upper_z))
    }

    #[test]
    fn test_split() {
        assert_eq!(("foo", "bar"), split("foobar"));
    }

    #[test]
    fn test_overlap() {
        assert_eq!(HashSet::from(['a']), overlap("a", "a"));
        assert_eq!(
            HashSet::from(['p']),
            overlap("vJrwpWtwJgWr", "hcsFMMfFFhFp")
        );
    }

    #[test]
    fn test_process() {
        assert_eq!(16, process("vJrwpWtwJgWrhcsFMMfFFhFp"));
    }
}

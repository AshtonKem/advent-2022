use std::{collections::VecDeque, path::PathBuf};

use itertools::Itertools;

use crate::utils::read_lines;

fn find_start_packet_index(input: String, header_size: usize) -> usize {
    let mut ring_buffer: VecDeque<char> = VecDeque::new();
    for (index, character) in input.chars().enumerate() {
        ring_buffer.push_front(character);
        ring_buffer.truncate(header_size);
        let char_count = ring_buffer.iter().unique().count();
        if char_count == header_size {
            return index + 1;
        }
    }
    input.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_start_packet_index() {
        assert_eq!(
            5,
            find_start_packet_index("bvwbjplbgvbhsrlpgdmjqwftvncz".to_string(), 4)
        );
        assert_eq!(
            6,
            find_start_packet_index("nppdvjthqldpwncqszvftbrmjlhg".to_string(), 4)
        );
        assert_eq!(
            10,
            find_start_packet_index("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string(), 4)
        );
        assert_eq!(
            11,
            find_start_packet_index("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string(), 4)
        );
    }

    #[test]
    pub fn test_start_message() {
        let header_size = 14;
        assert_eq!(
            19,
            find_start_packet_index("mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string(), header_size)
        );
        assert_eq!(
            23,
            find_start_packet_index("bvwbjplbgvbhsrlpgdmjqwftvncz".to_string(), header_size)
        );
        assert_eq!(
            23,
            find_start_packet_index("nppdvjthqldpwncqszvftbrmjlhg".to_string(), header_size)
        );
    }
}

pub fn run(path: &PathBuf, bonus: bool) -> usize {
    if let Ok(lines) = read_lines(path) {
        if let Some(Ok(input)) = lines.into_iter().next() {
            let header_size = if bonus { 14 } else { 4 };
            find_start_packet_index(input, header_size)
        } else {
            panic!("Expected a non-empty file");
        }
    } else {
        panic!("Expected a readable file")
    }
}

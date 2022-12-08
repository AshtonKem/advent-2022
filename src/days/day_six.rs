use std::{collections::VecDeque, path::PathBuf};

use itertools::Itertools;

use crate::utils::read_lines;

fn find_start_packet_index(input: String, header_size: usize) -> usize {
    let mut ring_buffer: VecDeque<char> = VecDeque::new();
    for (index, character) in input.chars().enumerate() {
        ring_buffer.push_front(character);
        ring_buffer.truncate(4);
        let char_count = ring_buffer.iter().unique().count();
        if char_count == header_size {
            return index + 1;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_start_packet_index() {
        assert_eq!(5, find_start_packet_index("bvwbjplbgvbhsrlpgdmjqwftvncz".to_string(), 4));
        assert_eq!(6, find_start_packet_index("nppdvjthqldpwncqszvftbrmjlhg".to_string(), 4));
        assert_eq!(10, find_start_packet_index("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string(), 4));
        assert_eq!(11, find_start_packet_index("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string(), 4));
    }
}

pub fn run(path: &PathBuf, _bonus_: bool) -> usize {
    if let Ok(lines) = read_lines(path) {
        if let Some(Ok(input)) = lines.into_iter().next() {
            find_start_packet_index(input, 4)
        } else {
            panic!("Expected a non-empty file");
        }
    } else {
        panic!("Expected a readable file")
    }
}

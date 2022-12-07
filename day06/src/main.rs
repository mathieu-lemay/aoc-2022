use std::collections::HashSet;
use std::fmt::Display;
use std::time::Instant;

use aoc_common::get_input_as_string;

fn solve(input: &str) -> (impl Display, impl Display) {
    let p1 = get_first_marker(input, 4);
    let p2 = get_first_marker(input, 14);

    (p1, p2)
}

fn main() {
    let input = get_input_as_string("day06.txt");

    let start = Instant::now();

    let (r1, r2) = solve(input.as_str());

    let t = start.elapsed().as_nanos() as f64 / 1000.0;

    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
    println!("Duration: {:.3}μs", t);
}

fn get_first_marker(input: &str, window_size: usize) -> usize {
    for i in 0..input.len() - window_size {
        let window = input
            .chars()
            .skip(i)
            .take(window_size)
            .collect::<HashSet<char>>();

        if window.len() == window_size {
            return i + window_size;
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        assert_eq!(get_first_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4), 7);
        assert_eq!(get_first_marker("bvwbjplbgvbhsrlpgdmjqwftvncz", 4), 5);
        assert_eq!(get_first_marker("nppdvjthqldpwncqszvftbrmjlhg", 4), 6);
        assert_eq!(get_first_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4), 10);
        assert_eq!(get_first_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4), 11);
    }

    #[test]
    fn test_p2() {
        assert_eq!(get_first_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14), 19);
        assert_eq!(get_first_marker("bvwbjplbgvbhsrlpgdmjqwftvncz", 14), 23);
        assert_eq!(get_first_marker("nppdvjthqldpwncqszvftbrmjlhg", 14), 23);
        assert_eq!(
            get_first_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14),
            29
        );
        assert_eq!(get_first_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14), 26);
    }
}

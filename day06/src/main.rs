use itertools::Itertools;
use std::collections::VecDeque;
use std::fmt::Display;
use std::time::Instant;

use aoc_common::get_input_as_string;

fn solve(input: &str) -> (impl Display, impl Display) {
    let (mut towers, moves) = parse_input(&input.iter().map(|s| s.as_str()).collect_vec());

    let p1 = process_moves(&mut towers.clone(), &moves);
    let p2 = process_moves_9001(&mut towers, &moves);

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

fn get_first_marker(input: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        assert_eq!(get_first_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
        assert_eq!(get_first_marker("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(get_first_marker("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(get_first_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(get_first_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

    #[test]
    fn test_p2() {
        let input = vec![
            "    [D]    ",
            "[N] [C]    ",
            "[Z] [M] [P]",
            " 1   2   3 ",
            "",
            "move 1 from 2 to 1",
            "move 3 from 1 to 3",
            "move 2 from 2 to 1",
            "move 1 from 1 to 2",
        ];

        let (mut towers, moves) = parse_input(&input);

        let res = process_moves_9001(&mut towers, &moves);

        assert_eq!(res, "MCD");
    }
}

use std::fmt::Display;
use std::time::Instant;

use aoc_common::get_input_as_string;
use itertools::Itertools;

fn solve(input: &str) -> (impl Display, impl Display) {
    let p1 = get_top_calories(input, 1);
    let p2 = get_top_calories(input, 3);

    (p1, p2)
}

fn get_top_calories(input: &str, n: usize) -> u32 {
    input
        .split("\n\n")
        .map(|e| {
            e.split('\n')
                .filter(|v| !v.is_empty())
                .map(|v| v.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .sorted()
        .rev()
        .take(n)
        .sum()
}

fn main() {
    let input = get_input_as_string("day01.txt");

    let start = Instant::now();

    let (r1, r2) = solve(input.as_str());

    let t = start.elapsed().as_nanos() as f64 / 1000.0;

    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
    println!("Duration: {:.3}μs", t);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
        let res = get_top_calories(&input, 1);

        assert_eq!(res, 24000);
    }

    #[test]
    fn test_p2() {
        let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
        let res = get_top_calories(&input, 3);

        assert_eq!(res, 45000);
    }
}

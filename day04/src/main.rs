use itertools::Itertools;
use std::fmt::Display;
use std::time::Instant;

use aoc_common::get_input;

fn solve(input: &[String]) -> (impl Display, impl Display) {
    let pairs = parse_pairs(input);

    let p1 = count_contained_pairs(&pairs);
    let p2 = count_overlapping_pairs(&pairs);

    (p1, p2)
}

fn main() {
    let input = get_input("day04.txt");

    let start = Instant::now();

    let (r1, r2) = solve(input.as_slice());

    let t = start.elapsed().as_nanos() as f64 / 1000.0;

    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
    println!("Duration: {:.3}μs", t);
}

struct Pair {
    x: u32,
    y: u32,
}

impl Pair {
    #[cfg(test)]
    fn new(x: u32, y: u32) -> Self {
        Pair { x, y }
    }

    fn contains(&self, other: &Pair) -> bool {
        self.x <= other.x && self.y >= other.y
    }

    fn overlaps(&self, other: &Pair) -> bool {
        self.x <= other.y && self.y >= other.x
    }
}

impl From<&str> for Pair {
    fn from(s: &str) -> Self {
        let (x, y) = s
            .split('-')
            .map(|v| v.parse().unwrap())
            .collect_tuple::<(u32, u32)>()
            .unwrap();

        Pair { x, y }
    }
}

fn parse_pairs(input: &[String]) -> Vec<(Pair, Pair)> {
    input
        .iter()
        .map(|s| {
            s.split(',')
                .map(|v| v.into())
                .collect_tuple::<(Pair, Pair)>()
                .unwrap()
        })
        .collect_vec()
}

fn count_contained_pairs(pairs: &[(Pair, Pair)]) -> usize {
    pairs
        .iter()
        .filter(|(p1, p2)| p1.contains(p2) || p2.contains(p1))
        .count()
}

fn count_overlapping_pairs(pairs: &[(Pair, Pair)]) -> usize {
    pairs.iter().filter(|(p1, p2)| p1.overlaps(p2)).count()
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn test_p1() {
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
"
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(String::from)
        .collect_vec();

        let pairs = parse_pairs(&input);

        let res = count_contained_pairs(&pairs);

        assert_eq!(res, 2);
    }

    #[test]
    fn test_p2() {
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
"
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(String::from)
        .collect_vec();

        let pairs = parse_pairs(&input);
        let res = count_overlapping_pairs(&pairs);

        assert_eq!(res, 4);
    }

    #[test]
    fn test_pair_overlaps() {
        assert!(!Pair::new(2, 4).overlaps(&Pair::new(6, 8)));
        assert!(!Pair::new(2, 3).overlaps(&Pair::new(4, 5)));
        assert!(Pair::new(5, 7).overlaps(&Pair::new(7, 9)));
        assert!(Pair::new(2, 8).overlaps(&Pair::new(3, 7)));
        assert!(Pair::new(6, 6).overlaps(&Pair::new(4, 6)));
        assert!(Pair::new(2, 6).overlaps(&Pair::new(4, 8)));
    }
}

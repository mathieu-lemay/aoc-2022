use std::collections::HashSet;
use std::fmt::Display;
use std::time::Instant;

use itertools::Itertools;

use aoc_common::get_input;

fn solve(input: &[String]) -> (impl Display, impl Display) {
    let p1 = get_sum_of_priorities(input);
    let p2 = get_sum_of_priorities_group(input);
    (p1, p2)
}

fn main() {
    let input = get_input("day03.txt");

    let start = Instant::now();

    let (r1, r2) = solve(input.as_slice());

    let t = start.elapsed().as_nanos() as f64 / 1000.0;

    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
    println!("Duration: {:.3}μs", t);
}

fn get_sum_of_priorities(input: &[String]) -> u32 {
    input.iter().map(|v| get_priority(find_bad_item(v))).sum()
}

fn get_sum_of_priorities_group(input: &[String]) -> u32 {
    input
        .iter()
        .chunks(3)
        .into_iter()
        .map(|c| get_priority(get_badge(c.collect::<Vec<&String>>())))
        .sum()
}

fn find_bad_item(values: &str) -> char {
    let (sack_a, sack_b) = values.split_at(values.len() / 2);
    let items_a = sack_a.chars().collect::<HashSet<char>>();
    let items_b = sack_b.chars().collect::<HashSet<char>>();

    if sack_a.len() != sack_a.len() {
        panic!("Not equal: {} != {}", sack_a.len(), sack_b.len());
    }

    let bad = items_a
        .intersection(&items_b)
        .copied()
        .collect::<HashSet<char>>();

    if bad.len() != 1 {
        panic!(
            "More that one bad item: {:?}. a={:?}, b={:?}",
            bad, sack_a, sack_b
        );
    }

    *bad.iter().next().unwrap()
}

fn get_badge(group: Vec<&String>) -> char {
    let group_a = group[0].chars().collect::<HashSet<char>>();
    let group_b = group[1].chars().collect::<HashSet<char>>();
    let group_c = group[2].chars().collect::<HashSet<char>>();

    let common = group_a
        .intersection(&group_b)
        .copied()
        .collect::<HashSet<char>>();
    let common = common
        .intersection(&group_c)
        .copied()
        .collect::<HashSet<char>>();

    if common.len() != 1 {
        panic!("More that one common item.");
    }

    *common.iter().next().unwrap()
}

fn get_priority(value: char) -> u32 {
    if value.is_lowercase() {
        (value as u32) - ('a' as u32) + 1
    } else {
        (value as u32) - ('A' as u32) + 27
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn test_p1() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
"
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(String::from)
        .collect_vec();

        let res = get_sum_of_priorities(&input);

        assert_eq!(res, 157);
    }

    #[test]
    fn test_p2() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
"
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(String::from)
        .collect_vec();

        let res = get_sum_of_priorities_group(&input);

        assert_eq!(res, 70);
    }
}

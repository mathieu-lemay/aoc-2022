use std::fmt::Display;
use std::time::Instant;

use aoc_common::get_input;
use itertools::Itertools;

fn main() {
    let input = get_input("day11.txt");

    let start = Instant::now();

    let (r1, r2) = solve(&input);

    let t = start.elapsed().as_micros() as f64 / 1000.0;

    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
    println!("Duration: {:.3}ms", t);
}

fn solve(input: &[String]) -> (impl Display, impl Display) {
    let mut monkeys = parse_input(input);
    let p1 = get_level(&mut monkeys, 20, true);

    monkeys = parse_input(input);
    let p2 = get_level(&mut monkeys, 10000, false);

    assert_eq!(p1, 58056);
    assert_eq!(p2, 15048718170);

    (p1, p2)
}

struct Monkey {
    items: Vec<u64>,
    op: Box<dyn Fn(u64) -> u64>,
    get_target: Box<dyn Fn(u64) -> usize>,
    modulo: u64,
}

fn parse_input(input: &[String]) -> Vec<Monkey> {
    let mut monkeys = Vec::new();
    let mut it = input.iter().filter(|s| !s.is_empty());

    // Skip Monkey id
    it.next();

    loop {
        let items = it
            .next()
            .unwrap()
            .strip_prefix("  Starting items: ")
            .unwrap()
            .split(", ")
            .map(|i| i.parse().unwrap())
            .collect::<Vec<u64>>();

        let op: Box<dyn Fn(u64) -> u64> = match it
            .next()
            .unwrap()
            .strip_prefix("  Operation: new = ")
            .unwrap()
        {
            "old * old" => Box::new(|v: u64| v * v),
            op_str => {
                if let Some(val) = op_str.strip_prefix("old + ") {
                    let val = val.parse::<u64>().unwrap();
                    Box::new(move |v: u64| v + val)
                } else if let Some(val) = op_str.strip_prefix("old - ") {
                    let val = val.parse::<u64>().unwrap();
                    Box::new(move |v: u64| v - val)
                } else if let Some(val) = op_str.strip_prefix("old * ") {
                    let val = val.parse::<u64>().unwrap();
                    Box::new(move |v: u64| v * val)
                } else if let Some(val) = op_str.strip_prefix("old / ") {
                    let val = val.parse::<u64>().unwrap();
                    Box::new(move |v: u64| v / val)
                } else {
                    panic!("Unable to parse operation: {}", op_str);
                }
            }
        };

        let modulo = it
            .next()
            .unwrap()
            .strip_prefix("  Test: divisible by ")
            .unwrap()
            .parse::<u64>()
            .expect("Invalid modulo");

        let tgt_true = it
            .next()
            .unwrap()
            .strip_prefix("    If true: throw to monkey ")
            .unwrap()
            .parse::<usize>()
            .expect("Invalid 'if true'");

        let tgt_false = it
            .next()
            .unwrap()
            .strip_prefix("    If false: throw to monkey ")
            .unwrap()
            .parse::<usize>()
            .expect("Invalid 'if false'");

        monkeys.push(Monkey {
            items,
            op,
            get_target: Box::new(move |v| if v == 0 { tgt_true } else { tgt_false }),
            modulo,
        });

        if it.next().is_none() {
            break;
        }
    }

    monkeys
}

fn get_level(monkeys: &mut [Monkey], nb_rounds: usize, has_relief: bool) -> u64 {
    let mut inspections = vec![0; monkeys.len()];

    let master_modulo: u64 = monkeys.iter().map(|m| m.modulo).product();

    for _ in 0..nb_rounds {
        for id in 0..monkeys.len() {
            let items = monkeys[id].items.to_vec();
            monkeys[id].items.clear();

            for item in items {
                inspections[id] += 1;

                let (v, tgt) = {
                    let m = &monkeys[id];

                    let mut v = (m.op)(item) % master_modulo;

                    if has_relief {
                        v /= 3;
                    }

                    let tgt = (m.get_target)(v % m.modulo);
                    (v, tgt)
                };

                monkeys[tgt].items.push(v);
            }
        }
    }

    inspections.iter().sorted().rev().take(2).product()
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    const TEST_INPUT: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
";

    #[test]
    fn test_p1() {
        let input = TEST_INPUT
            .split('\n')
            .filter(|s| !s.is_empty())
            .map(String::from)
            .collect_vec();

        let mut monkeys = parse_input(&input);

        assert_eq!(get_level(&mut monkeys, 20, true), 10605);
    }

    #[test]
    fn test_p2() {
        let input = TEST_INPUT
            .split('\n')
            .filter(|s| !s.is_empty())
            .map(String::from)
            .collect_vec();

        let mut monkeys = parse_input(&input);

        assert_eq!(get_level(&mut monkeys, 10000, false), 2713310158);
    }
}

use std::collections::HashMap;
use std::fmt::Display;
use std::time::Instant;

use aoc_common::get_input;
use itertools::Itertools;

fn main() {
    let input = get_input("day21.txt");

    let start = Instant::now();

    let (r1, r2) = solve(input.as_slice());

    let t = start.elapsed().as_micros() as f64 / 1000.0;

    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
    println!("Duration: {:.3}ms", t);
}

fn solve(input: &[String]) -> (impl Display, impl Display) {
    let monkeys = parse_monkeys(input);
    let p1 = get_root_number(&monkeys);
    let p2 = get_equality_number(&monkeys);

    let p2 = format!("Have fun solving that shit:\n{}", p2);

    assert_eq!(p1, 276156919469632);
    // assert_eq!(p2, 22646);

    (p1, p2)
}

fn parse_monkeys(input: &[String]) -> HashMap<String, Monkey> {
    let mut map = HashMap::with_capacity(input.len());

    for line in input {
        let (id, val) = match line.split_once(": ") {
            Some((i, j)) => (i, j),
            None => panic!("Error parsing {}", line),
        };

        let op = if let Ok(n) = val.parse::<u64>() {
            Monkey::Num(n)
        } else {
            let (id1, o, id2) = val.split(" ").collect_tuple().unwrap();

            match o {
                "+" => Monkey::Op(
                    id1.to_string(),
                    id2.to_string(),
                    Box::new(|a, b| a + b),
                    o.to_string(),
                ),
                "-" => Monkey::Op(
                    id1.to_string(),
                    id2.to_string(),
                    Box::new(|a, b| a - b),
                    o.to_string(),
                ),
                "*" => Monkey::Op(
                    id1.to_string(),
                    id2.to_string(),
                    Box::new(|a, b| a * b),
                    o.to_string(),
                ),
                "/" => Monkey::Op(
                    id1.to_string(),
                    id2.to_string(),
                    Box::new(|a, b| a / b),
                    o.to_string(),
                ),
                _ => panic!("Unsupported operator: {}", o),
            }
        };

        map.insert(id.to_string(), op);
    }

    map
}

fn get_root_number(monkeys: &HashMap<String, Monkey>) -> u64 {
    get_number("root", monkeys)
}

fn get_number(id: &str, monkeys: &HashMap<String, Monkey>) -> u64 {
    let monkey = monkeys.get(id).unwrap();

    match monkey {
        Monkey::Num(n) => *n,
        Monkey::Op(a, b, op, _) => {
            let va = get_number(a, monkeys);
            let vb = get_number(b, monkeys);

            op(va, vb)
        }
    }
}

fn get_equality_number(monkeys: &HashMap<String, Monkey>) -> String {
    let root = monkeys.get("root").unwrap();

    if let Monkey::Op(a, b, _, _) = root {
        let eq1 = get_equation(a, monkeys);
        let eq2 = get_equation(b, monkeys);

        format!("{} = {}", eq1, eq2)
    } else {
        panic!("panik")
    }
}

fn get_equation(id: &str, monkeys: &HashMap<String, Monkey>) -> String {
    if id == "humn" {
        return String::from("x");
    }

    let monkey = monkeys.get(id).unwrap();

    match monkey {
        Monkey::Num(n) => format!("{}", n),
        Monkey::Op(a, b, op, s) => {
            let va = get_equation(a, monkeys);
            let vb = get_equation(b, monkeys);

            let na = va.parse::<u64>();
            let nb = vb.parse::<u64>();

            if na.is_ok() && nb.is_ok() {
                format!("{}", op(na.unwrap(), nb.unwrap()))
            } else {
                format!("({} {} {})", va, s, vb)
            }
        }
    }
}

enum Monkey {
    Num(u64),
    Op(String, String, Box<dyn Fn(u64, u64) -> u64>, String),
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    const TEST_INPUT: &str = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";

    #[test]
    fn test_p1() {
        let input = TEST_INPUT
            .split('\n')
            .filter(|s| !s.is_empty())
            .map(String::from)
            .collect_vec();

        let monkeys = parse_monkeys(&input);

        assert_eq!(get_root_number(&monkeys), 152);
    }

    #[test]
    fn test_p2() {
        let input = TEST_INPUT
            .split('\n')
            .filter(|s| !s.is_empty())
            .map(String::from)
            .collect_vec();

        let monkeys = parse_monkeys(&input);

        assert_eq!(
            get_equality_number(&monkeys),
            "((4 + (2 * (x - 3))) / 4) = 150"
        );
    }
}

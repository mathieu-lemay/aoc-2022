use itertools::Itertools;
use std::collections::VecDeque;
use std::fmt::Display;
use std::time::Instant;

use aoc_common::get_input;

fn solve(input: &[String]) -> (impl Display, impl Display) {
    let (mut towers, moves) = parse_input(&input.iter().map(|s| s.as_str()).collect_vec());

    let p1 = process_moves(&mut towers.clone(), &moves);
    let p2 = process_moves_9001(&mut towers, &moves);

    (p1, p2)
}

fn main() {
    let input = get_input("day05.txt");

    let start = Instant::now();

    let (r1, r2) = solve(input.as_slice());

    let t = start.elapsed().as_nanos() as f64 / 1000.0;

    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
    println!("Duration: {:.3}μs", t);
}

#[derive(Debug, PartialEq, Eq)]
struct Move {
    n: usize,
    from: usize,
    to: usize,
}

fn parse_input(input: &[&str]) -> (Vec<VecDeque<char>>, Vec<Move>) {
    let nb_towers = input[0].len() / 4 + 1;
    let mut towers: Vec<VecDeque<char>> = Vec::with_capacity(nb_towers);

    for _ in 0..nb_towers {
        towers.push(VecDeque::new());
    }

    let mut idx: usize = 0;

    loop {
        let s = input[idx];
        idx += 1;

        for (i, tower) in towers.iter_mut().enumerate() {
            if let Some(c) = s.chars().nth(i * 4 + 1) {
                if c.is_alphabetic() {
                    tower.push_back(c);
                }
            }
        }

        if s.is_empty() {
            break;
        }
    }

    let mut moves = Vec::new();

    loop {
        if idx >= input.len() {
            break;
        }

        let (n, from, to) = input[idx]
            .split(' ')
            .map(|i| i.parse::<usize>().unwrap_or(0))
            .filter(|&i| i != 0)
            .collect_tuple()
            .unwrap();

        moves.push(Move { n, from, to });

        idx += 1;
    }

    (towers, moves)
}

fn process_moves(towers: &mut [VecDeque<char>], moves: &[Move]) -> String {
    for m in moves {
        for _ in 0..m.n {
            let c = towers[m.from - 1].pop_front().unwrap();
            towers[m.to - 1].push_front(c)
        }
    }

    towers.iter().map(|t| t[0]).collect::<String>()
}

fn process_moves_9001(towers: &mut [VecDeque<char>], moves: &[Move]) -> String {
    for m in moves {
        let mut tmp = VecDeque::new();

        for _ in 0..m.n {
            let c = towers[m.from - 1].pop_front().unwrap();
            tmp.push_front(c);
        }

        for c in tmp.iter() {
            towers[m.to - 1].push_front(*c)
        }
    }

    towers.iter().map(|t| t[0]).collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
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

        let (towers, moves) = parse_input(&input);
        assert_eq!(towers.len(), 3);
        assert_eq!(towers[0], vec!('N', 'Z'));
        assert_eq!(towers[1], vec!('D', 'C', 'M'));
        assert_eq!(towers[2], vec!('P'));

        assert_eq!(
            moves,
            vec!(
                Move {
                    n: 1,
                    from: 2,
                    to: 1
                },
                Move {
                    n: 3,
                    from: 1,
                    to: 3
                },
                Move {
                    n: 2,
                    from: 2,
                    to: 1
                },
                Move {
                    n: 1,
                    from: 1,
                    to: 2
                },
            )
        )
    }

    #[test]
    fn test_p1() {
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

        let res = process_moves(&mut towers, &moves);

        assert_eq!(res, "CMZ");
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

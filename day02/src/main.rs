use std::fmt::Display;
use std::time::Instant;

use aoc_common::get_input;

fn solve(input: &[String]) -> (impl Display, impl Display) {
    let p1 = get_score(input);
    let p2 = get_score_fixed(input);

    (p1, p2)
}

#[derive(Clone, PartialEq, Eq)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

fn get_score(input: &[String]) -> u32 {
    let mut score = 0;

    for round in input {
        let opponent = get_move(round.chars().next().unwrap());
        let player = get_move(round.chars().nth(2).unwrap());
        score += get_move_score(&player) + get_round_score(&opponent, &player);
    }

    score
}

fn get_score_fixed(input: &[String]) -> u32 {
    let mut score = 0;

    for round in input {
        let opponent = get_move(round.chars().next().unwrap());
        let player = determine_move(&opponent, round.chars().nth(2).unwrap());
        score += get_move_score(&player) + get_round_score(&opponent, &player);
    }

    score
}

fn get_move(code: char) -> Move {
    match code {
        'A' | 'X' => Move::Rock,
        'B' | 'Y' => Move::Paper,
        'C' | 'Z' => Move::Scissors,
        _ => panic!("Invalid move"),
    }
}

fn determine_move(opponent: &Move, outcome: char) -> Move {
    match outcome {
        'X' => match opponent {
            Move::Rock => Move::Scissors,
            Move::Paper => Move::Rock,
            Move::Scissors => Move::Paper,
        },
        'Y' => opponent.clone(),
        'Z' => match opponent {
            Move::Rock => Move::Paper,
            Move::Paper => Move::Scissors,
            Move::Scissors => Move::Rock,
        },
        _ => panic!("Invalid outcome"),
    }
}

fn get_move_score(move_: &Move) -> u32 {
    match move_ {
        Move::Rock => 1,
        Move::Paper => 2,
        Move::Scissors => 3,
    }
}

fn get_round_score(opponent: &Move, player: &Move) -> u32 {
    if opponent == player {
        return 3;
    }

    match (opponent, player) {
        (Move::Rock, Move::Paper) => 6,
        (Move::Paper, Move::Scissors) => 6,
        (Move::Scissors, Move::Rock) => 6,
        _ => 0,
    }
}

fn main() {
    let input = get_input("day02.txt");

    let start = Instant::now();

    let (r1, r2) = solve(input.as_slice());

    let t = start.elapsed().as_nanos() as f64 / 1000.0;

    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
    println!("Duration: {:.3}μs", t);
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn test_p1() {
        let input = "A Y
B X
C Z
"
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(String::from)
        .collect_vec();

        let res = get_score(&input);

        assert_eq!(res, 15);
    }

    #[test]
    fn test_p2() {
        let input = "A Y
B X
C Z
"
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(String::from)
        .collect_vec();

        let res = get_score_fixed(&input);

        assert_eq!(res, 12);
    }
}

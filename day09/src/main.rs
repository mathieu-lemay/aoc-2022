use std::cmp::Ordering;
use std::collections::HashSet;
use std::fmt::Display;
use std::time::Instant;

use aoc_common::get_input;

fn solve(input: &[String]) -> (impl Display, impl Display) {
    let motions = parse_motions(input);

    let p1 = get_visited_positions(&motions, 1);
    let p2 = get_visited_positions(&motions, 9);

    assert_eq!(p1, 6209);
    assert_eq!(p2, 2460);

    (p1, p2)
}

fn main() {
    let input = get_input("day09.txt");

    let start = Instant::now();

    let (r1, r2) = solve(&input);

    let t = start.elapsed().as_nanos() as f64 / 1000.0;

    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
    println!("Duration: {:.3}μs", t);
}

#[derive(Debug)]
enum Motion {
    Up(u8),
    Down(u8),
    Left(u8),
    Right(u8),
}

impl Motion {
    fn tick_coords(&self) -> (i16, i16) {
        match self {
            Self::Up(_) => (0, 1),
            Self::Down(_) => (0, -1),
            Self::Left(_) => (-1, 0),
            Self::Right(_) => (1, 0),
        }
    }

    fn len(&self) -> u8 {
        *match self {
            Self::Up(n) => n,
            Self::Down(n) => n,
            Self::Left(n) => n,
            Self::Right(n) => n,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Position(i16, i16);

impl Position {
    fn tick(&mut self, x: i16, y: i16) {
        self.0 += x;
        self.1 += y;
    }
}

struct Boundaries {
    u: i16,
    d: i16,
    l: i16,
    r: i16,
}

impl Boundaries {
    fn update(&mut self, pos: &Position) {
        if pos.0 > self.r {
            self.r = pos.0;
        } else if pos.0 < self.l {
            self.l = pos.0;
        }

        if pos.1 > self.u {
            self.u = pos.1;
        } else if pos.1 < self.d {
            self.d = pos.1;
        }
    }
}

impl From<&str> for Motion {
    fn from(s: &str) -> Self {
        let (dir, count) = s.split_once(' ').unwrap();
        let count = count.parse().unwrap();

        match dir {
            "U" => Motion::Up(count),
            "D" => Motion::Down(count),
            "L" => Motion::Left(count),
            "R" => Motion::Right(count),
            _ => panic!("Invalid direction: {}", dir),
        }
    }
}

fn get_visited_positions(motions: &Vec<Motion>, knots: usize) -> usize {
    let mut head = Position(0, 0);
    let mut tails = Vec::with_capacity(knots);
    let mut visited = HashSet::new();

    for _ in 0..knots {
        tails.push(Position(0, 0));
    }

    let mut boundaries = Boundaries {
        l: 0,
        d: 0,
        r: 5,
        u: 5,
    };

    #[cfg(test)]
    println!("== Initial State ==\n");

    render(&boundaries, &head, &tails, &visited);

    visited.insert(tails[knots - 1].clone());

    for motion in motions {
        #[cfg(test)]
        println!("== {:?} ==\n", motion);

        let tick_coords = motion.tick_coords();

        for _ in 0..motion.len() {
            head.tick(tick_coords.0, tick_coords.1);

            let m = get_move(&head, &tails[0]);
            tails[0].0 += m.0;
            tails[0].1 += m.1;

            for i in 1..knots {
                let m = get_move(&tails[i - 1], &tails[i]);
                tails[i].0 += m.0;
                tails[i].1 += m.1;
            }

            visited.insert(tails[knots - 1].clone());

            boundaries.update(&head);

            render(&boundaries, &head, &tails, &visited);
        }
    }

    visited.len()
}

fn get_move(head: &Position, tail: &Position) -> (i16, i16) {
    if (head.0 - tail.0).abs() <= 1 && (head.1 - tail.1).abs() <= 1 {
        return (0, 0);
    }

    let x = match head.0.cmp(&tail.0) {
        Ordering::Greater => 1,
        Ordering::Less => -1,
        Ordering::Equal => 0,
    };

    let y = match head.1.cmp(&tail.1) {
        Ordering::Greater => 1,
        Ordering::Less => -1,
        Ordering::Equal => 0,
    };

    (x, y)
}

#[cfg(test)]
fn render(
    boundaries: &Boundaries,
    head: &Position,
    tails: &[Position],
    visited: &HashSet<Position>,
) {
    for y in (boundaries.d..=boundaries.u).rev() {
        for x in boundaries.l..=boundaries.r {
            let cur = Position(x, y);

            let knot = tails.iter().position(|k| k == &cur);

            if &cur == head {
                print!("H");
            } else if knot.is_some() {
                print!("{}", knot.unwrap() + 1);
            } else if cur.0 == 0 && cur.1 == 0 {
                print!("s");
            } else if visited.contains(&cur) {
                print!("#");
            } else {
                print!(".");
            }
        }

        println!();
    }
    println!();
}

#[cfg(not(test))]
fn render(
    _boundaries: &Boundaries,
    _head: &Position,
    _tails: &[Position],
    _visited: &HashSet<Position>,
) {
}

fn parse_motions(input: &[String]) -> Vec<Motion> {
    input
        .iter()
        .map(|s| s.as_str().into())
        .collect::<Vec<Motion>>()
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    const TEST_INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
";

    #[test]
    fn test_p1() {
        let input = TEST_INPUT
            .split('\n')
            .filter(|s| !s.is_empty())
            .map(String::from)
            .collect_vec();

        let motions = parse_motions(&input);

        assert_eq!(get_visited_positions(&motions, 1), 13);
    }

    #[test]
    fn test_p2() {
        let input = TEST_INPUT
            .split('\n')
            .filter(|s| !s.is_empty())
            .map(String::from)
            .collect_vec();

        let motions = parse_motions(&input);

        assert_eq!(get_visited_positions(&motions, 9), 1);
    }

    #[test]
    fn test_p2_2() {
        let input = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"
            .split('\n')
            .filter(|s| !s.is_empty())
            .map(String::from)
            .collect_vec();

        let motions = parse_motions(&input);

        assert_eq!(get_visited_positions(&motions, 9), 36);
    }
}

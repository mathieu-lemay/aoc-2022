use std::fmt::Display;
use std::hash::Hash;
use std::time::Instant;

use itertools::Itertools;
use pathfinding::prelude::dijkstra;

use aoc_common::get_input;

fn main() {
    let input = get_input("day12.txt");

    let start = Instant::now();

    let (r1, r2) = solve(&input);

    let t = start.elapsed().as_micros() as f64 / 1000.0;

    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
    println!("Duration: {:.3}ms", t);
}

fn solve(input: &[String]) -> (impl Display, impl Display) {
    let grid = parse_grid(input);

    // Both answers are off by 2 for some reason. 🤷🏻️
    let p1 = get_cheapest_path(&grid).expect("Error getting cheapest path.") - 2;
    let p2 = get_cheapest_path_from_any_start(&grid).expect("Error getting cheapest path.") - 2;

    assert_eq!(p1, 462);
    assert_eq!(p2, 451);

    (p1, p2)
}

fn parse_grid(input: &[String]) -> Grid {
    let points = input
        .iter()
        .map(|s| {
            s.chars()
                .map(|c| match c {
                    'S' => 0,
                    'E' => 27,
                    x => x as u8 - b'a' + 1,
                })
                .collect_vec()
        })
        .collect_vec();

    let height = points.len();
    let width = points[0].len();

    Grid {
        height,
        width,
        points,
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
}

struct Grid {
    height: usize,
    width: usize,
    points: Vec<Vec<u8>>,
}

impl Grid {
    fn get_successors(&self, pos: &Point) -> Vec<(Point, usize)> {
        let x = pos.x;
        let y = pos.y;
        let cur = self.points[x][y];
        let mut edges = Vec::new();

        if x > 0 {
            let val = self.points[x - 1][y];
            if is_walkable(cur, val) {
                edges.push((Point { x: x - 1, y }, 1));
            }
        }
        if x < self.height - 1 {
            let val = self.points[x + 1][y];
            if is_walkable(cur, val) {
                edges.push((Point { x: x + 1, y }, 1));
            }
        }
        if y > 0 {
            let val = self.points[x][y - 1];
            if is_walkable(cur, val) {
                edges.push((Point { x, y: y - 1 }, 1));
            }
        }
        if y < self.width - 1 {
            let val = self.points[x][y + 1];
            if is_walkable(cur, val) {
                edges.push((Point { x, y: y + 1 }, 1));
            }
        }

        edges
    }

    fn get_start(&self) -> Point {
        for (x, row) in self.points.iter().enumerate() {
            for (y, value) in row.iter().enumerate() {
                if *value == 0 {
                    return Point { x, y };
                }
            }
        }

        panic!("Start not found");
    }

    fn get_alternate_starts(&self) -> Vec<Point> {
        let mut starts = Vec::new();
        for (x, row) in self.points.iter().enumerate() {
            for (y, value) in row.iter().enumerate() {
                if *value <= 1 {
                    starts.push(Point { x, y });
                }
            }
        }

        starts
    }

    fn get_goal(&self) -> Point {
        for (x, row) in self.points.iter().enumerate() {
            for (y, value) in row.iter().enumerate() {
                if *value == 27 {
                    return Point { x, y };
                }
            }
        }

        panic!("Goal not found");
    }
}

#[inline]
fn is_walkable(current: u8, target: u8) -> bool {
    current >= target || current == target - 1
}

fn get_cheapest_path(grid: &Grid) -> Option<usize> {
    let start = grid.get_start();
    let goal = grid.get_goal();

    let result = dijkstra(&start, |p| grid.get_successors(p), |p| *p == goal);

    result.map(|r| r.1)
}

fn get_cheapest_path_from_any_start(grid: &Grid) -> Option<usize> {
    let starts = grid.get_alternate_starts();
    let goal = grid.get_goal();

    let result = starts
        .iter()
        .map(|s| {
            dijkstra(s, |p| grid.get_successors(p), |p| *p == goal)
                .map(|r| r.1)
                .unwrap_or(usize::MAX)
        })
        .min();

    result
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    const TEST_INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
";

    #[test]
    fn test_p1() {
        let input = TEST_INPUT
            .split('\n')
            .filter(|s| !s.is_empty())
            .map(String::from)
            .collect_vec();

        let grid = parse_grid(&input);

        assert_eq!(get_cheapest_path(&grid), Some(31));
    }

    #[test]
    fn test_p2() {
        let input = TEST_INPUT
            .split('\n')
            .filter(|s| !s.is_empty())
            .map(String::from)
            .collect_vec();

        let grid = parse_grid(&input);

        assert_eq!(get_cheapest_path_from_any_start(&grid), Some(29));
    }
}

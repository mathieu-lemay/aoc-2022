use itertools::Itertools;
use std::cmp;

use crate::{Element, Grid};
use aoc_common::Point;

pub(super) fn solve(input: &[String]) -> usize {
    let mut sum = 0;

    let mut grid = parse_input(input);
    grid.render();

    loop {
        let res = pour_sand(&mut grid);
        grid.render();

        if !res {
            break;
        }

        sum += 1;
    }

    sum
}

fn parse_input(input: &[String]) -> Grid {
    let mut points = Vec::new();

    for s in input {
        let values: Vec<Point> = s
            .split(" -> ")
            .map(|vs| {
                let (x, y) = vs
                    .split(',')
                    .map(|v| v.parse::<usize>().unwrap())
                    .collect_tuple()
                    .unwrap();

                Point { x, y }
            })
            .collect_vec();

        points.push(values);
    }

    get_grid(points)
}

fn get_grid(edge_groups: Vec<Vec<Point>>) -> Grid {
    let max_y = edge_groups
        .iter()
        .map(|v| v.iter().map(|e| e.y).max().unwrap())
        .max()
        .unwrap();

    let h = max_y + 1 + 2;
    let w = h * 2 + 1;
    let offset_x = 500 - h;

    let mut grid = Grid::new(w, h, offset_x);

    for edges in edge_groups {
        for i in 0..(edges.len() - 1) {
            let a = &edges[i];
            let b = &edges[i + 1];

            grid.set(a.x, a.y, Element::Rock);
            grid.set(b.x, b.y, Element::Rock);

            if a.x == b.x {
                for y in (cmp::min(a.y, b.y) + 1)..(cmp::max(a.y, b.y)) {
                    grid.set(a.x, y, Element::Rock);
                }
            } else if a.y == b.y {
                for x in (cmp::min(a.x, b.x) + 1)..(cmp::max(a.x, b.x)) {
                    grid.set(x, a.y, Element::Rock);
                }
            }
        }
    }

    for x in 0..w {
        grid.set(x + offset_x, h - 1, Element::Rock);
    }

    grid
}

fn pour_sand(grid: &mut Grid) -> bool {
    let mut x = 500;
    let mut y = 0;

    if grid.get(500, 0) == Element::Sand {
        return false;
    }

    loop {
        for cy in y..(grid.height - 1) {
            let e = grid.get(x, cy + 1);
            if e != Element::Air {
                y = cy;
                break;
            }
        }

        let down_left = grid.get(x - 1, y + 1);
        if down_left == Element::Air {
            x -= 1;
            continue;
        }

        let down_right = grid.get(x + 1, y + 1);
        if down_right == Element::Air {
            x += 1;
            continue;
        }

        break;
    }

    grid.set(x, y, Element::Sand);

    true
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    const TEST_INPUT: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn test_solve() {
        let input = TEST_INPUT
            .split('\n')
            .filter(|s| !s.is_empty())
            .map(String::from)
            .collect_vec();

        assert_eq!(solve(&input), 93);
    }
}

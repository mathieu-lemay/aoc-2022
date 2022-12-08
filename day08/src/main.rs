use std::fmt::Display;
use std::time::Instant;

use aoc_common::get_input;
use itertools::Itertools;

fn solve(input: &[String]) -> (impl Display, impl Display) {
    let p1 = get_number_of_visible_trees(input);
    let p2 = get_best_scenic_score(input);

    assert_eq!(p1, 1713);
    assert_eq!(p2, 268464);

    (p1, p2)
}

fn main() {
    let input = get_input("day08.txt");

    let start = Instant::now();

    let (r1, r2) = solve(&input);

    let t = start.elapsed().as_nanos() as f64 / 1000.0;

    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
    println!("Duration: {:.3}μs", t);
}

#[derive(Debug)]
struct Tree {
    height: i8,
    visible: bool,
}

type Forest = Vec<Vec<Tree>>;

fn get_number_of_visible_trees(input: &[String]) -> usize {
    let mut forest = input
        .iter()
        .map(|s| {
            s.chars()
                .map(|c| Tree {
                    height: c as i8 - '0' as i8,
                    visible: false,
                })
                .collect_vec()
        })
        .collect_vec();

    let w = forest[0].len();
    let h = forest.len();

    for r in forest.iter_mut() {
        let mut max = -1;

        for mut tree in r.iter_mut() {
            if tree.height > max {
                tree.visible = true;
                max = tree.height;
            }
        }

        max = -1;

        for mut tree in r.iter_mut().rev() {
            if tree.height > max {
                tree.visible = true;
                max = tree.height;
            }
        }
    }

    for x in 0..w {
        let mut max = -1;

        for row in forest.iter_mut() {
            let mut tree = &mut row[x];
            if tree.height > max {
                tree.visible = true;
                max = tree.height;
            }
        }

        max = -1;

        for y in (0..h).rev() {
            let mut tree = &mut forest[y][x];
            if tree.height > max {
                tree.visible = true;
                max = tree.height;
            }
        }
    }

    let visible: usize = forest
        .into_iter()
        .map(|r| r.iter().filter(|t| t.visible).count())
        .sum();

    visible
}

fn get_best_scenic_score(input: &[String]) -> usize {
    let forest = parse_forest(input);

    let w = forest[0].len();
    let h = forest.len();

    let mut score: usize = 0;

    for x in 1..w - 1 {
        for y in 1..h - 1 {
            let s = get_scenic_score(&forest, x, y, w, h);

            if s > score {
                score = s;
            }
        }
    }

    score
}

fn get_scenic_score(forest: &Forest, x: usize, y: usize, w: usize, h: usize) -> usize {
    let mut sn = 0;
    let mut ss = 0;
    let mut se = 0;
    let mut sw = 0;

    let ref_height = forest[y][x].height;

    for i in (0..y).rev() {
        sn = y - i;
        if forest[i][x].height >= ref_height {
            break;
        }
    }

    #[allow(clippy::needless_range_loop)]
    for i in y + 1..h {
        ss = i - y;
        if forest[i][x].height >= ref_height {
            break;
        }
    }

    for i in (0..x).rev() {
        sw = x - i;
        if forest[y][i].height >= ref_height {
            break;
        }
    }

    for i in x + 1..w {
        se = i - x;
        if forest[y][i].height >= ref_height {
            break;
        }
    }

    sn * ss * se * sw
}

fn parse_forest(input: &[String]) -> Forest {
    input
        .iter()
        .map(|s| {
            s.chars()
                .map(|c| Tree {
                    height: c as i8 - '0' as i8,
                    visible: false,
                })
                .collect_vec()
        })
        .collect_vec()
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    const TEST_INPUT: &str = "30373
25512
65332
33549
35390
";

    #[test]
    fn test_p1() {
        let input = TEST_INPUT
            .split('\n')
            .filter(|s| !s.is_empty())
            .map(String::from)
            .collect_vec();

        assert_eq!(get_number_of_visible_trees(&input), 21);
    }

    #[test]
    fn test_p2() {
        let input = TEST_INPUT
            .split('\n')
            .filter(|s| !s.is_empty())
            .map(String::from)
            .collect_vec();

        assert_eq!(get_best_scenic_score(&input), 8);
    }

    #[test]
    fn test_get_scenic_score() {
        let input = TEST_INPUT
            .split('\n')
            .filter(|s| !s.is_empty())
            .map(String::from)
            .collect_vec();

        let forest = parse_forest(&input);

        assert_eq!(get_scenic_score(&forest, 2, 1, 5, 5), 4);
        assert_eq!(get_scenic_score(&forest, 2, 3, 5, 5), 8);
    }
}

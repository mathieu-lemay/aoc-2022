use std::fmt::Display;
use std::time::Instant;

use aoc_common::get_input;

mod part1;
mod part2;

fn main() {
    let input = get_input("day14.txt");

    let start = Instant::now();

    let (r1, r2) = solve(input.as_slice());

    let t = start.elapsed().as_micros() as f64 / 1000.0;

    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
    println!("Duration: {:.3}ms", t);
}

fn solve(input: &[String]) -> (impl Display, impl Display) {
    let p1 = part1::solve(input);
    let p2 = part2::solve(input);

    assert_eq!(p1, 1513);
    assert_eq!(p2, 22646);

    (p1, p2)
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Element {
    Air,
    Sand,
    Rock,
    Void,
}

struct Grid {
    elements: Vec<Vec<Element>>,
    width: usize,
    height: usize,
    offset_x: usize,
}

impl Grid {
    fn new(w: usize, h: usize, offset_x: usize) -> Self {
        let mut elements = Vec::with_capacity(h);

        for _ in 0..h {
            elements.push(vec![Element::Air; w]);
        }

        Grid {
            elements,
            width: w,
            height: h,
            offset_x,
        }
    }

    fn get(&self, x: usize, y: usize) -> Element {
        if x < self.offset_x || x - self.offset_x >= self.width || y >= self.height {
            return Element::Void;
        }

        self.elements[y][x - self.offset_x].clone()
    }

    fn set(&mut self, x: usize, y: usize, element: Element) {
        self.elements[y][x - self.offset_x] = element;
    }

    #[cfg(test)]
    fn render(&self) {
        for row in &self.elements {
            let r = row
                .iter()
                .map(|e| match e {
                    Element::Air => '.',
                    Element::Sand => 'o',
                    Element::Rock => '#',
                    Element::Void => ' ',
                })
                .collect::<String>();

            println!("{}", r);
        }
        println!();
    }

    #[cfg(not(test))]
    fn render(&self) {}
}

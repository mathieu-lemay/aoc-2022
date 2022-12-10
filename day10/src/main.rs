use std::fmt::Display;
use std::str::FromStr;
use std::time::Instant;

use aoc_common::get_input;
use itertools::Itertools;

fn main() {
    let input = get_input("day10.txt");

    let start = Instant::now();

    let (r1, r2) = solve(&input);

    let t = start.elapsed().as_nanos() as f64 / 1000.0;

    println!("Part 1: {}", r1);
    println!("Part 2: \n{}", r2);
    println!("Duration: {:.3}μs", t);
}

fn solve(input: &[String]) -> (impl Display, impl Display) {
    let instructions = parse_instructions(input);

    let p1 = get_sum_of_signal_strength(&instructions, &[20, 60, 100, 140, 180, 220]);
    let p2 = render_sprites(&instructions, 40, 6);

    (p1, p2)
}

#[derive(Debug)]
enum Instruction {
    Noop,
    AddX(i8),
}

impl Instruction {
    fn n_cycles(&self) -> u8 {
        match self {
            Instruction::Noop => 1,
            Instruction::AddX(_) => 2,
        }
    }
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens = s.split(' ').collect::<Vec<&str>>();

        match tokens[0] {
            "noop" => Ok(Instruction::Noop),
            "addx" => Ok(Instruction::AddX(tokens[1].parse().unwrap())),
            _ => Err(format!("Invalid instruction: {}", tokens[0])),
        }
    }
}

fn parse_instructions(input: &[String]) -> Vec<Instruction> {
    input.iter().map(|i| i.parse().unwrap()).collect_vec()
}

fn get_sum_of_signal_strength(instructions: &[Instruction], cycles: &[u32]) -> i32 {
    let mut x = 1i32;
    let mut sum = 0i32;
    let total_cycles = *cycles.iter().max().unwrap();

    let mut instr_iter = instructions.iter().cycle();

    let mut instr = instr_iter.next().unwrap();
    let mut instr_cycles = instr.n_cycles();

    for cycle in 1..=total_cycles {
        instr_cycles -= 1;

        if cycles.contains(&cycle) {
            sum += x * cycle as i32;
            #[cfg(test)]
            println!(
                "Cycle={}, x={}, Sum={}, instr_cycles={}",
                cycle, x, sum, instr_cycles
            );
        }

        if instr_cycles == 0 {
            #[cfg(test)]
            println!("Cycle={}, Instr={:?}", cycle, instr);
            if let Instruction::AddX(n) = instr {
                x += *n as i32;
            }

            instr = instr_iter.next().unwrap();
            instr_cycles = instr.n_cycles();
        }
    }

    sum
}

fn render_sprites(instructions: &[Instruction], w: u8, h: u8) -> String {
    let mut x = 1i32;

    let mut instr_iter = instructions.iter().cycle();

    let mut instr = instr_iter.next().unwrap();
    let mut instr_cycles = instr.n_cycles();

    let mut chars = Vec::with_capacity((w as usize + 1) * h as usize);

    for cycle in 0..(w * h) {
        let pos = cycle % w;

        if cycle > 0 && pos == 0 {
            chars.push('\n');
        }

        if (x - pos as i32).abs() <= 1 {
            chars.push('#');
        } else {
            chars.push('.');
        }

        instr_cycles -= 1;
        if instr_cycles == 0 {
            #[cfg(test)]
            println!("Cycle={}, Instr={:?}", cycle, instr);
            if let Instruction::AddX(n) = instr {
                x += *n as i32;
            }

            instr = instr_iter.next().unwrap();
            instr_cycles = instr.n_cycles();
        }
    }
    chars.push('\n');

    chars.iter().collect()
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    const TEST_INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn test_p1() {
        let input = TEST_INPUT
            .split('\n')
            .filter(|s| !s.is_empty())
            .map(String::from)
            .collect_vec();

        let instructions = parse_instructions(&input);

        assert_eq!(
            get_sum_of_signal_strength(&instructions, &vec![20, 60, 100, 140, 180, 220]),
            13140
        );
    }

    #[test]
    fn test_p2() {
        let input = TEST_INPUT
            .split('\n')
            .filter(|s| !s.is_empty())
            .map(String::from)
            .collect_vec();

        let instructions = parse_instructions(&input);

        let res = render_sprites(&instructions, 40, 6);
        let expected = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
";

        assert_eq!(res, expected);
    }
}

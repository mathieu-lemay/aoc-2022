use std::collections::HashMap;
use std::fmt::Display;
use std::path::PathBuf;
use std::time::Instant;

use aoc_common::get_input;

fn solve(input: &[String]) -> (impl Display, impl Display) {
    let files = parse_directory_structure(input);
    let p1 = get_sum_of_small_directories(&files);
    let p2 = get_size_of_dir_to_delete(&files);

    (p1, p2)
}

fn main() {
    let input = get_input("day07.txt");

    let start = Instant::now();

    let (r1, r2) = solve(&input);

    let t = start.elapsed().as_nanos() as f64 / 1000.0;

    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
    println!("Duration: {:.3}μs", t);
}

struct File {
    path: PathBuf,
    size: usize,
}

fn parse_directory_structure(input: &[String]) -> Vec<File> {
    let mut files = Vec::new();
    let mut path = PathBuf::from("/");

    let mut cmd_iter = input.iter();

    loop {
        let line = cmd_iter.next();

        if line.is_none() {
            break;
        }

        let cmd = line.unwrap();

        if let Some(cd_arg) = cmd.strip_prefix("$ cd ") {
            if cd_arg.starts_with('/') {
                path = PathBuf::from(cd_arg);
            } else if cd_arg == ".." {
                path.pop();
            } else {
                path.push(cd_arg);
            }
        } else if cmd == "$ ls" || cmd.starts_with("dir ") {
            continue;
        } else {
            let (size, name) = cmd.split_once(' ').unwrap();
            let size: usize = size.parse().unwrap();
            files.push(File {
                path: path.clone().join(name),
                size,
            })
        }
    }

    files
}

fn get_directory_sizes(files: &[File]) -> HashMap<String, usize> {
    let mut dir_sizes: HashMap<String, usize> = HashMap::new();

    for f in files {
        let mut p = f.path.clone();
        p.pop();

        loop {
            let k = p.clone().to_str().unwrap().to_string();
            let s = dir_sizes.get(&k).unwrap_or(&0) + f.size;
            dir_sizes.insert(k, s);

            if p.to_str().unwrap() == "/" {
                break;
            }

            p.pop();
        }
    }

    dir_sizes
}

fn get_sum_of_small_directories(files: &[File]) -> usize {
    get_directory_sizes(files)
        .iter()
        .map(|(_, &v)| v)
        .filter(|&s| s <= 100000)
        .sum()
}

fn get_size_of_dir_to_delete(files: &[File]) -> usize {
    let dir_sizes = get_directory_sizes(files);
    let total_size = dir_sizes.get(&String::from("/")).unwrap();

    dir_sizes
        .iter()
        .map(|(_, &s)| s)
        .filter(|&s| total_size - s <= 40000000)
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    const TEST_INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
";

    #[test]
    fn test_parse_directory_structure() {
        let input = TEST_INPUT
            .split('\n')
            .filter(|s| !s.is_empty())
            .map(String::from)
            .collect_vec();

        let files = parse_directory_structure(&input);

        assert_eq!(files.len(), 10);
    }

    #[test]
    fn test_p1() {
        let input = TEST_INPUT
            .split('\n')
            .filter(|s| !s.is_empty())
            .map(String::from)
            .collect_vec();

        let files = parse_directory_structure(&input);

        assert_eq!(get_sum_of_small_directories(&files), 95437);
    }

    #[test]
    fn test_p2() {
        let input = TEST_INPUT
            .split('\n')
            .filter(|s| !s.is_empty())
            .map(String::from)
            .collect_vec();

        let files = parse_directory_structure(&input);

        assert_eq!(get_size_of_dir_to_delete(&files), 24933642);
    }
}

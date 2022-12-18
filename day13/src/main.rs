use std::cmp::Ordering;
use std::fmt::Display;
use std::time::Instant;

use itertools::Itertools;
use serde_json::Value;

use aoc_common::get_input;

fn main() {
    let input = get_input("day13.txt");

    let start = Instant::now();

    let values = parse_values(&input);

    let (r1, r2) = solve(&values);

    let t = start.elapsed().as_nanos() as f64 / 1000.0;

    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
    println!("Duration: {:.3}μs", t);
}

fn solve(values: &[Value]) -> (impl Display, impl Display) {
    let p1 = get_ordered_pair_index_sum(values);
    let p2 = get_decoder_key(values);

    assert_eq!(p1, 6076);
    assert_eq!(p2, 24805);

    (p1, p2)
}

fn parse_values(input: &[String]) -> Vec<Value> {
    input
        .iter()
        .filter(|s| !s.is_empty())
        .map(|s| serde_json::from_str(s).unwrap())
        .collect_vec()
}

fn get_ordered_pair_index_sum(values: &[Value]) -> usize {
    let mut sum = 0;

    for (idx, mut chunk) in values.iter().chunks(2).into_iter().enumerate() {
        let v1 = chunk.next().unwrap();
        let v2 = chunk.next().unwrap();
        if get_ordering(v1, v2) != Ordering::Greater {
            sum += idx + 1
        }
    }

    sum
}

fn get_decoder_key(values: &[Value]) -> usize {
    let key2 = serde_json::from_str("[[2]]").unwrap();
    let key6 = serde_json::from_str("[[6]]").unwrap();
    let mut count2 = 0;
    let mut count6 = 0;

    for v in values {
        if get_ordering(v, &key2) == Ordering::Less {
            count2 += 1;
            count6 += 1;
        } else if get_ordering(v, &key6) == Ordering::Less {
            count6 += 1;
        }
    }

    (count2 + 1) * (count6 + 2)
}

fn get_ordering(v1: &Value, v2: &Value) -> Ordering {
    if v1.is_number() && v2.is_number() {
        let a = v1.as_u64().unwrap();
        let b = v2.as_u64().unwrap();

        return a.cmp(&b);
    }

    if v1.is_array() && v2.is_array() {
        let arr1 = v1.as_array().unwrap();
        let arr2 = v2.as_array().unwrap();

        for (a, b) in arr1.iter().zip(arr2.iter()) {
            let ord = get_ordering(a, b);

            if ord != Ordering::Equal {
                return ord;
            }
        }

        return arr1.len().cmp(&arr2.len());
    }

    if v1.is_number() {
        let v = vec![v1.clone()];

        return get_ordering(&Value::Array(v), v2);
    }

    if v2.is_number() {
        let v = vec![v2.clone()];

        return get_ordering(v1, &Value::Array(v));
    }

    Ordering::Less
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    const TEST_INPUT: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn test_p1() {
        let input = TEST_INPUT
            .split('\n')
            .filter(|s| !s.is_empty())
            .map(String::from)
            .collect_vec();

        let values = parse_values(&input);

        assert_eq!(get_ordered_pair_index_sum(&values), 13);
    }

    #[test]
    fn test_p2() {
        let input = TEST_INPUT
            .split('\n')
            .filter(|s| !s.is_empty())
            .map(String::from)
            .collect_vec();

        let values = parse_values(&input);

        assert_eq!(get_decoder_key(&values), 140);
    }
}

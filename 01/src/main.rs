use std::collections::HashMap;
use std::iter::*;

use itertools::{Either, Itertools};

fn main() {
    let input = std::fs::read_to_string("input2.txt").unwrap();
    let (left, right): (Vec<i32>, Vec<i32>) = input
        .split_whitespace()
        .filter_map(|s| s.parse::<i32>().ok())
        .enumerate()
        .partition_map(|(i, v)| if i & 1 == 0 { Either::Left(v) } else { Either::Right(v) });
    let part1: i32 = zip(left.iter().sorted(), right.iter().sorted()).map(|(l, r)| (r - l).abs()).sum();
    println!("part1: {}", part1);

    // part2
    let right_map = right.iter().fold(HashMap::new(), |mut acc, c| {
        *acc.entry(c).or_insert(0) += 1;
        acc
    });
    let part2 = left.iter().map(|l| l * right_map.get(l).unwrap_or(&0)).sum::<i32>();
    println!("part2: {}", part2);
}

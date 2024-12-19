use std::{collections::HashMap, fs};

fn main() {
    let filename = "input2.txt";
    let input = fs::read_to_string(filename).unwrap();
    let (strips_str, towels_str) = input.split_once("\n\n").unwrap();
    let mut strips: Vec<&str> = strips_str.split(", ").collect();
    let towels: Vec<&str> = towels_str.split("\n").collect();

    // reduce strips -> remove combinations that can be built of shorter strips
    strips.sort_by_key(|a| a.len());
    let mut reduced_strips: Vec<&str> = strips.iter().copied().filter(|strip| strip.len() == 1).collect();
    let max_length = strips.last().unwrap().len();
    for i in 2..=max_length {
        let strips_with_length: Vec<&str> = strips.iter().copied().filter(|strip| strip.len() == i).collect();
        let mut new_strips: Vec<&str> = Vec::new();
        (0..strips_with_length.len()).for_each(|j| {
            if !starts_with(strips_with_length[j], &reduced_strips) {
                new_strips.push(strips_with_length[j]);
            }
        });
        reduced_strips.extend(new_strips);
    }

    let part1 = towels.iter().filter(|towel| starts_with(towel, &reduced_strips)).count();
    println!("part1: {}", part1);

    // reduced still to slow for part to, reverse search, look from the end, so it's clear early
    // if a towel can be built from the strips and use caching for known pattern
    let mut part2 = 0;
    for (i, towel) in towels.iter().enumerate() {
        let mut cache: HashMap<&str, i64> = HashMap::new();
        part2 += ends_with(towel, &strips, &mut cache);
    }
    println!("part2: {}", part2);
}

fn starts_with(s: &str, strips: &[&str]) -> bool {
    if s.is_empty() {
        return true;
    }

    for prefix in strips.iter() {
        if prefix.len() > s.len() {
            continue;
        }
        if s.starts_with(*prefix) && starts_with(&s[prefix.len()..], strips) {
            return true;
        }
    }

    false
}

fn ends_with<'a>(s: &'a str, strips: &[&str], cache: &mut HashMap<&'a str, i64>) -> i64 {
    if s.is_empty() {
        return 1;
    }

    if let Some(&value) = cache.get(s) {
        return value;
    }

    let mut sum = 0;
    for prefix in strips.iter() {
        if prefix.len() > s.len() {
            continue;
        }

        if s.ends_with(*prefix) {
            let c = crate::ends_with(&s[..s.len() - prefix.len()], strips, cache);
            if c > 0 {
                sum += c;
                cache.insert(s, sum);
            }
        }
    }

    sum
}

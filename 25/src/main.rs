use std::fs;
use std::iter::zip;

fn main() {
    let filename = "input2.txt";
    let input = fs::read_to_string(filename).unwrap();
    let blocks = input.split("\n\n");
    let mut locks: Vec<[i32; 5]> = Vec::new();
    let mut keys: Vec<[i32; 5]> = Vec::new();

    for block in blocks {
        let mut profile = [-1, -1, -1, -1, -1];

        block.lines().enumerate().for_each(|(i, line)| {
            line.chars().enumerate().for_each(|(j, c)| {
                if c == '#' {
                    profile[j] += 1;
                }
            });
        });
        if block.lines().next().unwrap().starts_with(".") {
            keys.push(profile);
        } else {
            locks.push(profile);
        }
    }

    let mut part1 = 0;
    for key in &keys {
        for lock in &locks {
            if zip(key, lock).all(|(k, l)| k + l < 6) {
                part1 += 1;
            }
        }
    }
    println!("part1: {}", part1);
}

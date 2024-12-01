use std::collections::HashMap;
use std::{
    fs::File,
    io::{self, BufRead},
    iter::zip,
};

fn main() {
    let filename = "input2.txt";
    let file = File::open(filename).unwrap();
    let lines: Vec<_> = io::BufReader::new(file).lines().collect::<Result<_, _>>().unwrap();

    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    lines.iter().for_each(|line| {
        //println!("'{}'", line);
        let splits = line.split("  ").collect::<Vec<&str>>();
        left.push(splits[0].trim().parse::<i32>().unwrap());
        right.push(splits[1].trim().parse::<i32>().unwrap());
    });
    left.sort();
    right.sort();

    let mut dists = 0;
    zip(left.iter(), right.iter()).for_each(|(l, r)| {
        //println!("{} {}", l, r);
        dists += (r - l).abs();
    });

    println!("part1: {}", dists);

    let right_map = right.iter().fold(HashMap::new(), |mut acc, c| {
        *acc.entry(c).or_insert(0) += 1;
        acc
    });
    //println!("{:?}", right_map);

    let mut score = 0;
    left.iter().for_each(|l| {
        if let Some(count) = right_map.get(l) {
            score += l * count;
        }
    });
    println!("part2: {}", score);
}

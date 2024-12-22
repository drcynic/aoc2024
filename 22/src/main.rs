use std::collections::{HashMap, HashSet};
use std::fs;

fn main() {
    let filename = "input2.txt";
    let input = fs::read_to_string(filename).unwrap();
    let values: Vec<i64> = input.lines().map(|x| x.parse().unwrap()).collect();
    // println!("{:?}", values);

    let part1: i64 = values
        .iter()
        .map(|v| {
            let mut v = *v;
            for _ in 0..2000 {
                v = process(v);
            }
            v
        })
        .sum();
    println!("Part 1: {}", part1);

    // part 2
    // let values: Vec<i64> = vec![1, 2, 3, 2024];
    let bananas = part2(&values);
    println!("Part 2: {}", bananas);
}

fn part2(values: &Vec<i64>) -> i64 {
    let mut global_unique_sequences: HashSet<[i64; 4]> = HashSet::new();
    let mut unique_sequences_per_secret_number: Vec<HashMap<[i64; 4], i64>> = Vec::new();
    (0..values.len()).for_each(|i| {
        let mut v = values[i];
        let (prices, diffs): (Vec<i64>, Vec<i64>) = (0..2000)
            .map(|_| {
                let prev_price = v % 10;
                v = process(v);
                (prev_price, v % 10 - prev_price)
            })
            .collect();
        let mut unique_sequences: HashMap<[i64; 4], i64> = HashMap::new();
        for n in 0..diffs.len() - 4 {
            let s = &diffs[n..n + 4];
            if unique_sequences.contains_key(&[s[0], s[1], s[2], s[3]]) {
                continue; // only the first occurrence
            }
            unique_sequences.insert([s[0], s[1], s[2], s[3]], prices[n + 4]);
        }
        for k in unique_sequences.keys() {
            global_unique_sequences.insert(*k);
        }
        unique_sequences_per_secret_number.push(unique_sequences);
    });

    global_unique_sequences
        .iter()
        .map(|glob_seq| {
            unique_sequences_per_secret_number
                .iter()
                .map(|secret_seq| secret_seq.get(glob_seq).unwrap_or(&0))
                .sum()
        })
        .max()
        .unwrap()
}

fn process(v: i64) -> i64 {
    let v = ((v << 6) ^ v) % 16777216;
    let v = ((v >> 5) ^ v) % 16777216;
    let v = ((v << 11) ^ v) % 16777216;
    v
}

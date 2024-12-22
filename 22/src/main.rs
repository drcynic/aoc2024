use std::cmp::max;
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
            for n in 0..2000 {
                v = process(v);
            }
            v
        })
        .sum();
    println!("Part 1: {}", part1);

    // part 2
    // let values: Vec<i64> = vec![1, 2, 3, 2024];
    let mut global_unique_sequences: HashSet<[i64; 4]> = HashSet::new();
    let mut unique_sequences_per_secret_number: Vec<HashMap<[i64; 4], i64>> = Vec::new();
    for i in 0..values.len() {
        let mut v = values[i];
        let mut diffs = vec![];
        let mut prices = vec![];
        for n in 0..2000 {
            let new_v = process(v);
            let rs = v.to_string();
            let prev_price = rs[rs.len() - 1..].parse::<i64>().unwrap();
            let ls = new_v.to_string();
            let new_price = ls[ls.len() - 1..].parse::<i64>().unwrap();
            prices.push(prev_price);
            diffs.push(new_price - prev_price);
            v = new_v;
        }
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
    }

    let mut bananas = 0;
    global_unique_sequences.iter().for_each(|seq| {
        let mut local_bananas = 0;
        unique_sequences_per_secret_number.iter().for_each(|unique_sequences| {
            if let Some(v) = unique_sequences.get(seq) {
                local_bananas += v;
            }
        });
        bananas = max(bananas, local_bananas);
    });
    println!("Part 2: {}", bananas);
}

fn process(v: i64) -> i64 {
    let v = ((v << 6) ^ v) % 16777216;
    let v = ((v >> 5) ^ v) % 16777216;
    let v = ((v << 11) ^ v) % 16777216;
    v
}

fn find_subsequence<T>(container: &[T], sequence: &[T]) -> Option<usize>
where
    for<'a> &'a [T]: PartialEq,
{
    container.windows(sequence.len()).position(|window| window == sequence)
}

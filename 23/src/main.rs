use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs;

fn main() {
    let filename = "input2.txt";
    let input = fs::read_to_string(filename).unwrap();

    let mut con_by_comp: HashMap<&str, HashSet<&str>> = HashMap::new();
    let mut triples: HashSet<[&str; 3]> = HashSet::new();
    input.lines().for_each(|line| {
        let (l, r) = line.split_once("-").unwrap();
        for (k, cons) in &con_by_comp {
            if cons.contains(&l) && cons.contains(&r) {
                let mut triple = [k, l, r];
                triple.sort();
                triples.insert(triple);
            }
        }
        con_by_comp.entry(l).or_insert(HashSet::new()).insert(r);
        con_by_comp.entry(r).or_insert(HashSet::new()).insert(l);
    });

    let part1 = triples
        .iter()
        .filter(|triple| triple[0].starts_with("t") || triple[1].starts_with("t") || triple[2].starts_with("t"))
        .count();
    println!("Part 1: {}", part1);

    let mut max_set = HashSet::new();
    for (k, cons) in &con_by_comp {
        // generate all subsets of cons and check them
        let mut b = 0;
        let cons_vec: Vec<&str> = cons.iter().copied().collect();
        while b < 1 << cons_vec.len() {
            let mut subset = HashSet::new();
            for i in 0..cons_vec.len() {
                if b & (1 << i) != 0 {
                    subset.insert(cons_vec[i]);
                }
            }
            b += 1;

            let length = subset.len() + 1;
            if length <= max_set.len() {
                continue;
            }

            // println!("{:?},", subset);
            let mut found = true;
            for ss_entry in &subset {
                let mut adj_subset = subset.clone();
                adj_subset.remove(ss_entry);
                adj_subset.insert(k);
                if !adj_subset.is_subset(con_by_comp.get(ss_entry).unwrap()) {
                    found = false;
                    break;
                }
            }
            if found {
                subset.insert(k);
                max_set = subset;
            }
        }
    }
    println!("part2: {}", max_set.iter().sorted().join(","));
}

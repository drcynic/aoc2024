use std::collections::{HashMap, HashSet};
use std::{
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let filename = "input2.txt";
    let file = File::open(filename).unwrap();
    let lines: Vec<_> = io::BufReader::new(file).lines().collect::<Result<_, _>>().unwrap();

    let mut rules: HashMap<i32, HashSet<i32>> = HashMap::new();
    let mut updates: Vec<Vec<i32>> = Vec::new();

    lines.iter().for_each(|line| {
        //println!("{}", line);
        if line.find('|').is_some() {
            let (l, r) = line.split_once('|').unwrap();
            rules.entry(l.parse().unwrap()).or_default().insert(r.parse().unwrap());
        } else if line.find(',').is_some() {
            let entries: Vec<i32> = line.split(',').map(|x| x.parse::<i32>().unwrap()).collect();
            updates.push(entries);
        }
    });

    //println!("{:?}", rules);
    //println!("{:?}", updates);

    let mut part1 = 0;
    updates.iter().for_each(|update| {
        if update.iter().enumerate().all(|(i, entry)| !is_rule_violated(entry, &rules, &update[i + 1..])) {
            part1 += update[update.len() / 2];
        }
    });

    println!("part1: {}", part1);

    let mut part2 = 0;
    updates.iter().for_each(|update| {
        if update.iter().enumerate().all(|(i, entry)| !is_rule_violated(entry, &rules, &update[i + 1..])) {
            return;
        }
        // not valid -> fix update
        let updated_entries = fix(&rules, update);
        part2 += updated_entries[updated_entries.len() / 2];
    });

    println!("part2: {}", part2);
}

fn is_rule_violated(pivot: &i32, rules: &HashMap<i32, HashSet<i32>>, followers: &[i32]) -> bool {
    followers.iter().any(|x| rules.contains_key(x) && rules.get(x).unwrap().contains(pivot))
}

fn violated_rule_idx(pivot: &i32, rules: &HashMap<i32, HashSet<i32>>, followers: &[i32]) -> i32 {
    for (i, follower) in followers.iter().enumerate() {
        if rules.contains_key(follower) && rules.get(follower).unwrap().contains(pivot) {
            return i as i32;
        }
    }
    -1
}

fn fix(rules: &HashMap<i32, HashSet<i32>>, entries: &[i32]) -> Vec<i32> {
    let mut entries = entries.to_vec();

    while entries.iter().enumerate().any(|(i, entry)| is_rule_violated(entry, rules, &entries[i + 1..])) {
        for (i, entry) in entries.iter().enumerate() {
            let idx = violated_rule_idx(entry, rules, &entries[i + 1..]);
            if idx != -1 {
                entries.swap(i, i + idx as usize + 1);
                break;
            }
        }
    }

    entries
}

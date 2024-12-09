use std::fs;

#[derive(Debug)]
enum Entry {
    File { id: i64, length: i64 },
    Space(i64),
}

fn main() {
    let filename = "input2.txt";
    let input = fs::read_to_string(filename).unwrap();
    let line: Vec<char> = input.chars().collect();

    let mut filesystem: Vec<i64> = Vec::new();
    let mut entries: Vec<Entry> = Vec::new();
    for i in 0..line.len() {
        let l = line[i].to_string().parse::<i64>().unwrap();
        let id = if i & 1 == 0 {
            entries.push(Entry::File {
                length: l,
                id: (i / 2) as i64,
            });
            (i / 2) as i64
        } else {
            entries.push(Entry::Space(l));
            -1
        };
        for _j in 0..l {
            filesystem.push(id);
        }
    }

    let sum = part1(&filesystem);
    println!("part1: {:?}", sum);

    // part 2
    let entries_length = entries.len();
    for i in 0..entries.len() {
        if i >= entries.len() {
            break;
        }
        let rev_idx = entries_length - i - 1;
        if rev_idx >= entries.len() {
            continue;
        }
        if let Entry::File { id, length } = entries[rev_idx] {
            if let Some(found_idx) = entries.iter().enumerate().position(|(i, e)| match e {
                Entry::Space(s) => i < rev_idx && *s >= length,
                _ => false,
            }) {
                if let Entry::Space(current_space) = entries[found_idx] {
                    entries[found_idx] = Entry::File { id, length };
                    entries[rev_idx] = Entry::Space(length);
                    let remaining_spaces = current_space - length;
                    if remaining_spaces > 0 {
                        entries.insert(found_idx + 1, Entry::Space(remaining_spaces));
                    }
                }
            }
        }
    }

    // print_entries(&entries);
    let sum = count(&entries);
    println!("part2: {}", sum);
}

fn part1(filesystem: &Vec<i64>) -> i64 {
    let mut filesystem = filesystem.clone();
    let mut sum: i64 = 0;
    for i in 0..filesystem.len() {
        if i >= filesystem.len() {
            break;
        }

        if filesystem[i] == -1 {
            let mut item_to_move = filesystem.last().unwrap();
            while *item_to_move == -1 {
                filesystem.pop();
                item_to_move = filesystem.last().unwrap();
            }
            if i > filesystem.len() {
                break;
            }
            filesystem[i] = filesystem[filesystem.len() - 1];
            filesystem.pop();
        }
        sum += i as i64 * filesystem[i];
    }

    sum
}

fn count(entries: &[Entry]) -> i64 {
    let mut sum: i64 = 0;
    let mut idx = 0;
    for e in entries.iter() {
        if let Entry::File { id, length } = e {
            for _j in 0..*length {
                sum += idx * id;
                idx += 1;
            }
        }
        if let Entry::Space(length) = e {
            idx += length;
        }
    }
    sum
}

fn print_entries(entries: &Vec<Entry>) {
    for entry in entries {
        match entry {
            Entry::File { id, length } => {
                for _i in 0..*length {
                    print!("{}", id);
                }
            }
            Entry::Space(length) => {
                for _i in 0..*length {
                    print!(".");
                }
            }
        }
    }
    println!();
}

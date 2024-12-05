use regex::Regex;
use std::{
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let filename = "input2.txt";
    let file = File::open(filename).unwrap();
    let lines: Vec<_> = io::BufReader::new(file).lines().collect::<Result<_, _>>().unwrap();

    let mut part1 = 0;
    lines.iter().for_each(|line| {
        //println!("'{}'", line);
        let sum = find_muls(line);
        part1 += sum;
    });

    //println!("part1: {}", part1);

    let mut part2 = 0;
    let complete_line = lines.iter().fold(String::new(), |acc, line| acc + line);
    // lines.iter().for_each(|line| {
    // println!("'{}'", line);
    let mut line: &str = &complete_line;
    let next_dont_idx = line.find("don't()").unwrap();
    part2 += find_muls(&line[..next_dont_idx]);

    line = &line[next_dont_idx + 7..];
    while line.len() > 0 {
        let next_dont_idx = line.find("don't()");
        let next_do_idx = line.find("do()");
        if next_dont_idx.is_some() && next_do_idx.is_some() {
            if next_do_idx.unwrap() < next_dont_idx.unwrap() {
                part2 += find_muls(&line[next_do_idx.unwrap()..next_dont_idx.unwrap()]);
            }
            line = &line[next_dont_idx.unwrap() + 7..];
        } else if next_dont_idx.is_some() {
            break;
        } else if next_do_idx.is_some() {
            part2 += find_muls(&line[next_do_idx.unwrap()..]);
            line = &line[next_do_idx.unwrap() + 4..];
        } else {
            break;
        }
    }
    // });

    println!("part2: {}", part2);
}

fn find_muls(line: &str) -> i32 {
    let mut sum = 0;
    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    let captures = re.captures_iter(line);
    // println!("{:?}", captures);
    captures.for_each(|cap| {
        let str = &cap[0];
        let (l, r) = str.split_once(",").unwrap();
        let l = l[4..].parse::<i32>().unwrap();
        let r = r[..r.len() - 1].parse::<i32>().unwrap();
        // println!("'{}' '{}' '{}'", str, l, r);
        sum += l * r;
    });
    sum
}

use regex::Regex;
use std::{
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let filename = "input1.txt";
    let file = File::open(filename).unwrap();
    let lines = io::BufReader::new(file).lines();
    let re = Regex::new(r"(\d)").unwrap();

    let sum = lines
        .map(|line| {
            let line = line.unwrap();
            //println!("{}", line);
            let captures = re.captures_iter(&line);
            let values: Vec<i32> = captures.map(|cap| cap[1].parse::<i32>().unwrap()).collect();
            let entry = values.first().unwrap() * 10 + values.last().unwrap();
            entry
        })
        .sum::<i32>();

    println!("sum: {}", sum);
}

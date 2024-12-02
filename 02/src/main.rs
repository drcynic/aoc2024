use std::{
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let filename = "input2.txt";
    let file = File::open(filename).unwrap();
    let lines: Vec<_> = io::BufReader::new(file).lines().collect::<Result<_, _>>().unwrap();

    let mut safe_part1 = 0;
    let mut safe_part2 = 0;
    lines.iter().for_each(|line| {
        // println!("'{}'", line);
        let splits = line.split(" ").collect::<Vec<&str>>();
        let vals: Vec<i32> = splits.iter().map(|s| s.parse::<i32>().unwrap()).collect();
        // println!("{:?}", vals);

        if eval_row(&vals) {
            safe_part1 += 1;
            safe_part2 += 1;
            return;
        }

        // check if removing fixes it
        for i in 0..vals.len() {
            let mut vals2 = vals.clone();
            vals2.remove(i);
            if eval_row(&vals2) {
                safe_part2 += 1;
                return;
            }
        }
    });

    println!("part1: {}", safe_part1);
    println!("part2: {}", safe_part2);
}

fn eval_row(vals: &[i32]) -> bool {
    let increasing = (vals[1] - vals[0]) >= 0;
    for i in 0..vals.len() - 1 {
        let d = vals[i + 1] - vals[i];
        if (d.abs() > 3) || (increasing && d <= 0) || (!increasing && d >= 0) {
            return false;
        }
    }

    true
}

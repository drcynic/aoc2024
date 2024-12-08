use std::collections::HashSet;
use std::{collections::HashMap, fs};

fn main() {
    let filename = "input2.txt";
    let input = fs::read_to_string(filename).unwrap();
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let antennas: HashMap<char, Vec<(i32, i32)>> = grid.iter().enumerate().fold(HashMap::new(), |mut acc, (y, row)| {
        row.iter().enumerate().for_each(|(x, &value)| {
            if value != '.' {
                acc.entry(value).or_default().push((x as i32, y as i32));
            }
        });
        acc
    });
    //println!("{:?}", antennas);

    let unique = part1(&grid, &antennas);
    println!("part1: {}", unique.len());

    let unique = part2(&grid, &antennas);
    println!("part2: {}", unique.len());
}

fn part1(grid: &[Vec<char>], antennas: &HashMap<char, Vec<(i32, i32)>>) -> HashSet<(i32, i32)> {
    let min_x = 0;
    let max_x = grid[0].len() as i32;
    let min_y = 0;
    let max_y = grid.len() as i32;
    let mut unique: HashSet<(i32, i32)> = HashSet::new();

    for antenna in antennas.values() {
        for i in 0..antenna.len() {
            for j in 0..antenna.len() {
                if i == j {
                    continue;
                }
                let p1 = antenna[i];
                let p2 = antenna[j];
                let dx = p1.0 - p2.0;
                let dy = p1.1 - p2.1;
                let anti = (p1.0 + dx, p1.1 + dy);
                if anti.0 < min_x || anti.0 >= max_x || anti.1 < min_y || anti.1 >= max_y {
                    continue;
                }
                // println!("{:?} {:?} {:?}", p1, p2, anti);
                unique.insert(anti);
            }
        }
    }
    unique
}

fn part2(grid: &[Vec<char>], antennas: &HashMap<char, Vec<(i32, i32)>>) -> HashSet<(i32, i32)> {
    let min_x = 0;
    let max_x = grid[0].len() as i32;
    let min_y = 0;
    let max_y = grid.len() as i32;
    let mut unique: HashSet<(i32, i32)> = HashSet::new();

    for (_k, antenna) in antennas.iter() {
        for i in 0..antenna.len() {
            for j in 0..antenna.len() {
                if i == j {
                    continue;
                }
                let p1 = antenna[i];
                let p2 = antenna[j];
                let dx = p1.0 - p2.0;
                let dy = p1.1 - p2.1;
                let mut anti = p1;
                while anti.0 >= min_x && anti.0 < max_x && anti.1 >= min_y && anti.1 < max_y {
                    unique.insert(anti);
                    anti.0 += dx;
                    anti.1 += dy;
                }
                // println!("{:?} {:?} {:?}", p1, p2, anti);
            }
        }
    }

    unique
}

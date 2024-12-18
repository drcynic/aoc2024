use std::{
    cmp::Reverse,
    collections::{HashMap, HashSet},
    fs,
};

use priority_queue::PriorityQueue;

fn main() {
    let filename = "input2.txt";
    let num_bytes = if filename == "input1.txt" { 12 } else { 1024 };
    let bounds = if filename == "input1.txt" { (7, 7) } else { (71, 71) };
    let input = fs::read_to_string(filename).unwrap();
    //println!("{}", input);
    let mut bytes: HashSet<(i32, i32)> = input
        .lines()
        .take(num_bytes)
        .map(|line| {
            let (x, y) = line.split_once(",").unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect();
    // print_grid(&bytes, bounds);
    // println!();
    // print_grid2(&bytes);

    let cost = bfs(&bytes, (0, 0), bounds);
    println!("part1: {}", cost);

    let next_bytes: Vec<(i32, i32)> = input
        .lines()
        .skip(num_bytes)
        .map(|line| {
            let (x, y) = line.split_once(",").unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect();

    for (i, nb) in next_bytes.iter().enumerate() {
        println!("{}:{:?}", i, nb);
        bytes.insert(*nb);
        if bfs(&bytes, (0, 0), bounds) == 0 {
            println!("part2: {:?}", nb);
            break;
        }
    }
}

fn bfs(bytes: &HashSet<(i32, i32)>, start_pos: (i32, i32), bounds: (i32, i32)) -> i32 {
    let mut visited: HashMap<(i32, i32), i32> = HashMap::new();
    let mut pq = PriorityQueue::new();
    pq.push(start_pos, Reverse(0));
    while let Some((current, cost)) = pq.pop() {
        if let Some(vc) = visited.get(&current) {
            if *vc <= cost.0 {
                continue;
            }
        }
        visited.insert(current, cost.0);

        let (x, y) = current;
        if x < 0 || y < 0 || x >= bounds.0 || y >= bounds.1 || bytes.contains(&(x, y)) {
            continue;
        }
        if current == (bounds.0 - 1, bounds.1 - 1) {
            return cost.0;
        }

        pq.push((x - 1, y), Reverse(cost.0 + 1));
        pq.push((x, y - 1), Reverse(cost.0 + 1));
        pq.push((x + 1, y), Reverse(cost.0 + 1));
        pq.push((x, y + 1), Reverse(cost.0 + 1));
    }

    0
}

fn print_grid(grid: &HashSet<(i32, i32)>, bounds: (i32, i32)) {
    // let min_x = grid.iter().map(|(x, _)| x).min().unwrap();
    // let max_x = grid.iter().map(|(x, _)| x).max().unwrap();
    // let min_y = grid.iter().map(|(_, y)| y).min().unwrap();
    // let max_y = grid.iter().map(|(_, y)| y).max().unwrap();
    for y in 0..bounds.1 {
        for x in 0..bounds.0 {
            print!("{}", if grid.contains(&(x, y)) { "#" } else { "." });
        }
        println!();
    }
}
fn print_grid2(grid: &HashSet<(i32, i32)>) {
    let min_x = grid.iter().map(|(x, _)| x).min().unwrap();
    let max_x = grid.iter().map(|(x, _)| x).max().unwrap();
    let min_y = grid.iter().map(|(_, y)| y).min().unwrap();
    let max_y = grid.iter().map(|(_, y)| y).max().unwrap();
    for y in *min_y..=*max_y {
        for x in *min_x..=*max_x {
            print!("{}", if grid.contains(&(x, y)) { "#" } else { "." });
        }
        println!();
    }
}

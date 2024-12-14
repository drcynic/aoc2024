use getch_rs::{Getch, Key};
use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
struct Robot {
    pos: (i32, i32),
    vel: (i32, i32),
}

fn main() {
    let filename = "input2.txt";
    let bounds = if filename == "input1.txt" { (11, 7) } else { (101, 103) };
    let input = fs::read_to_string(filename).unwrap();
    let mut robots: Vec<Robot> = input
        .lines()
        .map(|line| {
            let (l, r) = line.split_once(" ").unwrap();
            let (pls, prs) = l[2..].split_once(",").unwrap();
            let pos = (pls.parse().unwrap(), prs.parse().unwrap());
            let (vls, vrs) = r.split_once(",").unwrap();
            let vel = (vls[2..].parse().unwrap(), vrs.parse().unwrap());
            Robot { pos, vel }
        })
        .collect();
    let part1 = simulate(&mut robots, bounds, 100, false);
    //println!("{:?}", &robots);
    //print_grid(&robots, bounds);
    println!("Part 1: {}", part1);

    let part2 = simulate(&mut robots, bounds, 100000, true);
    println!("Part 2: {}", part2);
}

fn simulate(robots: &mut [Robot], bounds: (i32, i32), steps: i32, is_part2: bool) -> i32 {
    let g = Getch::new();
    for _n in 0..steps {
        for r in robots.iter_mut() {
            r.pos.0 = (r.pos.0 + r.vel.0 + bounds.0) % bounds.0;
            r.pos.1 = (r.pos.1 + r.vel.1 + bounds.1) % bounds.1;
        }

        if is_part2 {
            let num_per_lines: HashMap<i32, i32> = robots.iter().map(|r| r.pos.1).fold(HashMap::new(), |mut acc, x| {
                *acc.entry(x).or_insert(0) += 1;
                acc
            });
            // visually step through the simulation where a lot of robots are on the same line
            // which I expect happens when forming the christmas tree
            if num_per_lines.iter().any(|(_, v)| *v > 30) {
                print_grid(robots, bounds);
                println!("step: {}", _n + 1);
                let k = g.getch().unwrap();
                if k == Key::Esc {
                    return _n + 1;
                }
            }
        }
    }

    let mut quadrants: [i32; 4] = [0, 0, 0, 0];
    for r in robots.iter_mut() {
        if r.pos.0 == bounds.0 / 2 || r.pos.1 == bounds.1 / 2 {
            continue;
        }

        let q = if r.pos.0 < bounds.0 / 2 && r.pos.1 < bounds.1 / 2 {
            0
        } else if r.pos.0 >= bounds.0 / 2 && r.pos.1 < bounds.1 / 2 {
            1
        } else if r.pos.0 < bounds.0 / 2 && r.pos.1 >= bounds.1 / 2 {
            2
        } else {
            3
        };
        quadrants[q] += 1;
    }

    quadrants[0] * quadrants[1] * quadrants[2] * quadrants[3]
}

fn print_grid(robots: &[Robot], bounds: (i32, i32)) {
    let mut robots_pos: HashMap<(i32, i32), i32> = HashMap::new();
    for r in robots.iter() {
        if robots_pos.contains_key(&r.pos) {
            let count = robots_pos.get(&r.pos).unwrap();
            robots_pos.insert(r.pos, count + 1);
            continue;
        }
        robots_pos.insert(r.pos, 1);
    }
    let mut grid = vec![vec!['.'; bounds.0 as usize]; bounds.1 as usize];
    for (pos, num) in robots_pos.iter() {
        grid[pos.1 as usize][pos.0 as usize] = num.to_string().chars().next().unwrap();
    }
    for row in grid {
        println!("{}", row.iter().collect::<String>());
    }
    println!();
}

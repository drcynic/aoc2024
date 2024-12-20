use std::{collections::HashMap, fs};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    pos: (i64, i64),
}

fn main() {
    let filename = "input2.txt";
    let mut grid: Vec<Vec<char>> = fs::read_to_string(filename).expect("error").lines().map(|l| l.chars().collect()).collect();
    let mut start_pos = (0, 0);
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == 'S' {
                start_pos = (x as i64, y as i64);
                grid[y][x] = '.';
            }
        }
    }

    let mut normal_path: Vec<(i64, i64)> = Vec::new();
    let _ = dfs(&grid, Pos { pos: start_pos }, 0, i64::MAX, &mut HashMap::new(), &mut normal_path);

    let max_cheat_cost = if filename == "input1.txt" { normal_path.len() as i64 - 1 } else { normal_path.len() as i64 - 100 };
    let count = get_num_cheats(&mut normal_path, max_cheat_cost, 2);
    println!("part1: {}", count);

    let max_cheat_cost = if filename == "input1.txt" { normal_path.len() as i64 - 50 } else { normal_path.len() as i64 - 100 };
    let count = get_num_cheats(&mut normal_path, max_cheat_cost, 20);
    println!("part2: {}", count);
}

fn get_num_cheats(normal_path: &mut Vec<(i64, i64)>, max_cheat_cost: i64, max_cheat_time: i64) -> i32 {
    let mut count = 0;
    for i in 0..normal_path.len() {
        for j in i..normal_path.len() {
            if i == j {
                continue;
            }
            let p1 = normal_path[i];
            let p2 = normal_path[j];

            let dist = (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs();
            if dist > max_cheat_time {
                continue;
            }

            let cheat_dist = (i + dist as usize + (normal_path.len() - j - 1)) as i64;
            if cheat_dist < max_cheat_cost {
                // println!("cheat dist: {}", cheat_dist);
                count += 1;
            }
        }
    }
    count
}

fn dfs(grid: &Vec<Vec<char>>, current: Pos, cost: i64, max_cost: i64, visited: &mut HashMap<Pos, i64>, res: &mut Vec<(i64, i64)>) -> i64 {
    if cost > max_cost {
        visited.insert(current, cost);
        return 0;
    }

    let (x, y) = current.pos;
    if x < 0 || y < 0 || x as usize >= grid[0].len() || y as usize >= grid.len() {
        return 0;
    }
    let next_char = grid[y as usize][x as usize];
    if next_char == '#' {
        return 0;
    }

    if next_char == 'E' {
        if cost <= max_cost {
            println!("found: {}", cost);
            res.push(current.pos);
            return 1;
        }
        return 0;
    }

    if let Some(vc) = visited.get(&current) {
        if *vc < cost {
            return *vc;
        }
    }

    visited.insert(current, cost);
    res.push(current.pos);

    dfs(grid, Pos { pos: (x - 1, y) }, cost + 1, max_cost, visited, res);
    dfs(grid, Pos { pos: (x, y - 1) }, cost + 1, max_cost, visited, res);
    dfs(grid, Pos { pos: (x + 1, y) }, cost + 1, max_cost, visited, res);
    dfs(grid, Pos { pos: (x, y + 1) }, cost + 1, max_cost, visited, res);

    0
}

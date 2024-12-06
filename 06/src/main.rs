use std::collections::HashSet;
use std::fs;

fn main() {
    let filename = "input2.txt";
    let input = fs::read_to_string(filename).unwrap();
    let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let dirs = [(0, -1), (1, 0), (0, 1), (-1, 0)];
    let idx = input.find("^").unwrap() as i32;
    let start_pos = (idx % (grid[0].len() + 1) as i32, idx / (grid[0].len() + 1) as i32);

    let visited = part1(&grid, dirs, start_pos);
    println!("part1: {}", visited);

    // brute force part2, finished in a few seconds, but probably it would be better to
    // use visited positions from part1 and put the obstacles only around the paths or
    // do it in a more clever way
    let mut part2 = 0;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if y == start_pos.1 as usize && x == start_pos.0 as usize || grid[y][x] == '#' {
                continue;
            }
            grid[y][x] = '#';
            if check_for_loop(&grid, dirs, start_pos) {
                part2 += 1;
            }
            grid[y][x] = '.';
        }
    }
    println!("part2: {}", part2);
}

fn part1(grid: &[Vec<char>], dirs: [(i32, i32); 4], mut pos: (i32, i32)) -> usize {
    let mut visited = HashSet::new();
    let mut dir_idx = 0;
    let mut dir = dirs[dir_idx];
    loop {
        visited.insert(pos);
        let new_pos = (pos.0 + dir.0, pos.1 + dir.1);
        // println!("{:?}", new_pos);
        if new_pos.0 < 0 || new_pos.0 >= grid[0].len() as i32 || new_pos.1 < 0 || new_pos.1 >= grid.len() as i32 {
            break;
        }
        if grid[new_pos.1 as usize][new_pos.0 as usize] == '#' {
            dir_idx = (dir_idx + 1) % 4;
            dir = dirs[dir_idx];
            continue;
        }
        pos = new_pos;
    }
    visited.len()
}

fn check_for_loop(grid: &[Vec<char>], dirs: [(i32, i32); 4], mut pos: (i32, i32)) -> bool {
    let mut visited = HashSet::new();
    let mut dir_idx = 0;
    let mut dir = dirs[dir_idx];
    loop {
        if visited.contains(&(pos, dir)) {
            return true;
        }
        visited.insert((pos, dir));
        let new_pos = (pos.0 + dir.0, pos.1 + dir.1);
        if new_pos.0 < 0 || new_pos.0 >= grid[0].len() as i32 || new_pos.1 < 0 || new_pos.1 >= grid.len() as i32 {
            break;
        }
        if grid[new_pos.1 as usize][new_pos.0 as usize] == '#' {
            dir_idx = (dir_idx + 1) % 4;
            dir = dirs[dir_idx];
            continue;
        }
        pos = new_pos;
    }
    false
}

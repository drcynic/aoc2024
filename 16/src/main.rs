use std::collections::{HashMap, HashSet};
use std::fs;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    pos: (i32, i32),
    dir: (i32, i32),
}

fn main() {
    let filename = "input2.txt";
    let input = fs::read_to_string(filename).unwrap();
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().map(|c| if c == 'S' { '.' } else { c }).collect())
        .collect();
    let start_pos = (1, grid.len() as i32 - 2);
    let current_pos = Pos {
        pos: start_pos,
        dir: (1, 0),
    };

    let mut visited: HashMap<Pos, i32> = HashMap::new();
    let mut part1 = i32::MAX;
    dfs(&grid, current_pos, 0, &mut part1, &mut visited);
    println!("part1: {}", part1);

    let part1 = if filename == "input1.txt" { part1 } else { 135536 };
    visited.clear();
    let mut path: Vec<(i32, i32)> = Vec::new();
    let mut result: HashSet<(i32, i32)> = HashSet::new();
    dfs2(&grid, current_pos, 0, part1, &mut visited, &mut path, &mut result);
    println!("part2: {}", result.len());
}

fn dfs2(
    grid: &Vec<Vec<char>>,
    current_pos: Pos,
    cost: i32,
    max_cost: i32,
    visited: &mut HashMap<Pos, i32>,
    path: &mut Vec<(i32, i32)>,
    result: &mut HashSet<(i32, i32)>,
) {
    if cost > max_cost {
        visited.insert(current_pos, cost);
        return;
    }

    let (x, y) = current_pos.pos;
    path.push((x, y));
    let next_char = grid[y as usize][x as usize];
    if next_char == '#' {
        path.pop();
        return;
    }
    if next_char == 'E' {
        if cost == max_cost {
            for p in path.iter() {
                result.insert(*p);
            }
        }
        path.pop();
        return;
    }

    // print_grid(grid, &current_pos);
    // let _k = Getch::new().getch().unwrap();
    //if k == Key::Esc {
    //    return _n + 1;
    //}

    if let Some(vc) = visited.get(&current_pos) {
        // CHANGE to part1: '<' instead of '<=' !!!
        if *vc < cost {
            path.pop();
            return;
        }
    }

    visited.insert(current_pos, cost);

    let (dx, dy) = current_pos.dir;
    let next_pos = Pos {
        pos: (x + dx, y + dy),
        dir: (dx, dy),
    };

    let (nx, ny) = next_pos.pos;
    if nx > 0 && ny > 0 && nx < grid[0].len() as i32 || ny < grid.len() as i32 {
        dfs2(grid, next_pos, cost + 1, max_cost, visited, path, result);
    }

    let next_pos = Pos {
        pos: (x, y),
        dir: rotate_left(dx, dy),
    };

    dfs2(grid, next_pos, cost + 1000, max_cost, visited, path, result);

    let next_pos = Pos {
        pos: (x, y),
        dir: rotate_right(dx, dy),
    };
    dfs2(grid, next_pos, cost + 1000, max_cost, visited, path, result);

    path.pop();
}

fn dfs(grid: &Vec<Vec<char>>, current_pos: Pos, cost: i32, min_cost: &mut i32, visited: &mut HashMap<Pos, i32>) {
    let (x, y) = current_pos.pos;
    let next_char = grid[y as usize][x as usize];
    if next_char == '#' {
        return;
    }
    if next_char == 'E' {
        if cost <= *min_cost {
            *min_cost = cost;
        }
        return;
    }

    // print_grid(grid, &current_pos);
    // let _k = Getch::new().getch().unwrap();
    //if k == Key::Esc {
    //    return _n + 1;
    //}

    if let Some(vc) = visited.get(&current_pos) {
        if *vc <= cost {
            return;
        }
    }

    visited.insert(current_pos, cost);

    let (dx, dy) = current_pos.dir;
    let next_pos = Pos {
        pos: (x + dx, y + dy),
        dir: (dx, dy),
    };

    let (nx, ny) = next_pos.pos;
    if nx > 0 && ny > 0 && nx < grid[0].len() as i32 || ny < grid.len() as i32 {
        dfs(grid, next_pos, cost + 1, min_cost, visited);
    }

    let next_pos = Pos {
        pos: (x, y),
        dir: rotate_left(dx, dy),
    };

    dfs(grid, next_pos, cost + 1000, min_cost, visited);

    let next_pos = Pos {
        pos: (x, y),
        dir: rotate_right(dx, dy),
    };
    dfs(grid, next_pos, cost + 1000, min_cost, visited);
}
fn rotate_left(dx: i32, dy: i32) -> (i32, i32) {
    match (dx, dy) {
        (1, 0) => (0, -1),
        (0, -1) => (-1, 0),
        (-1, 0) => (0, 1),
        (0, 1) => (1, 0),
        _ => panic!("Invalid direction"),
    }
}

fn rotate_right(dx: i32, dy: i32) -> (i32, i32) {
    match (dx, dy) {
        (1, 0) => (0, 1),
        (0, 1) => (-1, 0),
        (-1, 0) => (0, -1),
        (0, -1) => (1, 0),
        _ => panic!("Invalid direction"),
    }
}

fn print_grid(grid: &[Vec<char>], pos: &Pos) {
    for (y, row) in grid.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if (x as i32, y as i32) == pos.pos {
                if pos.dir == (1, 0) {
                    print!(">");
                } else if pos.dir == (-1, 0) {
                    print!("<");
                } else if pos.dir == (0, 1) {
                    print!("v");
                } else if pos.dir == (0, -1) {
                    print!("^");
                }
            } else {
                print!("{}", c);
            }
        }
        println!();
    }
}

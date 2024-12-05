use std::{
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let filename = "input2.txt";
    let file = File::open(filename).unwrap();
    let lines: Vec<_> = io::BufReader::new(file).lines().collect::<Result<_, _>>().unwrap();
    let grid: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();

    // for row in &grid {
    //     println!("{:?}", row);
    // }

    let num_rows = grid.len();
    let num_cols = grid[0].len();
    let chars = ['X', 'M', 'A', 'S'];
    let dirs = vec![(-1, 0), (1, 0), (0, -1), (0, 1), (-1, -1), (1, 1), (-1, 1), (1, -1)];
    let mut part1 = 0;
    for y in 0..num_rows {
        for x in 0..num_cols {
            if !check_pos(&grid, x, y, chars[0]) {
                continue;
            }

            for dir in &dirs {
                let (dx, dy) = dir;
                if !check_pos(&grid, (x as i32 + dx) as usize, (y as i32 + dy) as usize, chars[1]) {
                    continue;
                }
                if !check_pos(&grid, (x as i32 + 2 * dx) as usize, (y as i32 + 2 * dy) as usize, chars[2]) {
                    continue;
                }
                if !check_pos(&grid, (x as i32 + 3 * dx) as usize, (y as i32 + 3 * dy) as usize, chars[3]) {
                    continue;
                }
                part1 += 1;
            }
        }
    }

    println!("part1: {}", part1);

    let dirs = [(-1, -1), (-1, 1), (1, -1), (1, 1)];
    let chars = [vec!['M', 'M', 'S', 'S'], vec!['M', 'S', 'M', 'S'], vec!['S', 'M', 'S', 'M'], vec!['S', 'S', 'M', 'M']];
    let mut part2 = 0;
    for y in 1..num_rows - 1 {
        'cols: for x in 1..num_cols - 1 {
            if !check_pos(&grid, x, y, 'A') {
                continue;
            }

            for chars_to_use in chars.iter() {
                let mut found = true;
                for (i, dir) in dirs.iter().enumerate() {
                    let (dx, dy) = *dir;
                    if !check_pos(&grid, (x as i32 + dx) as usize, (y as i32 + dy) as usize, chars_to_use[i]) {
                        found = false;
                        break;
                    }
                }
                if found {
                    part2 += 1;
                    continue 'cols;
                }
            }
        }
    }

    println!("part2: {}", part2);
}

fn check_pos(grid: &[Vec<char>], x: usize, y: usize, look_for: char) -> bool {
    if x >= grid[0].len() || y >= grid.len() {
        return false;
    }
    grid[y][x] == look_for
}

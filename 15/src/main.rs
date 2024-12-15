use std::collections::{HashMap, HashSet};
use std::fs;

fn main() {
    let filename = "input2.txt";
    let mut start_pos = if filename == "input1.txt" { (4, 4) } else { (24, 24) };
    let (mut width, mut height) = if filename == "input1.txt" { (10, 10) } else { (50, 50) };
    let input = fs::read_to_string(filename).unwrap();
    let (warehouse_input, move_input) = input.split_once("\n\n").unwrap();
    let move_to_dir_idx: HashMap<char, usize> = HashMap::from([('<', 0), ('^', 1), ('>', 2), ('v', 3)]);
    let dirs = [(-1, 0), (0, -1), (1, 0), (0, 1)];
    let moves: Vec<char> = move_input.lines().fold(String::new(), |acc, line| acc + line).chars().collect();

    let mut boxes: HashSet<(i32, i32)> = collect_by_char(warehouse_input, 'O', 1, false);
    let walls: HashSet<(i32, i32)> = collect_by_char(warehouse_input, '#', 1, false);

    let mut pos = start_pos;
    moves.iter().for_each(|&m| {
        let dir_idx = move_to_dir_idx.get(&m).unwrap();
        let dir = dirs[*dir_idx];
        pos = move_step(&mut boxes, &walls, dir, pos);
    });

    print_grid(&boxes, &walls, pos, width, height, true);
    println!();

    let sum = boxes.iter().fold(0, |acc, (x, y)| acc + x + y * 100);
    println!("part1: {}\n", sum);

    width *= 2;
    pos = (start_pos.0 * 2, start_pos.1);
    let mut boxes: HashSet<(i32, i32)> = collect_by_char(warehouse_input, 'O', 2, true);
    let walls: HashSet<(i32, i32)> = collect_by_char(warehouse_input, '#', 2, false);
    moves.iter().for_each(|&m| {
        let dir_idx = move_to_dir_idx.get(&m).unwrap();
        let dir = dirs[*dir_idx];
        pos = move_step2(&mut boxes, &walls, dir, pos);
    });

    print_grid(&boxes, &walls, pos, width, height, false);
    println!();

    let sum = boxes.iter().fold(0, |acc, (x, y)| acc + x + y * 100);
    println!("part2: {}\n", sum);
}

fn move_step(boxes: &mut HashSet<(i32, i32)>, walls: &HashSet<(i32, i32)>, dir: (i32, i32), pos: (i32, i32)) -> (i32, i32) {
    let new_pos = (pos.0 + dir.0, pos.1 + dir.1);
    if walls.contains(&new_pos) {
        pos
    } else if boxes.contains(&new_pos) {
        if check_and_move_box(boxes, walls, new_pos, dir) {
            return new_pos;
        }
        return pos;
    } else {
        return new_pos;
    }
}

fn check_and_move_box(boxes: &mut HashSet<(i32, i32)>, walls: &HashSet<(i32, i32)>, pos: (i32, i32), dir: (i32, i32)) -> bool {
    let next_pos = (pos.0 + dir.0, pos.1 + dir.1);
    if walls.contains(&next_pos) || (boxes.contains(&next_pos) && !check_and_move_box(boxes, walls, next_pos, dir)) {
        false
    } else {
        boxes.remove(&pos);
        boxes.insert(next_pos);
        true
    }
}

fn move_step2(boxes: &mut HashSet<(i32, i32)>, walls: &HashSet<(i32, i32)>, dir: (i32, i32), pos: (i32, i32)) -> (i32, i32) {
    let new_pos = (pos.0 + dir.0, pos.1 + dir.1);
    if walls.contains(&new_pos) {
        pos
    } else {
        let new_side_pos = if dir == (-1, 0) || dir == (1, 0) {
            (new_pos.0 + dir.0, new_pos.1)
        } else {
            (new_pos.0 - 1, new_pos.1)
        };
        if boxes.contains(&new_pos) {
            if check_box2(boxes, walls, new_pos, dir) {
                move_box2(boxes, walls, new_pos, dir);
                return new_pos;
            }
            return pos;
        } else if dir != (1, 0) && boxes.contains(&new_side_pos) {
            if check_box2(boxes, walls, new_side_pos, dir) {
                move_box2(boxes, walls, new_side_pos, dir);
                return new_pos;
            }
            return pos;
        }
        new_pos
    }
}

fn check_box2(boxes: &mut HashSet<(i32, i32)>, walls: &HashSet<(i32, i32)>, pos: (i32, i32), dir: (i32, i32)) -> bool {
    let next_pos = (pos.0 + dir.0, pos.1 + dir.1);
    if walls.contains(&next_pos) {
        return false;
    }
    // we always get the box position (left part of the box), so we have to handle dirs to check the next position
    if dir == (-1, 0) || dir == (1, 0) {
        let next_box_check_pos = (next_pos.0 + dir.0, next_pos.1);
        if dir == (1, 0) && walls.contains(&next_box_check_pos) {
            return false;
        }
        if boxes.contains(&next_box_check_pos) && !check_box2(boxes, walls, next_box_check_pos, dir) {
            return false;
        }
    } else if dir == (0, -1) || dir == (0, 1) {
        if walls.contains(&(next_pos.0 + 1, next_pos.1)) {
            return false;
        }
        if boxes.contains(&next_pos) {
            if !check_box2(boxes, walls, next_pos, dir) {
                return false;
            }
        } else {
            let left_pos = (next_pos.0 - 1, next_pos.1);
            let contains_left = boxes.contains(&left_pos);
            let right_pos = (next_pos.0 + 1, next_pos.1);
            let contains_right = boxes.contains(&right_pos);
            if contains_left && contains_right {
                if !check_box2(boxes, walls, left_pos, dir) {
                    return false;
                }
                if !check_box2(boxes, walls, right_pos, dir) {
                    return false;
                }
            } else if contains_left {
                if !check_box2(boxes, walls, left_pos, dir) {
                    return false;
                }
            } else if contains_right && !check_box2(boxes, walls, right_pos, dir) {
                return false;
            }
        }
    }

    true
}

fn move_box2(boxes: &mut HashSet<(i32, i32)>, walls: &HashSet<(i32, i32)>, pos: (i32, i32), dir: (i32, i32)) {
    let next_pos = (pos.0 + dir.0, pos.1 + dir.1);
    if dir == (-1, 0) || dir == (1, 0) {
        let next_box_check_pos = (next_pos.0 + dir.0, next_pos.1);
        if boxes.contains(&next_box_check_pos) {
            move_box2(boxes, walls, next_box_check_pos, dir);
        }
    } else if dir == (0, -1) || dir == (0, 1) {
        if boxes.contains(&next_pos) {
            move_box2(boxes, walls, next_pos, dir);
        } else {
            let left_pos = (next_pos.0 - 1, next_pos.1);
            let right_pos = (next_pos.0 + 1, next_pos.1);
            if boxes.contains(&left_pos) {
                move_box2(boxes, walls, left_pos, dir);
            }
            if boxes.contains(&right_pos) {
                move_box2(boxes, walls, right_pos, dir);
            }
        }
    }

    if boxes.contains(&pos) {
        boxes.remove(&pos);
        boxes.insert(next_pos);
    }
}

fn collect_by_char(warehouse_input: &str, pick: char, width: i32, one_piece: bool) -> HashSet<(i32, i32)> {
    let mut result = HashSet::new();
    for (y, line) in warehouse_input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == pick {
                result.insert(((x as i32) * width, y as i32));
                if width > 1 && !one_piece {
                    result.insert(((x as i32) * width + 1, y as i32));
                }
            }
        }
    }
    result
}

fn print_grid(boxes: &HashSet<(i32, i32)>, walls: &HashSet<(i32, i32)>, pos: (i32, i32), width: i32, height: i32, part1: bool) {
    let mut grid = vec![vec!['.'; width as usize]; height as usize];

    for (x, y) in boxes {
        if part1 {
            grid[*y as usize][*x as usize] = 'O';
        } else {
            grid[*y as usize][*x as usize] = '[';
            grid[*y as usize][*x as usize + 1] = ']';
        }
    }
    for (x, y) in walls {
        grid[*y as usize][*x as usize] = '#';
    }
    grid[pos.1 as usize][pos.0 as usize] = '@';
    for row in grid {
        println!("{}", row.iter().collect::<String>());
    }
}

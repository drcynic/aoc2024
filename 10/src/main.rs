use std::{collections::HashSet, fs};

fn main() {
    let filename = "input2.txt";
    let input = fs::read_to_string(filename).unwrap();
    let grid: Vec<Vec<i32>> = input.lines().map(|line| line.bytes().map(|c| (c - 48) as i32).collect()).collect();
    let zeros: HashSet<(i32, i32)> = grid
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(x, c)| if *c == 0 { Some((x as i32, y as i32)) } else { None })
        })
        .collect();

    let mut part1 = 0;
    let mut part2 = 0;
    for pos in zeros.iter() {
        let mut reachable_9s: HashSet<(i32, i32)> = HashSet::new();
        let mut trailhead_sum = 0;
        trailhead_sum += check_pos(&grid, pos.0, pos.1, &mut vec![], &mut reachable_9s);
        part1 += reachable_9s.len();
        part2 += trailhead_sum;
    }
    println!("part 1: {}", part1);
    println!("part 2: {}", part2);
}

fn check_pos(grid: &[Vec<i32>], x: i32, y: i32, route: &mut Vec<(i32, i32)>, reachable_9s: &mut HashSet<(i32, i32)>) -> i32 {
    if x < 0 || x >= grid[0].len() as i32 || y < 0 || y >= grid.len() as i32 {
        return 0;
    }

    let val = grid[y as usize][x as usize];
    if route.len() == 9 && val == 9 {
        reachable_9s.insert((x, y));
        return 1;
    }

    if route.len() != val as usize {
        return 0;
    }

    route.push((x, y));

    let mut sum = 0;
    for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)].iter() {
        sum += check_pos(grid, x + dx, y + dy, route, reachable_9s);
    }

    route.pop();

    sum
}

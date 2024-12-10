use std::{collections::HashSet, fs};

fn main() {
    let filename = "input2.txt";
    let input = fs::read_to_string(filename).unwrap();
    //let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let grid: Vec<Vec<i32>> = input.lines().map(|line| line.bytes().map(|c| (c - 48) as i32).collect()).collect();
    println!("{:?}", grid);
    let zeros: HashSet<(i32, i32)> = grid
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(x, c)| if *c == 0 { Some((x as i32, y as i32)) } else { None })
        })
        .collect();
    println!("{:?}", zeros);
    //let dirs = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    let mut sum = 0;
    for pos in zeros.clone() {
        println!("{:?}", pos);
        let mut reachable_9s: HashSet<(i32, i32)> = HashSet::new();
        let mut route = vec![pos];
        for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)].iter() {
            check_pos(&grid, pos.0 + dx, pos.1 + dy, &mut route, &mut reachable_9s);
        }
        // println!("zeros: {:?} has {} reachable_9s", pos, reachable_9s.len());
        sum += reachable_9s.len();
    }
    println!("part 1: {}", sum);

    let mut sum = 0;
    for pos in zeros {
        println!("{:?}", pos);
        let mut trailhead_sum = 0;
        let mut route = vec![pos];
        for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)].iter() {
            trailhead_sum += check_pos2(&grid, pos.0 + dx, pos.1 + dy, &mut route);
        }
        println!("zeros: {:?} has {} distinct reachable 9s", pos, trailhead_sum);
        sum += trailhead_sum;
    }
    println!("part 2: {}", sum);
}

fn check_pos(grid: &[Vec<i32>], x: i32, y: i32, route: &mut Vec<(i32, i32)>, reachable_9s: &mut HashSet<(i32, i32)>) {
    if x < 0 || x >= grid[0].len() as i32 || y < 0 || y >= grid.len() as i32 {
        return;
    }

    if *route.last().unwrap() == (x, y) {
        return;
    }

    let val = grid[y as usize][x as usize];
    // println!("({}, {}: {}, route: {:?}", x, y, val, &route);
    if route.len() == 9 && val == 9 {
        // println!("found ");
        reachable_9s.insert((x, y));
    }

    if route.len() != val as usize {
        return;
    }

    route.push((x, y));

    for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)].iter() {
        check_pos(grid, x + dx, y + dy, route, reachable_9s);
    }

    route.pop();
}

fn check_pos2(grid: &[Vec<i32>], x: i32, y: i32, route: &mut Vec<(i32, i32)> ) -> i32 {
    if x < 0 || x >= grid[0].len() as i32 || y < 0 || y >= grid.len() as i32 {
        return 0;
    }

    if *route.last().unwrap() == (x, y) {
        return 0;
    }

    let val = grid[y as usize][x as usize];
    // println!("({}, {}: {}, route: {:?}", x, y, val, &route);
    if route.len() == 9 && val == 9 {
        // println!("found ");
        return 1;
    }

    if route.len() != val as usize {
        return 0;
    }

    route.push((x, y));

    let mut sum = 0;
    for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)].iter() {
        sum += check_pos2(grid, x + dx, y + dy, route);
    }

    route.pop();

    sum
}

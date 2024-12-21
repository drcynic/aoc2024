use memoize::memoize;
use std::cmp::min;
use std::fs;

fn main() {
    let filename = "input2.txt";
    let input = fs::read_to_string(filename).unwrap();

    let mut part1 = 0usize;
    input.lines().for_each(|line| {
        let num = line[0..line.len() - 1].parse::<usize>().unwrap();
        let length = calc(line, 2);
        let res = num * length;
        // println!("{} * {} = {}", num, length, res);
        part1 += res;
    });
    println!("Part 1: {}", part1);

    let mut part2 = 0usize;
    input.lines().for_each(|line| {
        let num = line[0..line.len() - 1].parse::<usize>().unwrap();
        let length = calc(line, 25);
        let res = num * length;
        // println!("{} * {} = {}", num, length, res);
        part2 += res;
    });
    println!("Part 2: {}", part2);
}

fn calc(seq: &str, num_pads: usize) -> usize {
    let mut loc = char_to_numpad_loc('A');

    memoized_flush_move_to_target();

    let mut sum = 0;
    seq.chars().for_each(|c| {
        let next = char_to_numpad_loc(c);
        let current = loc;
        let dist = (next.0 - current.0, next.1 - current.1);
        loc = next;
        if current.0 == 3 && next.1 == 0 {
            sum += move_to_target(dist.0, dist.1, num_pads, false);
        } else if current.1 == 0 && next.0 == 3 {
            sum += move_to_target(dist.0, dist.1, num_pads, true);
        } else {
            sum += min(
                move_to_target(dist.0, dist.1, num_pads, true),
                move_to_target(dist.0, dist.1, num_pads, false),
            );
        }
    });
    sum
}

#[memoize]
fn move_to_target(dy: i32, dx: i32, num_pads: usize, move_horz_first: bool) -> usize {
    let mut steps = vec![if dy < 0 { '^' } else { 'v' }; dy.unsigned_abs() as usize];
    steps.extend(vec![if dx < 0 { '<' } else { '>' }; dx.unsigned_abs() as usize]);
    if move_horz_first {
        steps.reverse();
    }

    steps.push('A');

    if num_pads == 0 {
        return steps.len();
    }

    let mut loc = char_to_robot_pad_loc('A');
    let mut sum = 0usize;
    for i in 0..steps.len() {
        let next = char_to_robot_pad_loc(steps[i]);
        let current = loc;
        loc = next;
        let dist = (next.0 - current.0, next.1 - current.1);
        if dist.0 == 0 || dist.1 == 0 {
            sum += move_to_target(dist.0, dist.1, num_pads - 1, false);
        } else if next == (1, 0) && current.0 == 0 {
            sum += move_to_target(dist.0, dist.1, num_pads - 1, false);
        } else if current == (1, 0) && next.0 == 0 {
            sum += move_to_target(dist.0, dist.1, num_pads - 1, true);
        } else {
            let horz_first = crate::move_to_target(dist.0, dist.1, num_pads - 1, false);
            let vert_first = crate::move_to_target(dist.0, dist.1, num_pads - 1, true);
            sum += min(horz_first, vert_first);
        }
    }

    sum
}

fn char_to_numpad_loc(key: char) -> (i32, i32) {
    match key {
        '7' => (0, 0),
        '8' => (0, 1),
        '9' => (0, 2),
        '4' => (1, 0),
        '5' => (1, 1),
        '6' => (1, 2),
        '1' => (2, 0),
        '2' => (2, 1),
        '3' => (2, 2),
        '0' => (3, 1),
        'A' => (3, 2),
        _ => unreachable!(),
    }
}

fn char_to_robot_pad_loc(key: char) -> (i32, i32) {
    match key {
        '^' => (0, 1),
        'A' => (0, 2),
        '<' => (1, 0),
        'v' => (1, 1),
        '>' => (1, 2),
        _ => unreachable!(),
    }
}

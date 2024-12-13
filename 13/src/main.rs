use std::cmp::max;
use std::fs;

#[derive(Debug)]
struct Claw {
    a: (i64, i64),
    b: (i64, i64),
    prize: (i64, i64),
}

fn main() {
    let filename = "input2.txt";
    let input = fs::read_to_string(filename).unwrap();
    let mut claws: Vec<Claw> = Vec::new();
    input.split("\n\n").for_each(|config| {
        let mut lines = config.lines();
        let a = read_pos(lines.next().unwrap(), "X+");
        let b = read_pos(lines.next().unwrap(), "X+");
        let prize = read_pos(lines.next().unwrap(), "X=");
        claws.push(Claw { a, b, prize });
    });
    let part1 = claws.iter().map(|claw| solve_claw_simple(claw)).sum::<i64>();
    println!("Part 1: {}", part1);

    claws.iter_mut().for_each(|claw| {
        claw.prize.0 += 10000000000000;
        claw.prize.1 += 10000000000000;
    });
    let part2 = claws.iter().map(|claw| solve_claw_better(claw)).sum::<i64>();
    println!("Part 2: {}", part2);
}

fn solve_claw_simple(claw: &Claw) -> i64 {
    let max_x = max(claw.prize.0 / claw.a.0, claw.prize.0 / claw.b.0);
    let max_y = max(claw.prize.1 / claw.a.1, claw.prize.1 / claw.b.1);
    let max_all = max(max_x, max_y);
    let mut min_cost = i64::MAX;
    for a in 0..max_all + 1 {
        for b in 0..max_all + 1 {
            let x = claw.a.0 * a + claw.b.0 * b;
            let y = claw.a.1 * a + claw.b.1 * b;
            let cost = a * 3 + b;
            if x == claw.prize.0 && y == claw.prize.1 && cost < min_cost {
                min_cost = cost;
            }
        }
    }
    let cost = if min_cost == i64::MAX { 0 } else { min_cost };
    // println!("Claw: {:?}, min_cost: {}", claw, cost);
    cost
}

fn solve_claw_better(claw: &Claw) -> i64 {
    let (m, n) = solve_equation_system(&claw);

    if m > 0.0 && is_integer(m) && n > 0.0 && is_integer(n) {
        let cost = (m as i64) + (n as i64) * 3;
        cost
    } else {
        0
    }
}

fn solve_equation_system(claw: &Claw) -> (f64, f64) {
    let ax = claw.a.0 as f64;
    let ay = claw.a.1 as f64;
    let bx = claw.b.0 as f64;
    let by = claw.b.1 as f64;
    let cx = claw.prize.0 as f64;
    let cy = claw.prize.1 as f64;

    let m = (ax * cy - ay * cx) / (ax * by - ay * bx);
    let n = (cx - bx * m) / ax;

    (m, n)
}
fn is_integer(x: f64) -> bool {
    (x - x.round()).abs() < f64::EPSILON
}

fn read_pos(button: &str, search_term: &str) -> (i64, i64) {
    let a_comma_index = button.find(",").unwrap();
    let x = button[button.find(search_term).unwrap() + 2..a_comma_index].parse::<i64>().unwrap();
    let y = button[a_comma_index + 4..].parse::<i64>().unwrap();
    (x, y)
}

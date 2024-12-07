use std::fs;

#[derive(Debug)]
struct Equation {
    result: i64,
    numbers: Vec<i64>,
}

fn main() {
    let filename = "input2.txt";
    let input = fs::read_to_string(filename).unwrap();
    let equations: Vec<Equation> = input
        .lines()
        .map(|line| {
            let (l, r) = line.split_once(": ").unwrap();
            Equation {
                result: l.parse().unwrap(),
                numbers: r.split(" ").map(|x| x.parse().unwrap()).collect(),
            }
        })
        .collect();
    // println!("{:?}", equations);

    let ops = [add, mul];
    let part1: i64 = equations.iter().map(|eq| eval_eq(eq, &ops)).sum();
    println!("part1: {}", part1);

    let ops = [add, mul, con];
    let part2: i64 = equations.iter().map(|eq| eval_eq(eq, &ops)).sum();
    println!("part2: {}", part2);
}

fn add(a: i64, b: i64) -> i64 {
    a + b
}

fn mul(a: i64, b: i64) -> i64 {
    a * b
}

fn con(a: i64, b: i64) -> i64 {
    let f = 10_i64.pow(b.ilog10() + 1);
    a * f + b
}

fn eval_eq(eq: &Equation, ops: &[fn(i64, i64) -> i64]) -> i64 {
    if eq.numbers.len() == 1 {
        return if eq.numbers[0] == eq.result { eq.result } else { 0 };
    }
    let len = eq.numbers.len() as u32;
    for i in 0..ops.len().pow(len - 1) {
        let mut r = eq.numbers[0];
        for j in 1..eq.numbers.len() {
            let k = i / ops.len().pow((j - 1) as u32) % ops.len();
            r = ops[k](r, eq.numbers[j]);
        }
        if r == eq.result {
            return r;
        }
    }
    0
}

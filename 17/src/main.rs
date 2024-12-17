use std::fs;

#[derive(Debug)]
struct Computer {
    reg_a: u64,
    reg_b: u64,
    reg_c: u64,
    instr_ptr: usize,
}

fn main() {
    let filename = "input2.txt";
    let input = fs::read_to_string(filename).unwrap();
    let (reg_lines, prog_string) = input.split_once("\n\n").unwrap();
    let reg_a: u64 = reg_lines.split_once("\n").unwrap().0.split_once(": ").unwrap().1.parse().unwrap();
    let instr: Vec<u64> = prog_string.split_once(": ").unwrap().1.split(",").map(|x| x.trim_end().parse().unwrap()).collect();
    let mut computer = Computer { reg_a, reg_b: 0, reg_c: 0, instr_ptr: 0 };

    print!("Part 1: ");
    run(&mut computer, &instr);
    println!();

    println!("Part 2 instructions: {:?}", instr);
    let reg_a = check_instr(&instr, &instr.clone(), 0).unwrap();
    println!("result: {}", reg_a);
}

fn check_instr(instr: &[u64], output: &[u64], a: u64) -> Option<u64> {
    println!("checking for {} in a: {}", instr.last().unwrap(), a);
    for i in 0..8 {
        let a_next = a * 8 + i;
        let mut computer = Computer { reg_a: a_next, reg_b: 0, reg_c: 0, instr_ptr: 0 };
        run(&mut computer, &instr[..instr.len() - 2]); // without the jmp at the end
        let b = computer.reg_b % 8;
        if b == output[output.len() - 1] {
            // println!("{}: {}", a_next, b);
            if output.len() > 1 {
                if let Some(a_res) = check_instr(instr, &output[..output.len() - 1], a_next) {
                    return Some(a_res);
                }
            } else {
                return Some(a_next);
            }
        }
    }
    None
}

fn run(computer: &mut Computer, instr: &[u64]) {
    loop {
        if !execute_instruction(computer, instr) {
            break;
        }
    }
}

fn execute_instruction(computer: &mut Computer, instructions: &[u64]) -> bool {
    if computer.instr_ptr >= instructions.len() {
        return false;
    }

    // println!("{:?}", computer);

    let opcode = instructions[computer.instr_ptr];
    let operand = instructions[computer.instr_ptr + 1];
    match opcode {
        0 => adv(computer, operand),
        1 => bxl(computer, operand),
        2 => bst(computer, operand),
        3 => jnz(computer, operand),
        4 => bxc(computer, operand),
        5 => out(computer, operand),
        6 => bdv(computer, operand),
        7 => cdv(computer, operand),
        _ => panic!("Invalid opcode"),
    }

    if opcode != 3 || computer.reg_a == 0 {
        computer.instr_ptr += 2;
    }

    true
}

fn combo_operand(computer: &Computer, operand: u64) -> u64 {
    match operand {
        0..=3 => operand,
        4 => computer.reg_a,
        5 => computer.reg_b,
        6 => computer.reg_c,
        _ => panic!("Invalid operand"),
    }
}

fn adv(computer: &mut Computer, operand: u64) {
    let v = combo_operand(computer, operand);
    computer.reg_a /= 2_u64.pow(v.try_into().unwrap());
}

fn bxl(computer: &mut Computer, operand: u64) {
    computer.reg_b ^= operand;
}

fn bst(computer: &mut Computer, operand: u64) {
    let v = combo_operand(computer, operand);
    computer.reg_b = v % 8;
}

fn jnz(computer: &mut Computer, operand: u64) {
    if computer.reg_a != 0 {
        computer.instr_ptr = operand as usize;
    }
}

fn bxc(computer: &mut Computer, _operand: u64) {
    computer.reg_b ^= computer.reg_c;
}

fn out(computer: &mut Computer, operand: u64) {
    print!("{},", combo_operand(computer, operand) % 8);
}

fn bdv(computer: &mut Computer, operand: u64) {
    let v = combo_operand(computer, operand);
    computer.reg_b = computer.reg_a / 2_u64.pow(v.try_into().unwrap());
}

fn cdv(computer: &mut Computer, operand: u64) {
    let v = combo_operand(computer, operand);
    computer.reg_c = computer.reg_a / 2_u64.pow(v.try_into().unwrap());
}

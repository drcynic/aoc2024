use petgraph::dot::{Config, Dot};
use petgraph::graph::NodeIndex;
use petgraph::Graph;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone, PartialEq)]
struct Gate<'a> {
    input: (&'a str, &'a str),
    func: fn(a: bool, b: bool) -> bool,
    function: &'a str,
    output: &'a str,
}

fn main() {
    let filename = "input2.txt";
    let input = fs::read_to_string(filename).unwrap();
    let (values_input, gates_input) = input.split_once("\n\n").unwrap();
    let values: HashMap<&str, bool> = values_input
        .lines()
        .map(|line| {
            let (k, v) = line.split_once(": ").unwrap();
            (k, if v == "1" { true } else { false })
        })
        .collect();

    let mut gates: Vec<Gate> = gates_input
        .lines()
        .map(|gate_line| {
            let (gate_str, output) = gate_line.split_once(" -> ").unwrap();
            let gate_parts: Vec<&str> = gate_str.split(" ").collect();
            let func = match gate_parts[1] {
                "AND" => |a: bool, b: bool| a & b,
                "OR" => |a: bool, b: bool| a | b,
                "XOR" => |a: bool, b: bool| a ^ b,
                _ => panic!("Unknown gate type"),
            };
            Gate { input: (gate_parts[0], gate_parts[2]), func, function: gate_parts[1], output }
        })
        .collect();
    // println!("{:?}", &gates.len());

    let part1 = calc_output(&values, &gates);
    println!("Part 1: {}", part1);

    // gen graph for visualization
    let mut deps = Graph::<String, &str>::new();
    let mut node_indices: HashMap<&str, NodeIndex> = HashMap::new();
    for value in values.keys() {
        let idx = deps.add_node(value.to_string());
        node_indices.insert(value, idx);
    }
    for gate in &gates {
        let idx_func = deps.add_node(format!("{} {}", gate.function, gate.output));
        node_indices.insert(gate.output, idx_func);
    }
    for gate in &gates {
        deps.add_edge(*node_indices.get(gate.input.0).unwrap(), *node_indices.get(gate.output).unwrap(), "");
        deps.add_edge(*node_indices.get(gate.input.1).unwrap(), *node_indices.get(gate.output).unwrap(), "");
    }
    // Output the tree to `graphviz` `DOT` format, can be visualized at https://viz-js.com/
    println!("{:?}", Dot::with_config(&deps, &[Config::EdgeNoLabel]));

    // calc the target output: add x and y inputs and mask to 46 bits
    let mut x_input_values: Vec<(&str, bool)> = values.iter().filter(|(k, _)| k.starts_with("x")).map(|(k, v)| (*k, *v)).collect();
    x_input_values.sort_by(|lhs, rhs| lhs.cmp(rhs));
    println!("{:#048b}", get_value(&x_input_values));
    let mut y_input_values: Vec<(&str, bool)> = values.iter().filter(|(k, _)| k.starts_with("y")).map(|(k, v)| (*k, *v)).collect();
    y_input_values.sort_by(|lhs, rhs| lhs.cmp(rhs));
    println!("{:#048b}", get_value(&y_input_values));
    let target_output = (get_value(&x_input_values) + get_value(&y_input_values)) & ((1 << 46) - 1);
    println!("{}", target_output);
    println!("{:#048b}", target_output);

    // all but a few outputs have a OR instead a XOR find these
    let mut indices_for_gates_to_swap = vec![];
    let gates_by_output: HashMap<&str, &Gate> = gates.iter().map(|gate| (gate.output, gate)).collect();
    let mut output_gates = gates_by_output.iter().filter(|(output, _)| output.starts_with("z")).collect::<Vec<_>>();
    output_gates.sort_by(|lhs, rhs| lhs.0.cmp(rhs.0));
    let mut output_without_xor = output_gates.iter().filter(|(_, gate)| gate.function != "XOR").collect::<Vec<_>>();
    output_without_xor.sort_by(|lhs, rhs| lhs.0.cmp(rhs.0));
    for i in 0..output_without_xor.len() - 1 {
        let (output, gate) = output_without_xor[i];
        let idx = gates.iter().position(|g| g.input == gate.input && g.output == gate.output).unwrap();
        indices_for_gates_to_swap.push(idx);
        println!("{}: {} -> idx {}", output, gate.function, idx);
    }

    let is_fixed_input_output = |v: &str| v.starts_with("x") || v.starts_with("y") || v.starts_with("z");
    let mut xors_without_fixed_input_output = gates
        .iter()
        .filter(|gate| {
            gate.function == "XOR"
                && (!is_fixed_input_output(gate.input.0) && !is_fixed_input_output(gate.input.1) && !is_fixed_input_output(gate.output))
        })
        .collect::<Vec<_>>();
    for gate in xors_without_fixed_input_output {
        let idx = gates.iter().position(|g| g.input == gate.input && g.output == gate.output).unwrap();
        println!("{}: {} -> idx: {}", gate.output, gate.function, idx);
        indices_for_gates_to_swap.push(idx);
    }
    println!("{:?}", indices_for_gates_to_swap);

    // results this:
    // z10: OR -> idx 187
    // z32: AND -> idx 193
    // z39: AND -> idx 21
    // grm: XOR -> idx: 58
    // twr: XOR -> idx: 182
    // ggn: XOR -> idx: 199

    // by looking at graph viz we see that z10 <=> ggn, z32 <=> grm, z39 <=> twr are the swaps
    swap_gate_outputs(&mut gates, 187, 199);
    swap_gate_outputs(&mut gates, 193, 58);
    swap_gate_outputs(&mut gates, 21, 182);

    // calc new output and check which two bits still differ
    let results_after_6_swaps = calc_output(&values, &gates);
    println!("{:#048b} <- target ", target_output);
    println!("{:#048b} <- results after 6 swaps", results_after_6_swaps & ((1 << 46) - 1));
    let xor_6_swaps = target_output ^ (results_after_6_swaps & ((1 << 46) - 1));
    println!("{:#048b} <- target XOR current", xor_6_swaps);
    let index = xor_6_swaps.trailing_zeros().to_string();
    println!("result with 6 swaps differ at index: {}", index);
    let indices: Vec<usize> =
        gates.iter().enumerate().filter(|(i, g)| g.input.0.ends_with(&index) && g.input.1.ends_with(&index)).map(|(i, _)| i).collect();
    println!("indices with x{} and y{} inputs: {:?}", index, index, indices);
    println!("swap these gates");
    swap_gate_outputs(&mut gates, indices[0], indices[1]);
    let results_after_8_swaps = calc_output(&values, &gates);
    println!("{:#048b} <- target ", target_output);
    println!("{:#048b} <- results after 8 swaps", results_after_8_swaps & ((1 << 46) - 1));
    let xor_8_swaps = target_output ^ (results_after_8_swaps & ((1 << 46) - 1));
    println!("{:#048b} <- target XOR current", xor_8_swaps);
    if xor_8_swaps == 0 {
        println!("successful corrected adder");
        indices_for_gates_to_swap.extend(indices);
        let mut names: Vec<&str> = indices_for_gates_to_swap.iter().map(|idx| gates[*idx].output).collect();
        names.sort();
        println!("Part 2: {}", names.join(","));
    }
}

fn swap_gate_outputs(gates: &mut Vec<Gate>, i: usize, j: usize) {
    let tmp = gates.get_mut(i).unwrap().output;
    gates.get_mut(i).unwrap().output = gates.get(j).unwrap().output;
    gates.get_mut(j).unwrap().output = tmp;
}

fn calc_output(values: &HashMap<&str, bool>, gates: &Vec<Gate>) -> i64 {
    let mut gates_by_input: HashMap<&str, &Gate> = HashMap::new();
    for gate in gates {
        gates_by_input.insert(gate.input.0, gate);
        gates_by_input.insert(gate.input.1, gate);
    }

    let gates_by_output: HashMap<&str, &Gate> = gates.iter().map(|gate| (gate.output, gate)).collect();
    let mut output_gates = gates_by_output.iter().filter(|(output, _)| output.starts_with("z")).collect::<Vec<_>>();
    output_gates.sort_by(|lhs, rhs| lhs.0.cmp(rhs.0));

    output_gates
        .iter()
        .enumerate()
        .map(|(idx, (_, gate))| {
            let v = if eval(gate, &values, &gates_by_output) { 1 } else { 0 };
            v << idx
        })
        .sum()
}

fn get_value(values: &[(&str, bool)]) -> i64 {
    values.iter().enumerate().map(|(idx, (_, v))| if *v { 1 } else { 0 } << idx).sum()
}

fn eval(gate: &Gate, values: &HashMap<&str, bool>, gates_by_output: &HashMap<&str, &Gate>) -> bool {
    let a = eval_gate_input(gate.input.0, values, gates_by_output);
    let b = eval_gate_input(gate.input.1, values, gates_by_output);
    (gate.func)(a, b)
}

fn eval_gate_input(input: &str, values: &HashMap<&str, bool>, gates_by_output: &HashMap<&str, &Gate>) -> bool {
    if input.starts_with("x") || input.starts_with("y") {
        *values.get(input).unwrap()
    } else {
        eval(gates_by_output.get(input).unwrap(), values, gates_by_output)
    }
}

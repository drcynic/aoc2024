use std::collections::{HashMap, HashSet};
use std::fs;

fn main() {
    let filename = "input2.txt";
    let input = fs::read_to_string(filename).unwrap();
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let plants: HashMap<char, Vec<(i32, i32)>> = grid.iter().enumerate().fold(HashMap::new(), |mut acc, (y, row)| {
        row.iter().enumerate().for_each(|(x, &value)| {
            acc.entry(value).or_default().push((x as i32, y as i32));
        });
        acc
    });

    let mut part1 = 0;
    let mut part2 = 0;
    for (k, v) in &plants {
        let connected = connected_plants(&v.iter().cloned().collect());
        for c in connected {
            let (area, perimeter) = area_and_perimeter(&c);
            part1 += area * perimeter;

            let (area, sides) = area_and_sides(&c);
            part2 += area * sides;
        }
    }

    println!("part1: {}", part1);
    println!("part2: {}", part2);
}

fn connected_plants(plant: &HashSet<(i32, i32)>) -> Vec<HashSet<(i32, i32)>> {
    let mut visited = HashSet::new();
    let mut connected_plants: Vec<HashSet<(i32, i32)>> = Vec::new();
    for p in plant {
        let mut collected = HashSet::new();
        collect(plant, p.0, p.1, &mut visited, &mut collected);
        if !collected.is_empty() {
            connected_plants.push(collected);
        }
    }

    connected_plants
}

fn collect(plant: &HashSet<(i32, i32)>, x: i32, y: i32, visited: &mut HashSet<(i32, i32)>, collected: &mut HashSet<(i32, i32)>) {
    if plant.contains(&(x, y)) && !visited.contains(&(x, y)) {
        visited.insert((x, y));
        collected.insert((x, y));
        collect(plant, x - 1, y, visited, collected);
        collect(plant, x, y - 1, visited, collected);
        collect(plant, x + 1, y, visited, collected);
        collect(plant, x, y + 1, visited, collected);
    }
}

fn area_and_perimeter(plant: &HashSet<(i32, i32)>) -> (i32, i32) {
    let area = plant.len() as i32;
    let mut perimeter = 0;
    for d in [(-1, 0), (0, -1), (1, 0), (0, 1)].iter() {
        for p in plant {
            if !plant.contains(&(p.0 + d.0, p.1 + d.1)) {
                perimeter += 1;
            }
        }
    }
    (area, perimeter)
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Side {
    Top,
    Bottom,
    Left,
    Right,
}

fn area_and_sides(plant: &HashSet<(i32, i32)>) -> (i32, i32) {
    let area = plant.len() as i32;
    let mut distinct_sides: HashMap<&Side, HashSet<(i32, i32)>> = HashMap::new();
    let side_types = [Side::Left, Side::Top, Side::Right, Side::Bottom];
    for (i, d) in [(-1, 0), (0, -1), (1, 0), (0, 1)].iter().enumerate() {
        for p in plant {
            if !plant.contains(&(p.0 + d.0, p.1 + d.1)) {
                distinct_sides.entry(&side_types[i]).or_default().insert((p.0, p.1));
            }
        }
    }

    let mut collected = 0;
    for (k, v) in &distinct_sides {
        let mut used = HashSet::new();
        for p in v {
            collected += collect_sides(v, p, k, &mut used);
        }
    }

    (area, collected)
}

fn collect_sides(sides: &HashSet<(i32, i32)>, p: &(i32, i32), side: &Side, used: &mut HashSet<(i32, i32)>) -> i32 {
    let search_dirs = if *side == Side::Top || *side == Side::Bottom {
        [(-1, 0), (1, 0)]
    } else {
        [(0, -1), (0, 1)]
    };

    if used.contains(p) {
        return 0;
    }
    used.insert((p.0, p.1));

    for d in search_dirs.iter() {
        if sides.contains(&(p.0 + d.0, p.1 + d.1)) {
            collect_sides(sides, &(p.0 + d.0, p.1 + d.1), side, used);
        }
    }

    1
}

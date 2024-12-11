use std::collections::HashMap;

fn main() {
    let input = "125 17";
    //let input = "";
    let stones: Vec<&str> = input.split_whitespace().collect();
    let mut cache = HashMap::new();

    let mut sum = 0;
    for stone in &stones {
        sum += blink_stone(stone, 25, &mut cache);
    }
    println!("part1: {:?}", sum);

    sum = 0;
    for stone in &stones {
        sum += blink_stone(stone, 75, &mut cache);
    }
    println!("part2: {:?}", sum);
}

fn blink_stone(stone: &str, num_blinks: usize, cache: &mut HashMap<(String, usize), i64>) -> i64 {
    if num_blinks == 0 {
        return 1;
    }

    let ce = (stone.to_owned(), num_blinks);
    if let Some(v) = cache.get(&ce) {
        return *v;
    }

    let n = stone.parse::<i64>().unwrap();
    let r = if n == 0 {
        blink_stone("1", num_blinks - 1, cache)
    } else if stone.len() % 2 == 0 {
        let first_half = stone[..stone.len() / 2].parse::<i64>().unwrap().to_string();
        let last_half = stone[stone.len() / 2..].parse::<i64>().unwrap().to_string();
        blink_stone(&first_half, num_blinks - 1, cache) + blink_stone(&last_half, num_blinks - 1, cache)
    } else {
        blink_stone(&(n * 2024).to_string(), num_blinks - 1, cache)
    };

    cache.insert(ce, r);
    r
}

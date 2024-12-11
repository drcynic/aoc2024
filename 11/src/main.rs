use std::collections::HashMap;

#[derive(Eq, Hash, PartialEq, Debug, Clone)]
struct CacheEntry {
    stone: String,
    num_blinks: usize,
}

fn main() {
    //let input = "125 17";
    let input = "9694820 93 54276 1304 314 664481 0 4";
    let stones: Vec<String> = input.split_whitespace().map(|s| s.to_string()).collect();

    let mut sum = 0;
    let mut cache: HashMap<CacheEntry, i64> = HashMap::new();
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

fn blink_stone(stone: &str, num_blinks: usize, cache: &mut HashMap<CacheEntry, i64>) -> i64 {
    if num_blinks == 0 {
        return 1;
    }

    let ce = CacheEntry {
        stone: stone.to_owned(),
        num_blinks,
    };
    if cache.contains_key(&ce) {
        return *cache.get(&ce).unwrap();
    }

    let n = stone.parse::<i64>().unwrap();
    let r = if n == 0 {
        let key = "1".to_string();
        blink_stone(&key, num_blinks - 1, cache)
    } else if stone.len() % 2 == 0 {
        let first_half = n.to_string().chars().take(n.to_string().len() / 2).collect::<String>();
        let first_half = first_half.parse::<i64>().unwrap().to_string();
        let last_half = n.to_string().chars().skip(n.to_string().len() / 2).collect::<String>();
        let last_half = last_half.parse::<i64>().unwrap().to_string();
        blink_stone(&first_half, num_blinks - 1, cache) + blink_stone(&last_half, num_blinks - 1, cache)
    } else {
        blink_stone(&(n * 2024).to_string(), num_blinks - 1, cache)
    };

    cache.insert(ce, r);
    r
}

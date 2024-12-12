use std::collections::HashMap;

type Cache = HashMap<(i64, i32), i64>;
type StoneCache = HashMap<i64, StoneResult>;

#[derive(Clone, Debug)]
enum StoneResult {
    Single(i64),
    Pair(i64, i64),
}

fn blinking_stone(stone: i64, cache: &mut StoneCache) -> StoneResult {
    if let Some(result) = cache.get(&stone) {
        return result.clone();
    }

    let result = if stone == 0 {
        StoneResult::Single(1)
    } else {
        let s = stone.to_string();
        let n = s.len();
        
        if n % 2 == 0 {
            let (first, second) = s.split_at(n / 2);
            StoneResult::Pair(
                first.parse().unwrap_or(0),
                second.parse().unwrap_or(0)
            )
        } else {
            StoneResult::Single(2024 * stone)
        }
    };

    cache.insert(stone, result.clone());
    result
}

fn number_stones(stone: i64, deep: i32, cache: &mut Cache, stone_cache: &mut StoneCache) -> i64 {
    if deep < 1 {
        return 1;
    }

    if let Some(&result) = cache.get(&(stone, deep)) {
        return result;
    }

    let result = match blinking_stone(stone, stone_cache) {
        StoneResult::Pair(a, b) => {
            number_stones(a, deep - 1, cache, stone_cache) + 
            number_stones(b, deep - 1, cache, stone_cache)
        },
        StoneResult::Single(n) => number_stones(n, deep - 1, cache, stone_cache),
    };

    cache.insert((stone, deep), result);
    result
}

fn main() {
    let puzzle_input = vec![20, 82084, 1650, 3, 346355, 363, 7975858, 0];
    let times = 75;
    
    let mut cache = HashMap::new();
    let mut stone_cache = HashMap::new();
    
    let result: i64 = puzzle_input.iter()
        .map(|&stone| number_stones(stone, times, &mut cache, &mut stone_cache))
        .sum();
    
    println!("Solution for 75 blinks: {}", result);
    println!("Cache info:");
    println!("number_stones cache size: {}", cache.len());
    println!("blinking_stone cache size: {}", stone_cache.len());
}
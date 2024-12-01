# 🎄 Advent of Code 2024 🎄

Welcome to my Advent of Code 2024 solutions repository! Join me on this magical coding journey through the winter wonderlands of algorithms and data structures! ⭐️

## 🎮 Follow Along Live!

Watch me solve these puzzles live on my Twitch channel:
[krystal_mess323](https://www.twitch.tv/krystal_mess323) 🎥

## 🎯 Current Progress

- [x] Day 1: Historian Hysteria 📚
  - Part 1: Completed! ⭐️
  - Part 2: Coming soon! 🔜

## 🛠️ Running the Solutions

1. Make sure you have Rust installed 🦀
2. Clone this repository
3. Navigate to the project directory
4. Place your input in `input.txt`
5. Run the solution:
```bash
cargo run
```

## 🎁 Directory Structure

```
advent_of_code/
├── src/
│   └── main.rs    # Solution code
├── input.txt      # Your puzzle input
└── README.md      # You are here! 👋
```

## 🌟 Join the Fun!

- Follow my Twitch stream for live solving sessions! 📺
- Check out [Advent of Code](https://adventofcode.com/2024) to try the puzzles yourself! 🎮
- Share your own solutions and let's learn together! 🤝

## 📚 Solutions Documentation

### Day 1: Historian Hysteria

#### Problem Description
The Chief Historian is missing, and we need to help find them! The Elvish Senior Historians have discovered two lists of location IDs that need to be reconciled. The task is to:
1. Take two lists of numbers
2. Sort both lists
3. Pair up corresponding numbers
4. Calculate the absolute difference between each pair
5. Sum all differences

#### Solution Approach
```rust
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    match run() {
        Ok(result) => println!("Total distance: {}", result),
        Err(e) => eprintln!("Error: Could not process input file: {}\nMake sure you're running from the project root and input.txt exists.", e),
    }
}

fn run() -> io::Result<i32> {
    // Try current directory first, then src directory
    let paths = ["input.txt", "src/input.txt"];
    let file = paths.iter()
        .find_map(|&path| File::open(path).ok())
        .ok_or_else(|| io::Error::new(
            io::ErrorKind::NotFound,
            "Could not find input.txt in current directory or src/ directory"
        ))?;
    
    let reader = io::BufReader::new(file);
    let mut left_list = Vec::new();
    let mut right_list = Vec::new();
    
    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }
        
        let numbers: Vec<i32> = line
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();
            
        if numbers.len() == 2 {
            left_list.push(numbers[0]);
            right_list.push(numbers[1]);
        }
    }
    
    Ok(calculate_total_distance(&mut left_list, &mut right_list))
}

fn calculate_total_distance(left: &mut Vec<i32>, right: &mut Vec<i32>) -> i32 {
    left.sort_unstable();
    right.sort_unstable();
    
    left.iter()
        .zip(right.iter())
        .map(|(a, b)| (a - b).abs())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_case() {
        let mut left = vec![3, 4, 2, 1, 3, 3];
        let mut right = vec![4, 3, 5, 3, 9, 3];
        assert_eq!(calculate_total_distance(&mut left, &mut right), 11);
    }
}
```

#### Key Features
- Robust error handling
- File input support
- Unit tests included
- Efficient sorting with sort_unstable
- Iterator-based calculations

#### Example Input
```
3 4
4 3
2 5
1 3
3 9
3 3
```

#### Example Output
```
Total distance: 11
```

## ❤️ Happy Coding!

May your code be bug-free and your algorithms be swift! 🚀

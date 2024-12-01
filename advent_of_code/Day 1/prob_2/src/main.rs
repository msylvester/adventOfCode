use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;

fn main() {
    match run() {
        Ok(result) => println!("Similarity score: {}", result),
        Err(e) => eprintln!("Error: Could not process input file: {}\nMake sure you're running from the project root and input.txt exists.", e),
    }
}

fn run() -> io::Result<i32> {
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
    
    Ok(calculate_similarity_score(&left_list, &right_list))
}

fn calculate_similarity_score(left: &[i32], right: &[i32]) -> i32 {
    // Create frequency map for right list
    let right_freq: HashMap<i32, i32> = right.iter()
        .fold(HashMap::new(), |mut map, &num| {
            *map.entry(num).or_insert(0) += 1;
            map
        });
    
    // Calculate similarity score
    left.iter()
        .map(|&num| num * right_freq.get(&num).unwrap_or(&0))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_case() {
        let left = vec![3, 4, 2, 1, 3, 3];
        let right = vec![4, 3, 5, 3, 9, 3];
        assert_eq!(calculate_similarity_score(&left, &right), 31);
    }
}
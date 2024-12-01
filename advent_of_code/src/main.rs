use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    let path = Path::new("src/input.txt");
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    
    let mut left_list = Vec::new();
    let mut right_list = Vec::new();
    
    // Parse input
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
    
    let result = calculate_total_distance(&mut left_list, &mut right_list);
    println!("Total distance: {}", result);
    
    Ok(())
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
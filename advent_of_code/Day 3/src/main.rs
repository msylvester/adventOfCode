use std::fs::File;
use std::io::{self, Read};
use regex::Regex;

/// Processes a string containing multiplication instructions in the format mul(x,y)
/// and returns the sum of all valid multiplications.
/// 
/// # Arguments
/// * `input` - A string containing zero or more multiplication instructions
/// 
/// # Returns
/// * `i32` - The sum of all valid multiplications
fn process_multiplications(input: &str) -> i32 {
    let array_of_mult = process_input(input);
    
    array_of_mult.iter()
        .map(|expr| {
            let numbers: Vec<i32> = expr
                .trim_start_matches("mul(")
                .trim_end_matches(")")
                .split(',')
                .filter_map(|num| num.parse().ok())
                .collect();
            
            if numbers.len() == 2 {
                numbers[0] * numbers[1]
            } else {
                0
            }
        })
        .sum()
}

/// Extracts valid multiplication instructions from the input string using regex.
/// Valid format is mul(x,y) where x and y are 1-3 digit numbers.
/// 
/// # Arguments
/// * `input` - The string to process
/// 
/// # Returns
/// * `Vec<&str>` - Vector of matched multiplication instructions
fn process_input(input: &str) -> Vec<&str> {
    let re = Regex::new(r"mul\(\d{1,3},\s*\d{1,3}\)").unwrap();
    re.find_iter(input)
        .map(|mat| mat.as_str())
        .collect()
}

/// Reads the contents of a file into a String.
/// 
/// # Arguments
/// * `filename` - The name of the file to read
/// 
/// # Returns
/// * `io::Result<String>` - The file contents or an error
fn read_file(filename: &str) -> io::Result<String> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() -> io::Result<()> {
    let filename = "input.txt";
    match read_file(filename) {
        Ok(contents) => {
            let result = process_multiplications(&contents);
            println!("Total: {}", result);
            Ok(())
        }
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            Err(e)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(process_multiplications(input), 161);
    }
    
    #[test]
    fn test_invalid_cases() {
        let input = "mul(4* mul(6,9! ?(12,34) mul ( 2 , 4 )";
        assert_eq!(process_multiplications(input), 0);
    }

    #[test]
    fn test_file_reading() {
        std::fs::write("test_input.txt", "mul(2,4)mul(3,3)").unwrap();
        let contents = read_file("test_input.txt").unwrap();
        assert_eq!(process_multiplications(&contents), 17);
        std::fs::remove_file("test_input.txt").unwrap();
    }
}
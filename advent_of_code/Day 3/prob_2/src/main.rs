use std::fs::File;
use std::io::{self, Read};
use regex::Regex;

fn process_multiplications(input: &str) -> i32 {
    let mut enabled = true;
    let mut total = 0;
    
    // Create regex patterns for all instruction types
    let mul_re = Regex::new(r"mul\((\d{1,3}),\s*(\d{1,3})\)").unwrap();
    let do_re = Regex::new(r"do\(\)").unwrap();
    let dont_re = Regex::new(r"don't\(\)").unwrap();
    
    // Get all matches with their positions
    let mut instructions: Vec<(usize, &str)> = Vec::new();
    
    // Collect multiplication instructions
    for m in mul_re.find_iter(input) {
        instructions.push((m.start(), m.as_str()));
    }
    
    // Collect do() instructions
    for m in do_re.find_iter(input) {
        instructions.push((m.start(), m.as_str()));
        println!("Found do() at position {}", m.start());
    }
    
    // Collect don't() instructions
    for m in dont_re.find_iter(input) {
        instructions.push((m.start(), m.as_str()));
        println!("Found don't() at position {}", m.start());
    }
    
    // Sort instructions by position
    instructions.sort_by_key(|k| k.0);
    
    // Process instructions in order
    println!("\nProcessing instructions in order:");
    for (pos, instruction) in instructions {
        println!("Position {}: {} (enabled: {})", pos, instruction, enabled);
        
        if instruction == "do()" {
            enabled = true;
            println!("-> Enabling multiplications");
        } else if instruction == "don't()" {
            enabled = false;
            println!("-> Disabling multiplications");
        } else if enabled && instruction.starts_with("mul") {
            let caps: Vec<i32> = mul_re.captures(instruction)
                .map(|cap| {
                    let n1 = cap[1].parse::<i32>().unwrap_or(0);
                    let n2 = cap[2].parse::<i32>().unwrap_or(0);
                    n1 * n2
                })
                .into_iter()
                .collect();
            
            if let Some(result) = caps.first() {
                total += result;
                println!("-> Processing multiplication: {} = {}", instruction, result);
            }
        } else if !enabled && instruction.starts_with("mul") {
            println!("-> Skipping disabled multiplication: {}", instruction);
        }
    }
    
    println!("\nFinal total: {}", total);
    total
}

fn read_file(filename: &str) -> io::Result<String> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("File contents:\n{}\n", contents);
    Ok(contents)
}

fn main() -> io::Result<()> {
    let filename = "input.txt";
    match read_file(filename) {
        Ok(contents) => {
            let result = process_multiplications(&contents);
            println!("\nTotal: {}", result);
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
    fn test_with_control_instructions() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)do()?mul(8,5))";
        assert_eq!(process_multiplications(input), 48); // 2*4 + 8*5
    }
}

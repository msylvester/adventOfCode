use std::fs::read_to_string;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Clone, Copy)]
enum Operator {
    Add,
    Multiply,
    Concat,
}

fn evaluate_expression(numbers: &[i64], operators: &[Operator]) -> i64 {
    let mut result = numbers[0];
    for (i, &op) in operators.iter().enumerate() {
        match op {
            Operator::Add => result += numbers[i + 1],
            Operator::Multiply => result *= numbers[i + 1],
            Operator::Concat => {
                // Convert both numbers to strings, concatenate, then parse back to i64
                let concat_str = format!("{}{}", result, numbers[i + 1]);
                result = i64::from_str(&concat_str).unwrap_or(0);
            }
        }
    }
    result
}

fn try_all_combinations(test_value: i64, numbers: &[i64]) -> bool {
    if numbers.len() == 1 {
        return test_value == numbers[0];
    }

    let num_operators = numbers.len() - 1;
    // Each position can have 3 operators now (Add, Multiply, Concat)
    // So we need 3^n combinations for n operators
    let total_combinations = 3_i32.pow(num_operators as u32);

    for i in 0..total_combinations {
        let mut operators = Vec::with_capacity(num_operators);
        let mut num = i;
        
        // Convert number to base-3 to get operator combinations
        for _ in 0..num_operators {
            operators.push(match num % 3 {
                0 => Operator::Add,
                1 => Operator::Multiply,
                2 => Operator::Concat,
                _ => unreachable!(),
            });
            num /= 3;
        }

        if evaluate_expression(numbers, &operators) == test_value {
            return true;
        }
    }
    false
}

fn parse_line(line: &str) -> Option<(i64, Vec<i64>)> {
    let parts: Vec<&str> = line.split(':').collect();
    if parts.len() != 2 {
        return None;
    }

    let test_value = i64::from_str(parts[0].trim()).ok()?;
    let numbers: Vec<i64> = parts[1]
        .trim()
        .split_whitespace()
        .filter_map(|n| i64::from_str(n).ok())
        .collect();

    Some((test_value, numbers))
}

fn solve_calibration(input: &str) -> i64 {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .filter_map(|line| parse_line(line))
        .filter(|(test_value, numbers)| try_all_combinations(*test_value, numbers))
        .map(|(test_value, _)| test_value)
        .sum()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get the current executable's directory
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("input.txt");
    
    println!("Looking for input file at: {}", path.display());
    
    let input = read_to_string(&path)?;
    let result = solve_calibration(&input);
    println!("Solution: {}", result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate_expression() {
        // Test concatenation
        let numbers = vec![15, 6];
        let operators = vec![Operator::Concat];
        assert_eq!(evaluate_expression(&numbers, &operators), 156);

        // Test mixed operations
        let numbers = vec![17, 8, 14];
        let operators = vec![Operator::Concat, Operator::Add];
        assert_eq!(evaluate_expression(&numbers, &operators), 192);

        // Test original cases
        let numbers = vec![10, 19];
        let operators = vec![Operator::Multiply];
        assert_eq!(evaluate_expression(&numbers, &operators), 190);
    }

    #[test]
    fn test_try_all_combinations() {
        // Test original cases
        assert!(try_all_combinations(190, &vec![10, 19]));
        assert!(try_all_combinations(3267, &vec![81, 40, 27]));
        assert!(try_all_combinations(292, &vec![11, 6, 16, 20]));
        
        // Test new cases with concatenation
        assert!(try_all_combinations(156, &vec![15, 6]));
        assert!(try_all_combinations(7290, &vec![6, 8, 6, 15]));
        assert!(try_all_combinations(192, &vec![17, 8, 14]));
    }
}
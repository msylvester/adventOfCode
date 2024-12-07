use std::fs::read_to_string;
use std::str::FromStr;

fn evaluate_expression(numbers: &[i64], operators: &[char]) -> i64 {
    let mut result = numbers[0];
    for (i, &op) in operators.iter().enumerate() {
        match op {
            '+' => result += numbers[i + 1],
            '*' => result *= numbers[i + 1],
            _ => panic!("Invalid operator"),
        }
    }
    result
}

fn try_all_combinations(test_value: i64, numbers: &[i64]) -> bool {
    if numbers.len() == 1 {
        return test_value == numbers[0];
    }

    let num_operators = numbers.len() - 1;
    // Try all possible combinations of operators
    for i in 0..(1 << num_operators) {
        let mut operators = Vec::with_capacity(num_operators);
        for j in 0..num_operators {
            // Use bit manipulation to generate combinations
            // 0 represents '+', 1 represents '*'
            operators.push(if (i & (1 << j)) != 0 { '*' } else { '+' });
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
    let input = read_to_string("input.txt")?;
    let result = solve_calibration(&input);
    println!("Solution: {}", result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate_expression() {
        let numbers = vec![10, 19];
        let operators = vec!['*'];
        assert_eq!(evaluate_expression(&numbers, &operators), 190);

        let numbers = vec![11, 6, 16, 20];
        let operators = vec!['+', '*', '+'];
        assert_eq!(evaluate_expression(&numbers, &operators), 292);
    }

    #[test]
    fn test_try_all_combinations() {
        assert!(try_all_combinations(190, &vec![10, 19]));
        assert!(try_all_combinations(3267, &vec![81, 40, 27]));
        assert!(try_all_combinations(292, &vec![11, 6, 16, 20]));
        assert!(!try_all_combinations(83, &vec![17, 5]));
    }
}
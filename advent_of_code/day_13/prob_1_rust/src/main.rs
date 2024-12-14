use std::fs::read_to_string;

type Point = (i64, i64);

/// Check if it's possible to reach the goal with the given movements
fn is_possible(a: Point, b: Point, goal: Point) -> Option<(bool, i64, i64)> {
    // Try all combinations up to 100 presses each (as per puzzle constraints)
    for i in 0..=100 {
        for j in 0..=100 {
            let x = a.0 * i + b.0 * j;
            let y = a.1 * i + b.1 * j;
            if x == goal.0 && y == goal.1 {
                return Some((true, i, j));
            }
        }
    }
    None
}

/// Calculate minimum tokens needed to reach goal
fn find_min(a: Point, b: Point, goal: Point) -> Option<i64> {
    is_possible(a, b, goal).map(|(_, num_a, num_b)| {
        // A costs 3 tokens, B costs 1 token
        (num_a * 3) + (num_b * 1)
    })
}

/// Parse button A movement from input string
fn parse_a(string_one: &str) -> Point {
    // Example: "Button A: X+94, Y+34"
    let parts: Vec<&str> = string_one.split(", ").collect();
    let x = parts[0].split('+').nth(1).unwrap().parse::<i64>().unwrap();
    let y = parts[1].split('+').nth(1).unwrap().parse::<i64>().unwrap();
    (x, y)
}

/// Parse button B movement from input string
fn parse_b(string_two: &str) -> Point {
    // Example: "Button B: X+22, Y+67"
    let parts: Vec<&str> = string_two.split(", ").collect();
    let x = parts[0].split('+').nth(1).unwrap().parse::<i64>().unwrap();
    let y = parts[1].split('+').nth(1).unwrap().parse::<i64>().unwrap();
    (x, y)
}

/// Parse goal coordinates from input string
fn parse_goal(string_three: &str) -> Point {
    // Example: "Prize: X=8400, Y=5400"
    let parts: Vec<&str> = string_three.split(", ").collect();
    let x = parts[0].split('=').nth(1).unwrap().parse::<i64>().unwrap();
    let y = parts[1].split('=').nth(1).unwrap().parse::<i64>().unwrap();
    (x, y)
}

/// Solve the entire puzzle, finding total tokens needed for all possible prizes
fn solve_puzzle(input_text: &str) -> i64 {
    // Split input into groups of 4 lines (3 lines of data + 1 blank)
    let machines: Vec<&str> = input_text.trim().split("\n\n").collect();
    let mut total_tokens = 0;

    for machine in machines {
        let lines: Vec<&str> = machine.split('\n').collect();
        let a_move = parse_a(lines[0]);
        let b_move = parse_b(lines[1]);
        let goal = parse_goal(lines[2]);

        if let Some(tokens) = find_min(a_move, b_move, goal) {
            total_tokens += tokens;
        }
    }

    total_tokens
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_text = read_to_string("input_final.txt")?;
    println!("Processing input...");
    let result = solve_puzzle(&input_text);
    println!("Total tokens needed: {}", result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_a() {
        let input = "Button A: X+94, Y+34";
        assert_eq!(parse_a(input), (94, 34));
    }

    #[test]
    fn test_parse_b() {
        let input = "Button B: X+22, Y+67";
        assert_eq!(parse_b(input), (22, 67));
    }

    #[test]
    fn test_parse_goal() {
        let input = "Prize: X=8400, Y=5400";
        assert_eq!(parse_goal(input), (8400, 5400));
    }

    #[test]
    fn test_is_possible() {
        let a = (2, 1);
        let b = (1, 2);
        let goal = (4, 4);
        assert!(is_possible(a, b, goal).is_some());
    }
}
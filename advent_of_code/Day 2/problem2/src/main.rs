use std::fs;

fn is_safe_report(levels: &[i32]) -> bool {
    if levels.len() < 2 {
        return true;
    }

    let diff = levels[1] - levels[0];
    let increasing = diff > 0;

    if diff.abs() < 1 || diff.abs() > 3 {
        return false;
    }

    levels.windows(2).all(|pair| {
        let curr_diff = pair[1] - pair[0];
        (increasing && curr_diff > 0 || !increasing && curr_diff < 0) 
            && curr_diff.abs() >= 1 
            && curr_diff.abs() <= 3
    })
}

fn is_safe_with_dampener(levels: &[i32]) -> bool {
    if is_safe_report(levels) {
        return true;
    }

    // Try removing each level
    for i in 0..levels.len() {
        let mut filtered: Vec<i32> = levels.iter().cloned().collect();
        filtered.remove(i);
        if is_safe_report(&filtered) {
            return true;
        }
    }
    false
}

fn solve(input: &str) -> usize {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .filter(|line| {
            let levels: Vec<i32> = line
                .split_whitespace()
                .filter_map(|n| n.parse().ok())
                .collect();
            is_safe_with_dampener(&levels)
        })
        .count()
}

fn main() -> std::io::Result<()> {
    let input = fs::read_to_string("input/day2.txt")?;
    println!("Safe reports with dampener: {}", solve(&input));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "7 6 4 2 1\n1 2 7 8 9\n9 7 6 2 1\n1 3 2 4 5\n8 6 4 4 1\n1 3 6 7 9\n";
        assert_eq!(solve(input), 4);
    }
}
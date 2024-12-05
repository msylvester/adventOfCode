use std::collections::HashSet;
use std::fs;

#[derive(Debug)]
struct Grid {
    data: Vec<Vec<char>>,
    height: usize,
    width: usize,
}

impl Grid {
    fn new(input: &str) -> Self {
        let data: Vec<Vec<char>> = input
            .lines()
            .map(|line| line.chars().collect())
            .collect();
        let height = data.len();
        let width = if height > 0 { data[0].len() } else { 0 };
        Grid { data, height, width }
    }

    fn is_valid_pos(&self, row: i32, col: i32) -> bool {
        row >= 0 && row < self.height as i32 && col >= 0 && col < self.width as i32
    }

    fn get_char(&self, row: i32, col: i32) -> Option<char> {
        if self.is_valid_pos(row, col) {
            Some(self.data[row as usize][col as usize])
        } else {
            None
        }
    }

    fn check_mas(&self, row: i32, col: i32, delta_row: i32, delta_col: i32) -> bool {
        let positions = [(0, 0), (1, 1), (2, 2)];
        let expected = ['M', 'A', 'S'];
        
        positions.iter().zip(expected.iter()).all(|((dr, dc), &expected_char)| {
            let r = row + dr * delta_row;
            let c = col + dc * delta_col;
            self.get_char(r, c) == Some(expected_char)
        })
    }

    fn find_xmas_patterns(&self) -> usize {
        let mut found_patterns = HashSet::new();

        for row in 0..self.height as i32 {
            for col in 0..self.width as i32 {
                // Check all possible X patterns
                let patterns = [
                    // Forward MAS in both directions
                    ((1, 1), (1, -1)),
                    // Forward MAS in first direction, backward in second
                    ((1, 1), (-1, -1)),
                    // Backward MAS in first direction, forward in second
                    ((-1, 1), (1, -1)),
                    // Backward MAS in both directions
                    ((-1, 1), (-1, -1)),
                ];

                for &(dir1, dir2) in &patterns {
                    if self.check_mas(row, col, dir1.0, dir1.1) && 
                       self.check_mas(row, col, dir2.0, dir2.1) {
                        found_patterns.insert((row, col));
                    }
                }
            }
        }

        found_patterns.len()
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read input from file
    let input = fs::read_to_string("input.txt")?;
    
    let grid = Grid::new(&input);
    let count = grid.find_xmas_patterns();
    println!("Found {} X-MAS patterns", count);
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_case() {
        let input = ".M.S......\n\
                    .A..MSMS.\n\
                    .M.S.MAA..\n\
                    ..A.ASMSM.\n\
                    .M.S.M....\n\
                    ..........\n\
                    S.S.S.S.S.\n\
                    .A.A.A.A..\n\
                    M.M.M.M.M.\n\
                    ..........";
        
        let grid = Grid::new(input);
        assert_eq!(grid.find_xmas_patterns(), 9);
    }
}
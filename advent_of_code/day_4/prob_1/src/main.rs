use std::fs::read_to_string;
use std::io;

fn count_xmas(grid: &[String]) -> i32 {
    if grid.is_empty() {
        return 0;
    }

    let height = grid.len();
    let width = grid[0].len();

    let directions = [
        (0, 1),   // right
        (0, -1),  // left
        (1, 0),   // down
        (-1, 0),  // up
        (1, 1),   // down-right
        (1, -1),  // down-left
        (-1, 1),  // up-right
        (-1, -1)  // up-left
    ];

    let mut count = 0;

    for y in 0..height {
        for x in 0..width {
            for &(dy, dx) in directions.iter() {
                count += check_xmas(grid, x, y, dx, dy, width, height) as i32;
            }
        }
    }
    count
}

fn check_xmas(grid: &[String], x: usize, y: usize, dx: i32, dy: i32, width: usize, height: usize) -> bool {
    let chars: [char; 4] = ['X', 'M', 'A', 'S'];
    
    for i in 0..4 {
        let new_x = x as i32 + (dx * i);
        let new_y = y as i32 + (dy * i);
        
        if new_x < 0 || new_x >= width as i32 || new_y < 0 || new_y >= height as i32 {
            return false;
        }
        
        let current_char = grid[new_y as usize].chars().nth(new_x as usize).unwrap_or('\0');
        if current_char != chars[i as usize] {  // Fixed: convert i to usize
            return false;
        }
    }
    
    true
}
fn main() -> io::Result<()> {
    // Read from file and create grid
    let content = read_to_string("input.txt")?;
    
    // Split by whitespace and collect directly into Vec<String>
    let grid: Vec<String> = content
        .lines()
        .map(|line| line.trim().to_string())
        .collect();

    let result = count_xmas(&grid);
    println!("Number of XMAS patterns found: {}", result);

    Ok(())
}

// fn main() {

//     //TODO: REPLACE THE CONSTANT WITH THE TEXT FILE

//     let grid: Vec<String> = vec![
//         "MMMSXXMASM".to_string(),
//         "MSAMXMSMSA".to_string(),
//         "AMXSXMAAMM".to_string(),
//         "MSAMASMSMX".to_string(),
//         "XMASAMXAMM".to_string(),
//         "XXAMMXXAMA".to_string(),
//         "SMSMSASXSS".to_string(),
//         "SAXAMASAAA".to_string(),
//         "MAMMMXMMMM".to_string(),
//         "MXMXAXMASX".to_string(),
//     ];

//     let result = count_xmas(&grid);
//     println!("Number of XMAS patterns found: {}", result);
// }
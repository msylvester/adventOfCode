use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type Position = (i32, i32);

fn calculate_antinodes(grid: &[String]) -> usize {
    let mut antinodes: HashSet<Position> = HashSet::new();
    
    let n = grid.len() as i32;
    let m = grid[0].len() as i32;
    
    // Dictionary to store positions of each frequency
    let mut nodes: HashMap<char, Vec<Position>> = HashMap::new();
    
    // Find all antennas and group them by frequency
    for i in 0..n {
        for (j, ch) in grid[i as usize].chars().enumerate() {
            if ch != '.' {
                nodes.entry(ch)
                    .or_insert_with(Vec::new)
                    .push((i, j as i32));
            }
        }
    }
    
    fn add_antinodes(
        point1: Position,
        point2: Position,
        n: i32,
        m: i32,
        antinodes: &mut HashSet<Position>
    ) {
        let (x1, y1) = point1;
        let (x2, y2) = point2;
        
        // Calculate direction vector
        let dx = x2 - x1;
        let dy = y2 - y1;
        
        // Start from second point
        let mut current_x = x2;
        let mut current_y = y2;
        
        // Add the antenna position as an antinode
        antinodes.insert((x2, y2));
        
        // Keep extending in the same direction until we hit grid boundary
        loop {
            current_x += dx;
            current_y += dy;
            
            // Check if we're still within grid bounds
            if current_x >= 0 && current_x < n && 
               current_y >= 0 && current_y < m {
                antinodes.insert((current_x, current_y));
            } else {
                break;
            }
        }
    }
    
    // Process each frequency group
    for (_frequency, antenna_list) in nodes.iter() {
        // For each pair of same-frequency antennas
        for i in 0..antenna_list.len() {
            for j in 0..i {
                // Calculate antinodes in both directions
                add_antinodes(antenna_list[i], antenna_list[j], n, m, &mut antinodes);
                add_antinodes(antenna_list[j], antenna_list[i], n, m, &mut antinodes);
            }
        }
    }
    
    antinodes.len()
}

fn read_input(filename: &str) -> io::Result<Vec<String>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    
    Ok(reader
        .lines()
        .filter_map(|line| line.ok())
        .filter(|line| !line.is_empty())
        .collect())
}

fn main() -> io::Result<()> {
    println!("Reading input file...");
    
    match read_input("input_two.txt") {
        Ok(grid) => {
            if grid.is_empty() {
                println!("Error: Input file is empty");
                return Ok(());
            }
            
            println!("Grid loaded, calculating antinodes...");
            let result = calculate_antinodes(&grid);
            println!("Total antinodes: {}", result);
        }
        Err(e) => {
            println!("Error reading input file: {}", e);
        }
    }
    
    Ok(())
}
use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type Point = (i32, i32);
type Region = HashSet<Point>;

/// Gets connected regions of same characters in a 2D grid
fn get_regions(grid: &Vec<Vec<char>>) -> Vec<Region> {
    if grid.is_empty() || grid[0].is_empty() {
        return vec![];
    }

    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;
    let mut visited = HashSet::new();
    let mut regions = Vec::new();

    // DFS implementation using a stack
    fn dfs(
        r: i32, 
        c: i32, 
        letter: char, 
        grid: &Vec<Vec<char>>, 
        visited: &mut HashSet<Point>,
        current_region: &mut Region,
        rows: i32,
        cols: i32
    ) {
        let mut stack = VecDeque::new();
        stack.push_back((r, c));

        while let Some((r, c)) = stack.pop_back() {
            if r < 0 || r >= rows || c < 0 || c >= cols || 
               grid[r as usize][c as usize] != letter || 
               visited.contains(&(r, c)) {
                continue;
            }

            visited.insert((r, c));
            current_region.insert((r, c));

            // Check all 4 directions
            let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
            for (dr, dc) in directions.iter() {
                stack.push_back((r + dr, c + dc));
            }
        }
    }

    // Iterate through each cell in the grid
    for i in 0..rows {
        for j in 0..cols {
            if !visited.contains(&(i, j)) {
                let mut current_region = HashSet::new();
                dfs(
                    i,
                    j,
                    grid[i as usize][j as usize],
                    grid,
                    &mut visited,
                    &mut current_region,
                    rows,
                    cols,
                );
                regions.push(current_region);
            }
        }
    }

    regions
}

/// Calculates the perimeter of a region by counting exposed sides
fn calc_perimeter(region: &Region) -> i32 {
    let mut perimeter = 0;
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    for &(r, c) in region.iter() {
        for (dr, dc) in directions.iter() {
            if !region.contains(&(r + dr, c + dc)) {
                perimeter += 1;
            }
        }
    }

    perimeter
}

/// Calculates the total price of fencing all regions
fn calculate_total_price(grid: &Vec<Vec<char>>) -> i32 {
    let regions = get_regions(grid);
    let mut total_price = 0;

    for region in regions {
        let area = region.len() as i32;
        let perimeter = calc_perimeter(&region);
        let price = area * perimeter;
        total_price += price;

        // Uncomment for debugging
        // if let Some(&(r, c)) = region.iter().next() {
        //     println!(
        //         "Region {}: Area={}, Perimeter={}, Price={}", 
        //         grid[r as usize][c as usize], area, perimeter, price
        //     );
        // }
    }

    total_price
}

/// Reads the grid from a file
fn read_grid(filename: &str) -> io::Result<Vec<Vec<char>>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut grid = Vec::new();

    for line in reader.lines() {
        grid.push(line?.chars().collect());
    }

    Ok(grid)
}

fn main() -> io::Result<()> {
    // Read input from file
    let grid = read_grid("input_three.txt")?;

    // Calculate and print the total price
    let total = calculate_total_price(&grid);
    println!("Total price of fencing all regions: {}", total);

    Ok(())
}
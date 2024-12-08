use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type Position = (i32, i32);

fn collect_antennas(grid: &[String]) -> HashMap<char, Vec<Position>> {
    let mut nodes = HashMap::new();
    
    for (i, row) in grid.iter().enumerate() {
        for (j, ch) in row.chars().enumerate() {
            if ch != '.' {
                nodes.entry(ch)
                    .or_insert_with(Vec::new)
                    .push((i as i32, j as i32));
            }
        }
    }
    nodes
}

fn add_antinode(node1: Position, node2: Position, n: i32, m: i32, antinodes: &mut HashSet<Position>) {
    let (x1, y1) = node1;
    let (x2, y2) = node2;
    let newx = x2 + (x2 - x1);
    let newy = y2 + (y2 - y1);
    
    if 0 <= newx && newx < n && 0 <= newy && newy < m {
        antinodes.insert((newx, newy));
    }
}

fn find_antinodes(grid: &[String]) -> usize {
    let n = grid.len() as i32;
    let m = grid[0].len() as i32;
    
    let nodes = collect_antennas(grid);
    let mut antinodes = HashSet::new();
    
    // Process each frequency group
    for (frequency, node_list) in nodes.iter() {
        println!("\nProcessing frequency: {}", frequency);
        println!("Positions for frequency {}: {:?}", frequency, node_list);
        
        for i in 0..node_list.len() {
            for j in 0..i {
                println!("  Checking pair: {:?} -> {:?}", node_list[i], node_list[j]);
                add_antinode(node_list[i], node_list[j], n, m, &mut antinodes);
                println!("  Checking pair: {:?} -> {:?}", node_list[j], node_list[i]);
                add_antinode(node_list[j], node_list[i], n, m, &mut antinodes);
                println!("  Current antinodes: {:?}", antinodes);
            }
        }
    }
    
    antinodes.len()
}

fn main() -> io::Result<()> {
    println!("Reading input file...");
    
    // Read and process the input file
    let file = File::open("input_two.txt")?;
    let reader = BufReader::new(file);
    
    let grid: Vec<String> = reader
        .lines()
        .filter_map(|line| line.ok())
        .filter(|line| !line.is_empty())
        .collect();
    
    if grid.is_empty() {
        println!("Error: Input file is empty");
        return Ok(());
    }
    
    println!("Grid loaded, processing antinodes...");
    let result = find_antinodes(&grid);
    println!("\nTotal antinodes: {}", result);
    
    Ok(())
}
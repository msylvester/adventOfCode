use std::fs::read_to_string;
use std::io;


fn main() -> io::Result<()> {
    // Read from file and create grid
    let content = read_to_string("input.txt")?;
    
    // Split by whitespace and collect directly into Vec<String>
    let grid: Vec<String> = content
        .lines()
        .map(|line| line.trim().to_string())
        .collect();

    // let result = count_xmas(&grid);
    // println!("Number of XMAS patterns found: {}", result);

    Ok(())
}

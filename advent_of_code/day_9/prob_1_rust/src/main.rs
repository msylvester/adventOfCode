use std::fs::File;
use std::io::{self, Read};

fn read_input(file_path: &str) -> io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn parse_input(input_str: &str) -> Vec<u32> {
    input_str
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect()
}

fn create_space_and_blocks(data: &[u32]) -> Vec<Option<usize>> {
    let mut fs = Vec::new();
    let mut free_space = false;
    let mut id = 0;
    
    for &num in data {
        if free_space {
            // Append None to represent free spaces
            for _ in 0..num {
                fs.push(None);
            }
            free_space = false;
        } else {
            // Append block IDs to represent blocks
            for _ in 0..num {
                fs.push(Some(id));
            }
            id += 1;
            free_space = true;
        }
    }
    fs
}

fn move_blocks(fs: &mut Vec<Option<usize>>) {
    for i in (0..fs.len()).rev() {
        if fs[i].is_none() {
            continue;
        }
        
        let block_id = fs[i];
        fs[i] = None;
        
        // Find the first free space
        for j in 0..fs.len() {
            if fs[j].is_none() {
                fs[j] = block_id;
                break;
            }
        }
    }
}

fn calculate_checksum(fs: &[Option<usize>]) -> usize {
    let mut checksum = 0;
    let mut i = 0;
    
    while i < fs.len() && fs[i].is_some() {
        checksum += i * fs[i].unwrap();
        i += 1;
    }
    
    checksum
}

fn main() -> io::Result<()> {
    // Step 1: Read the input from a file
    let input_str = read_input("input_two.txt")?;

    // Step 2: Parse the input into a vector of integers
    let data = parse_input(&input_str);

    // Step 3: Create the initial list of blocks and free spaces
    let mut fs = create_space_and_blocks(&data);

    // Step 4: Move the blocks to free spaces
    move_blocks(&mut fs);

    // Step 5: Calculate and print the checksum
    let checksum = calculate_checksum(&fs);
    println!("{}", checksum);

    Ok(())
}
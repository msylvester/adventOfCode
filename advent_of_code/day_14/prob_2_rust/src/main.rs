use image::{ImageBuffer, Luma};
use std::fs;

fn print_map(facility: &Vec<Vec<i32>>, width: usize, height: usize) {
    // Create a new grayscale image
    let mut img = ImageBuffer::new(width as u32, height as u32);

    for y in 0..height {
        for x in 0..width {
            let pixel = if facility[y][x] == 0 { 0 } else { 255 };
            img.put_pixel(x as u32, y as u32, Luma([pixel as u8]));
        }
    }

    // Save the image
    img.save("tree.bmp").expect("Failed to save image");
}

fn main() {
    // Constants
    const WIDTH: i32 = 101;
    const HEIGHT: i32 = 103;

    // Read input file
    let content = fs::read_to_string("./input.txt")
        .expect("Failed to read input file");

    // Parse robots data
    let mut robots: Vec<Vec<i32>> = content
        .lines()
        .map(|line| {
            line.split(|c: char| !c.is_digit(10) && c != '-')
                .filter(|s| !s.is_empty())
                .map(|s| s.parse().unwrap())
                .collect()
        })
        .collect();

    // Initialize facility grid
    let mut facility = vec![vec![0; WIDTH as usize]; HEIGHT as usize];

    // Set initial positions
    for robot in &robots {
        let (p_x, p_y) = (robot[0], robot[1]);
        facility[p_y as usize][p_x as usize] += 1;
    }

    // Main simulation loop
    let mut iteration = 1;
    loop {
        for j in 0..robots.len() {
            let (mut p_x, mut p_y, v_x, v_y) = (
                robots[j][0],
                robots[j][1],
                robots[j][2],
                robots[j][3],
            );

            // Remove current robot position
            facility[p_y as usize][p_x as usize] -= 1;

            // Calculate new position
            p_x = (p_x + v_x).rem_euclid(WIDTH);
            p_y = (p_y + v_y).rem_euclid(HEIGHT);

            // Update robot position
            robots[j][0] = p_x;
            robots[j][1] = p_y;

            // Record new position
            facility[p_y as usize][p_x as usize] += 1;
        }

        // Check if all robots are in unique positions
        let max_count = facility
            .iter()
            .flat_map(|row| row.iter())
            .max()
            .unwrap_or(&0);

        if *max_count == 1 {
            break;
        }

        iteration += 1;
    }

    // Create tree image and print iteration count
    print_map(&facility, WIDTH as usize, HEIGHT as usize);
    println!("Tree found at iteration: {}", iteration);
}
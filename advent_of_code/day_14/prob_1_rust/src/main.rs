use std::collections::HashMap;
use std::fs;

fn main() {
    // Constants
    const WIDTH: i32 = 101;
    const HEIGHT: i32 = 103;
    const X_MID: i32 = WIDTH / 2;
    const Y_MID: i32 = HEIGHT / 2;

    // Read input file
    let content = fs::read_to_string("./input.txt")
        .expect("Failed to read input file");

    // Initialize facility hashmap
    let mut facility: HashMap<(i32, i32), i32> = HashMap::new();
    for i in 0..WIDTH {
        for j in 0..HEIGHT {
            facility.insert((i, j), 0);
        }
    }

    // Process each robot
    for line in content.lines() {
        // Parse numbers from the line
        let numbers: Vec<i32> = line
            .split(|c: char| !c.is_digit(10) && c != '-')  // Using base 10
            .filter(|s| !s.is_empty())
            .map(|s| s.parse().unwrap())
            .collect();

        if numbers.len() != 4 {
            continue;
        }

        let (mut p_x, mut p_y, v_x, v_y) = (numbers[0], numbers[1], numbers[2], numbers[3]);

        // Run 100 iterations
        p_x = (p_x + 100 * v_x).rem_euclid(WIDTH);
        p_y = (p_y + 100 * v_y).rem_euclid(HEIGHT);

        // Record final position
        *facility.entry((p_x, p_y)).or_insert(0) += 1;
    }

    // Count robots in each quadrant
    let mut quadrants = [0; 4];
    for ((x, y), count) in facility.iter() {
        if *count > 0 {
            if *x < X_MID && *y < Y_MID {
                quadrants[0] += count;
            } else if *x > X_MID && *y < Y_MID {
                quadrants[1] += count;
            } else if *x > X_MID && *y > Y_MID {
                quadrants[2] += count;
            } else if *x < X_MID && *y > Y_MID {
                quadrants[3] += count;
            }
        }
    }

    // Calculate and print the product
    let product: i32 = quadrants.iter().product();
    println!("{}", product);
}
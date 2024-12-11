use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Self {
        Position { x, y }
    }
}

struct Grid {
    data: Vec<Vec<i32>>,
    rows: i32,
    cols: i32,
}

impl Grid {
    fn from_file(filename: &str) -> io::Result<Self> {
        let file = File::open(filename)?;
        let reader = io::BufReader::new(file);
        let mut data = Vec::new();

        for line in reader.lines() {
            let row: Vec<i32> = line?
                .chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect();
            data.push(row);
        }

        let rows = data.len() as i32;
        let cols = data[0].len() as i32;

        Ok(Grid { data, rows, cols })
    }

    fn is_valid(&self, pos: Position) -> bool {
        pos.x >= 0 && pos.x < self.rows && pos.y >= 0 && pos.y < self.cols
    }

    fn get_value(&self, pos: Position) -> i32 {
        self.data[pos.x as usize][pos.y as usize]
    }

    fn find_zeros(&self) -> Vec<Position> {
        let mut zeros = Vec::new();
        for i in 0..self.rows {
            for j in 0..self.cols {
                if self.data[i as usize][j as usize] == 0 {
                    zeros.push(Position::new(i, j));
                }
            }
        }
        zeros
    }
}

fn calc_paths_to_nine(start: Position, grid: &Grid) -> usize {
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut nine_positions = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((start, vec![start]));

    while let Some((pos, path)) = queue.pop_front() {
        let current_height = grid.get_value(pos);

        if current_height == 9 {
            nine_positions.insert(pos);
            continue;
        }

        for (dx, dy) in directions.iter() {
            let new_pos = Position::new(pos.x + dx, pos.y + dy);

            if !grid.is_valid(new_pos) 
                || path.contains(&new_pos)
                || grid.get_value(new_pos) != current_height + 1 {
                continue;
            }

            let mut new_path = path.clone();
            new_path.push(new_pos);
            queue.push_back((new_pos, new_path));
        }
    }

    nine_positions.len()
}

fn solve_hiking_trails(filename: &str) -> io::Result<usize> {
    let grid = Grid::from_file(filename)?;
    let zero_locations = grid.find_zeros();
    
    println!("Zero locations: {:?}", zero_locations.len());
    
    let total_score = zero_locations
        .iter()
        .map(|&pos| calc_paths_to_nine(pos, &grid))
        .sum();

    Ok(total_score)
}

fn main() -> io::Result<()> {
    let result = solve_hiking_trails("final_input.txt")?;
    println!("Sum of trailhead scores: {}", result);
    Ok(())
}
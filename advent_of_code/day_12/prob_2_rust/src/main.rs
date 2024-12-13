use std::collections::{HashMap, HashSet};
use std::time::Instant;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug)]
struct Cell {
    visited: bool,
    perimeter: i32,
    region_id: Option<usize>,
    label: char,
    loc: (i32, i32),
    nbrs: Vec<(i32, i32)>,
}

impl Cell {
    fn new(label: char) -> Self {
        Cell {
            visited: false,
            perimeter: 0,
            region_id: None,
            label,
            loc: (0, 0),
            nbrs: Vec::new(),
        }
    }
}

#[derive(Debug)]
struct Region {
    cells: HashSet<(i32, i32)>,
    id: usize,
    walls: i32,
    label: char,
}

impl Region {
    fn new(id: usize) -> Self {
        Region {
            cells: HashSet::new(),
            id,
            walls: 0,
            label: '.',
        }
    }

    fn add_cell(&mut self, cell_loc: (i32, i32)) {
        self.cells.insert(cell_loc);
    }

    fn perimeter(&self, cells: &HashMap<(i32, i32), Cell>) -> i32 {
        self.cells.iter()
            .map(|loc| cells.get(loc).unwrap().perimeter)
            .sum()
    }

    fn area(&self) -> i32 {
        self.cells.len() as i32
    }
}

fn process_grid(filename: &str) -> io::Result<(i32, i32)> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let grid: Vec<Vec<char>> = reader
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();

    let nrows = grid.len() as i32;
    let ncols = grid[0].len() as i32;
    let mut cells: HashMap<(i32, i32), Cell> = HashMap::new();
    let mut regions: HashMap<usize, Region> = HashMap::new();

    // Create border region
    let border = Region::new(0);
    regions.insert(0, border);

    // Initialize cells including border
    for row in -1..=nrows {
        for col in -1..=ncols {
            let mut cell = if row < 0 || row >= nrows || col < 0 || col >= ncols {
                let mut cell = Cell::new('.');
                cell.visited = true;
                cell.region_id = Some(0);
                cell
            } else {
                Cell::new(grid[row as usize][col as usize])
            };
            cell.loc = (row, col);
            cells.insert((row, col), cell);
        }
    }

    // Identify neighbors and calculate perimeters
    let mut nbrs_map: HashMap<(i32, i32), Vec<(i32, i32)>> = HashMap::new();
    for row in 0..nrows {
        for col in 0..ncols {
            let cell_label = cells[&(row, col)].label;
            let mut neighbor_count = 0;
            let mut nbrs = Vec::new();
            
            for (dr, dc) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
                let nr = row + dr;
                let nc = col + dc;
                if let Some(neighbor) = cells.get(&(nr, nc)) {
                    if neighbor.label == cell_label {
                        nbrs.push((nr, nc));
                        neighbor_count += 1;
                    }
                }
            }
            
            cells.get_mut(&(row, col)).unwrap().perimeter = 4 - neighbor_count;
            nbrs_map.insert((row, col), nbrs);
        }
    }

    // Update all neighbors at once
    for ((row, col), nbrs) in nbrs_map {
        if let Some(cell) = cells.get_mut(&(row, col)) {
            cell.nbrs = nbrs;
        }
    }

    // Find regions
    let mut next_region_id = 1;
    for row in 0..nrows {
        for col in 0..ncols {
            if !cells[&(row, col)].visited {
                let mut region = Region::new(next_region_id);
                region.label = cells[&(row, col)].label;
                
                let mut stack = vec![(row, col)];
                cells.get_mut(&(row, col)).unwrap().visited = true;
                cells.get_mut(&(row, col)).unwrap().region_id = Some(next_region_id);
                region.add_cell((row, col));

                while let Some((r, c)) = stack.pop() {
                    let nbrs = cells[&(r, c)].nbrs.clone();
                    for (nr, nc) in nbrs {
                        if !cells[&(nr, nc)].visited {
                            cells.get_mut(&(nr, nc)).unwrap().visited = true;
                            cells.get_mut(&(nr, nc)).unwrap().region_id = Some(next_region_id);
                            region.add_cell((nr, nc));
                            stack.push((nr, nc));
                        }
                    }
                }
                
                regions.insert(next_region_id, region);
                next_region_id += 1;
            }
        }
    }

    // Calculate Part A score
    let score_a = regions.values()
        .map(|region| region.area() * region.perimeter(&cells))
        .sum();

    // Part B: Count walls between regions
    for row in 0..nrows {
        // Northern walls
        let mut in_wall = false;
        let mut cur_n = None;
        let mut cur_s = None;

        for col in 0..ncols {
            let next_n = cells[&(row - 1, col)].region_id;
            let next_s = cells[&(row, col)].region_id;
            
            if next_n != next_s && (!in_wall || (in_wall && next_s != cur_s)) {
                in_wall = true;
                if let Some(region_id) = next_s {
                    regions.get_mut(&region_id).unwrap().walls += 1;
                }
            }
            cur_n = next_n;
            cur_s = next_s;
            if cur_n == cur_s {
                in_wall = false;
            }
        }

        // Southern walls
        let mut in_wall = false;
        let mut cur_n = None;
        let mut cur_s = None;

        for col in 0..ncols {
            let next_n = cells[&(row, col)].region_id;
            let next_s = cells[&(row + 1, col)].region_id;
            
            if next_n != next_s && (!in_wall || (in_wall && next_n != cur_n)) {
                in_wall = true;
                if let Some(region_id) = next_n {
                    regions.get_mut(&region_id).unwrap().walls += 1;
                }
            }
            cur_n = next_n;
            cur_s = next_s;
            if cur_n == cur_s {
                in_wall = false;
            }
        }
    }

    // Scan vertical walls
    for col in 0..ncols {
        // Western walls
        let mut in_wall = false;
        let mut cur_w = None;
        let mut cur_e = None;

        for row in 0..nrows {
            let next_w = cells[&(row, col - 1)].region_id;
            let next_e = cells[&(row, col)].region_id;
            
            if next_w != next_e && (!in_wall || (in_wall && next_e != cur_e)) {
                in_wall = true;
                if let Some(region_id) = next_e {
                    regions.get_mut(&region_id).unwrap().walls += 1;
                }
            }
            cur_w = next_w;
            cur_e = next_e;
            if cur_w == cur_e {
                in_wall = false;
            }
        }

        // Eastern walls
        let mut in_wall = false;
        let mut cur_w = None;
        let mut cur_e = None;

        for row in 0..nrows {
            let next_w = cells[&(row, col)].region_id;
            let next_e = cells[&(row, col + 1)].region_id;
            
            if next_w != next_e && (!in_wall || (in_wall && next_w != cur_w)) {
                in_wall = true;
                if let Some(region_id) = next_w {
                    regions.get_mut(&region_id).unwrap().walls += 1;
                }
            }
            cur_w = next_w;
            cur_e = next_e;
            if cur_w == cur_e {
                in_wall = false;
            }
        }
    }

    // Calculate Part B score
    let score_b = regions.values()
        .map(|region| region.walls * region.area())
        .sum();

    Ok((score_a, score_b))
}

fn main() -> io::Result<()> {
    let start = Instant::now();
    let (score_a, score_b) = process_grid("input_three.txt")?;
    let duration = start.elapsed();

    println!("Part A: Score = {}", score_a);
    println!("Part B: Score = {}", score_b);
    println!("Total execution time = {:.2} ms", duration.as_secs_f64() * 1000.0);

    Ok(())
}
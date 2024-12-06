use std::fs;
use std::collections::{HashMap, HashSet, VecDeque};


fn has_cycle(graph: &HashMap<u32, Vec<u32>>, start: u32, visited: &mut HashSet<u32>, stack: &mut HashSet<u32>) -> bool {
    if stack.contains(&start) {
        return true;
    }
    if visited.contains(&start) {
        return false;
    }
    
    visited.insert(start);
    stack.insert(start);
    
    if let Some(neighbors) = graph.get(&start) {
        for &neighbor in neighbors {
            if has_cycle(graph, neighbor, visited, stack) {
                return true;
            }
        }
    }
    
    stack.remove(&start);
    false
}

fn is_valid_order(numbers: &[u32], rules: &[(u32, u32)]) -> bool {
    let mut graph: HashMap<u32, Vec<u32>> = HashMap::new();
    
    for &num in numbers {
        graph.entry(num).or_insert(Vec::new());
    }
    
    for i in 0..numbers.len() {
        for j in i + 1..numbers.len() {
            let a = numbers[i];
            let b = numbers[j];
            
            for &(before, after) in rules {
                if a == after && b == before {
                    return false;
                }
            }
            
            graph.entry(a).or_default().push(b);
        }
    }
    
    let mut visited = HashSet::new();
    let mut stack = HashSet::new();
    
    for &node in numbers {
        if !visited.contains(&node) {
            if has_cycle(&graph, node, &mut visited, &mut stack) {
                return false;
            }
        }
    }
    
    true
}

fn topological_sort(numbers: &[u32], rules: &[(u32, u32)]) -> Vec<u32> {
    let mut graph: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut in_degree: HashMap<u32, usize> = HashMap::new();
    
    // Initialize nodes
    for &num in numbers {
        graph.entry(num).or_insert(Vec::new());
        in_degree.insert(num, 0);
    }
    
    // Build graph based on rules
    for &num1 in numbers {
        for &num2 in numbers {
            if num1 != num2 {
                let mut should_add_edge = false;
                for &(before, after) in rules {
                    if num1 == before && num2 == after {
                        should_add_edge = true;
                        break;
                    }
                }
                if should_add_edge {
                    graph.entry(num1).or_default().push(num2);
                    *in_degree.entry(num2).or_default() += 1;
                }
            }
        }
    }
    
    // Perform topological sort using Kahn's algorithm
    let mut sorted = Vec::new();
    let mut queue = VecDeque::new();
    
    // Add all vertices with in-degree 0 to queue
    for &num in numbers {
        if in_degree[&num] == 0 {
            queue.push_back(num);
        }
    }
    
    while let Some(node) = queue.pop_front() {
        sorted.push(node);
        
        if let Some(neighbors) = graph.get(&node) {
            for &neighbor in neighbors {
                *in_degree.get_mut(&neighbor).unwrap() -= 1;
                if in_degree[&neighbor] == 0 {
                    queue.push_back(neighbor);
                }
            }
        }
    }
    
    sorted
}

fn main() -> std::io::Result<()> {
    let input_rules = match fs::read_to_string("/Users/mikress/advent_of_code/advent_of_code/day_5/prob_2/input.txt") {
        Ok(contents) => contents,
        Err(e) => {
            println!("Error reading input.txt: {}", e);
            return Err(e);
        }
    };

    let input_order = match fs::read_to_string("/Users/mikress/advent_of_code/advent_of_code/day_5/prob_2/input_two.txt") {
        Ok(contents) => contents,
        Err(e) => {
            println!("Error reading input_two.txt: {}", e);
            return Err(e);
        }
    };

    // Parse rules (number|number format)
    let vector_rules: Vec<(u32, u32)> = input_rules
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let parts: Vec<&str> = line.trim().split('|').collect();
            if parts.len() != 2 {
                panic!("Invalid rule format: {}", line);
            }
            (
                parts[0].trim().parse().expect("Invalid first number"),
                parts[1].trim().parse().expect("Invalid second number")
            )
        })
        .collect();

    // Parse orders (comma-separated numbers)
    let orders: Vec<Vec<u32>> = input_order
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            line.trim()
                .split(',')
                .map(|n| n.trim().parse().unwrap())
                .collect()
        })
        .collect();

    let mut sum = 0;
    
    for numbers in orders {
        if !is_valid_order(&numbers, &vector_rules) {
            let sorted_order = topological_sort(&numbers, &vector_rules);
            
            if sorted_order.len() >= 3 {
                let middle_index = sorted_order.len() / 2;
                println!("Original invalid order: {:?}", numbers);
                println!("Corrected order: {:?}", sorted_order);
                println!("Middle number: {}", sorted_order[middle_index]);
                sum += sorted_order[middle_index];
            }
        }
    }

    println!("Sum of middle numbers from corrected invalid orders: {}", sum);
    Ok(()) // Added this line
}
# 🏰 Day 12: Garden Fencing Challenge - Part B 🌳

## 🎯 Problem Description
The garden needs special walls placed between regions! After analyzing the regions in Part A, we now need to count the distinct walls between different regions and calculate a new cost based on these walls.

## 🧩 Part B: Calculating Wall Costs
For each region, we need to:
- 🧱 Count distinct walls between different regions (no double counting!)
- 📏 Calculate area of each region
- 💰 Multiply walls × area for each region
- ➕ Sum all costs for final answer

### Example
```
AABB
AABB
CCDD
CCDD
```
In this grid:
- Each letter forms a region
- Regions A and B share 1 wall
- Regions A and C share 1 wall
- Regions B and D share 1 wall
- Regions C and D share 1 wall
- Each region has an area of 4
- Cost per region = walls × area
- Total cost = sum of all region costs

## 🔍 Solution Approach
1. First identify all regions using flood fill
2. Scan grid in all 4 directions (North, South, East, West)
3. For each direction:
   - Track when walls start and end
   - Count unique walls between regions
   - Ensure no double counting of walls
4. Calculate cost for each region using: area × number of walls
5. Sum all costs for final answer

## ✨ The Answer
The total cost of walls between regions is: **830516**

## 🚀 Running the Code
```bash
# Python version
python garden_walls.py

# Rust version
cargo run
```

## 💡 Implementation Notes
- Uses a wall tracking system to avoid double counting
- Handles borders correctly by adding padding around grid
- Efficiently calculates wall counts in a single pass per direction
- Works with any grid size and any character labels

## 🎮 Performance
- Time complexity: O(rows × cols) for grid traversal
- Space complexity: O(rows × cols) for region tracking
- Efficient wall counting without redundant calculations

## 🔗 Related Problems
- Part A: Initial region perimeter calculation
- Both parts involve region identification and costing
- Part B builds on Part A's region detection

## 🎉 Fun Fact
The problem tests understanding of:
- Region connectivity in 2D grids
- Efficient wall counting algorithms
- Handling of shared boundaries
- Grid traversal techniques
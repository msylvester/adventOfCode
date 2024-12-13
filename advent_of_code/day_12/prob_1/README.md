# 🌳 Day 12: Garden Fencing Challenge 🏡

## Description
It's time to build fences in our garden clusters! You're given a grid where each letter represents a different garden plot. Connected plots of the same letter form a region that needs to be fenced.

## 🎯 Part A: Calculating Fencing Costs
Each garden region needs to be fenced individually, and the cost of fencing a region is calculated by:
- 📏 Finding the perimeter of the region (counting exposed edges)
- 🌱 Counting the area (number of cells) in the region
- 💰 Multiplying perimeter × area to get the region's cost

The final answer is the sum of costs for all regions.

### Example
```
AABB
AABB
CCDD
CCDD
```
In this grid:
- Each letter represents a different garden region
- Each region has an area of 4 cells
- Each region has a perimeter of 8 edges
- Cost per region = 4 × 8 = 32
- Total cost for all regions = 32 × 4 = 128

## 🔧 Solution Approach
1. Parse the input grid
2. Use flood fill or DFS to identify connected regions
3. For each region:
   - Calculate its perimeter by counting exposed edges
   - Calculate its area by counting cells
   - Multiply area × perimeter
4. Sum all region costs

## ✨ The Answer
The total cost of fencing all garden regions is: **1361494**

## 🚀 Running the Code
1. Save your garden layout in `input_three.txt`
2. Run the program:
   ```bash
   # Python version
   python garden_fencing.py
   
   # Rust version
   cargo run
   ```

## 🛠️ Implementation Notes
- Uses depth-first search to find connected regions
- Efficiently calculates perimeter by checking adjacent cells
- Works with any size grid containing any letters
- Handles empty or invalid inputs gracefully

## 🏆 Performance
- Time complexity: O(rows × cols) for grid traversal
- Space complexity: O(rows × cols) for visited tracking
- Processes large gardens efficiently using iterative approaches
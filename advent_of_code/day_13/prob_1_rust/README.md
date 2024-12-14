# Button Path Puzzle Solver 🎮

## Description 📝
This program solves a puzzle about finding the optimal path using two buttons (A and B). Each button press moves in a specific direction and costs different amounts of tokens:
- Button A: Costs 3 tokens 🔴
- Button B: Costs 1 token 🔵

## Problem ❓
Given a set of coordinates and movement patterns, find the minimum number of tokens needed to reach all prizes. Each button can be pressed up to 100 times per prize.

## Solution 🎯
The total minimum tokens needed: **31589** 🎊

## How to Run 🚀
1. Make sure you have Rust installed 🦀
2. Clone this repository
3. Place your input in `input_final.txt`
4. Run:
```bash
cargo run
```

## Testing 🧪
Run the test suite with:
```bash
cargo test
```

## Input Format 📋
The input file should contain groups of 3 lines separated by blank lines:
```
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400
```

## Implementation Details 🔧
- Uses Rust's type system for safety
- Handles large numbers with i64
- Includes comprehensive test suite
- Memory efficient with references

## Performance ⚡
- Solves the puzzle using an optimized algorithm
- Handles multiple prize locations efficiently

## Author 👨‍💻
Created for Advent of Code challenge 🎄

## License 📄
Feel free to use and modify! 🆓
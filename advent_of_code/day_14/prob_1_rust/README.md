# 🤖 Robot Position Tracking 📍

## Problem Description
Track the final positions of robots moving in a 2D grid and calculate the product of robot counts in each quadrant.

## 🎯 Solution
The final answer is: **230436441** 🎉

## 🛠️ Implementation Details
The program:
1. Reads robot positions and velocities from input file 📄
2. Simulates 100 time steps for each robot ⏱️
3. Records final positions in a grid 📊
4. Divides the grid into quadrants and counts robots in each 🗺️
5. Calculates the product of these counts ✖️

## 🚀 How to Run
```bash
# Clone the repository
git clone [your-repo-url]

# Navigate to the project directory
cd robot-tracking

# Run with Cargo
cargo run
```

## 🧮 Input Format
Each line in `input.txt` should contain:
- Initial X position
- Initial Y position
- X velocity
- Y velocity

Example:
```
p=0,4 v=3,-3
p=6,3 v=-1,-3
```

## 🎨 Grid Visualization
```
Q1 | Q2
---|---
Q4 | Q3
```

## 🔧 Technical Details
- Written in Rust 🦀
- Uses HashMap for efficient position tracking
- Handles negative coordinates
- Implements modular arithmetic for grid wrapping

## ⚙️ Requirements
- Rust (latest stable version)
- Cargo package manager

## 📝 Notes
- Grid size: 101 x 103
- Runs 100 iterations per robot
- Handles edge cases with modulo operations

## 💡 Contributing
Feel free to submit issues and enhancement requests! 🤝

## ⭐ Credits
Developed for the robot tracking challenge. Keep those robots in line! 🤖✨
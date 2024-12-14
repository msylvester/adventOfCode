# 🤖 Robot Tree Pattern Finder 🌳

## 🎯 Solution
**Final Answer: 8270** 🎉
(Number of iterations until robots form a unique tree pattern)

## 🧩 Problem Description
Simulate robots moving in a 2D grid until they form a unique pattern (a "tree") where no two robots occupy the same position.

## 🔍 Key Features
- Grid visualization as bitmap image 🖼️
- Robot movement simulation ⚙️
- Collision detection system 💥
- Pattern recognition algorithm 🔎

## 🛠️ Implementation Details
The program:
1. Reads initial robot positions and velocities 📥
2. Simulates robot movement step by step 🔄
3. Checks for unique positioning after each step ✨
4. Generates a bitmap image of the final pattern 🎨
5. Reports the number of iterations taken 📊

## 🚀 How to Run
```bash
# Clone the repository
git clone [your-repo-url]

# Navigate to project directory
cd robot-pattern-finder

# Run with Cargo
cargo run
```

## 📊 Technical Specifications
- Grid size: 101 x 103
- Output: Black and white bitmap ("tree.bmp")
- Language: Rust 🦀
- Image processing: `image` crate

## 📝 Input Format
Each line in `input.txt` contains:
```
p=<x>,<y> v=<dx>,<dy>
```
Where:
- x, y: Initial position coordinates
- dx, dy: Velocity components

## 🎮 Controls
- No user interaction required
- Automatic simulation until pattern is found
- Results saved automatically

## ⚙️ Requirements
- Rust (latest stable version)
- Cargo package manager
- `image` crate (0.24 or later)

## 📋 Output
1. Creates "tree.bmp" showing final pattern
2. Prints iteration count (8270) to console

## 💡 Contributing
Contributions welcome! Feel free to:
- Report bugs 🐛
- Suggest improvements 🔧
- Submit pull requests 🤝

## ⭐ Performance
- Memory efficient grid representation
- Optimized collision detection
- Fast pattern recognition

May your robots find their way to the perfect tree! 🌟
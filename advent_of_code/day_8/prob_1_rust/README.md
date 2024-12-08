# 📡 Antenna Antinode Finder

This program solves a grid-based antenna problem by finding valid antinode positions for pairs of antennas with matching frequencies. 

## 🎯 Problem Description

Given a grid where:
- `.` represents empty space 🔲
- Any other character represents an antenna of that frequency (e.g., 'A', 'B', etc.) 📻

The program finds all possible "antinode" positions by:
1. 🔍 Identifying pairs of antennas with the same frequency
2. 📏 For each pair, calculating potential antinode positions by extending the line between them by the same distance
3. 🧮 Counting valid antinode positions that fall within the grid boundaries

An antinode position is valid if:
- ✨ It's calculated by extending the line between two antennas of the same frequency
- 📍 It falls within the grid boundaries
- 🔄 The same position can be an antinode for multiple antenna pairs

## 💡 Example

Input grid:
```
.A..
..A.
....
....
```

In this example:
- 🎯 Two 'A' antennas form a pair
- 📐 Their positions can generate antinodes by extending the line between them
- ✅ Only antinodes that fall within the 4x4 grid are counted

## 🚀 Usage

1. Create an input.txt file with your grid data
2. Run the program:
```bash
cargo run
```

## 🎉 Solution

For the full problem input, the program finds **396** valid antinode positions! 🎯

## 🛠️ Implementation Details

The solution is implemented in Rust and uses:
- 📦 HashMap to group antennas by frequency
- 🗃️ HashSet to track unique antinode positions
- 📖 Efficient file reading with BufReader
- ⚡ Error handling for file operations and grid validation

## ⚡ Performance

The algorithm runs with complexity O(n * k^2) where:
- n is the number of different frequencies 📊
- k is the maximum number of antennas of any frequency 📈

Memory usage is proportional to the number of antinodes found plus the input grid size. 💾

## 🤝 Contributing

Feel free to open issues or submit PRs if you have suggestions for improvements! 🌟
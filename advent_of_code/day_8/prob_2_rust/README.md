# 📡 Continuous Antenna Path Calculator

## 🎯 Problem Overview
This program solves an advanced antenna positioning problem where we calculate continuous paths of antinodes between matching antenna pairs. The final solution finds **1200** unique antinode positions! 🎉

## 📝 Problem Details

### 🔍 Input Format
- A grid represented as a text file where:
  - `.` represents empty space 🔲
  - Any other character represents an antenna of that frequency (e.g., 'A', 'B', etc.) 📻

### 🎮 How It Works
1. 🔎 Locates all antennas in the grid and groups them by frequency
2. 🔄 For each pair of matching frequency antennas:
   - Calculates the direction vector between them
   - Adds the second antenna position as an antinode
   - ➡️ Continues in that direction until hitting a grid boundary
   - 🔄 Repeats in the opposite direction (swapping start/end points)
3. 📊 Tracks all unique positions encountered
4. 🧮 Returns the total count of unique antinode positions

### 🌟 Key Differences from Basic Version
- ➡️ Creates continuous paths instead of single points
- 📍 Includes antenna positions as valid antinodes
- 🔄 Extends paths until grid boundaries
- 🎯 Results in significantly more antinode positions

## 🚀 Usage

1. Create `input_two.txt` with your grid data:
```
....A....
.....A...
.........
.........
```

2. Run the program:
```bash
cargo run
```

## 💻 Technical Implementation

### 🛠️ Key Components
- 📦 HashMap: Groups antennas by frequency
- 🗃️ HashSet: Stores unique antinode positions
- 📐 Vector math: Calculates direction and continuation paths
- ⚡ Efficient grid boundary checking

### 🎯 Algorithm Steps
1. 📥 Read and parse input grid
2. 🔍 Group antennas by frequency
3. 📊 For each frequency group:
   - Calculate paths between each antenna pair
   - Store all valid positions
4. 📤 Return total unique positions found

## ⚡ Performance
- 🚀 Time Complexity: O(n * k^2 * max(N,M))
  - n: number of frequencies
  - k: max antennas per frequency
  - N,M: grid dimensions
- 💾 Space Complexity: O(N*M) for storing antinodes

## 🎉 Results
For the full input grid, we find:
- 🎯 Total unique antinode positions: 1200
- ✨ Including continuous paths and antenna positions
- 🔍 All positions within grid boundaries

## 🤝 Contributing
Feel free to suggest improvements or report issues! We welcome:
- 🔧 Performance optimizations
- 📈 Additional test cases
- 🎨 Visualization improvements
- 📚 Documentation enhancements

## 🧪 Testing
Create your own test cases by:
1. 📝 Making a new grid input file
2. 🎯 Placing antennas with matching frequencies
3. 🔄 Running the program
4. ✅ Verifying the paths

Remember: The answer should be 1200 for the original problem input! 🎉

---
Happy Antenna Path Finding! 🚀 ✨ 📡
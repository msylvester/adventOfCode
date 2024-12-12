# 🎄 Advent of Code 2024 - Day 11: Blinking Stones 💎

## 🎯 Part 2 Solution
This repository contains a solution to Part 2 of Day 11's challenge, implementing a recursive stone transformation algorithm with memoization.

## 🧩 Problem Description
A sequence of stones blinks according to specific rules:
- If a stone's value is 0, it becomes 1
- If a stone has an even number of digits, it splits into two stones (first half and second half of digits)
- If a stone has an odd number of digits, it multiplies by 2024

## ✨ Solution Features
- Implemented in Rust 🦀
- Uses recursive approach with memoization
- Solution: **205,913,561,055,242**
- Cache Performance:
  - Number stones cache: 135,858 entries
  - Blinking stone cache: 3,972 entries

## 🚀 Implementation Details

### 💾 Data Structures
```rust
type Cache = HashMap<(i64, i32), i64>;
type StoneCache = HashMap<i64, StoneResult>;

enum StoneResult {
    Single(i64),
    Pair(i64, i64)
}
```

### 🔑 Key Functions
- `blinking_stone`: Applies transformation rules to a single stone
- `number_stones`: Recursively calculates the number of stones after multiple transformations
- Both functions use memoization to avoid redundant calculations

### 📥 Input
```rust
let puzzle_input = vec![20, 82084, 1650, 3, 346355, 363, 7975858, 0];
```

## 🏃‍♂️ Running the Solution

1. Save the code in `main.rs`
2. Run with cargo:
```bash
cargo run --release
```

## 📊 Performance
- Uses two separate caches to optimize different aspects of computation
- Achieves significant speedup through memoization
- Memory usage is optimized through strategic caching

## 🔍 Cache Analysis
- Number stones cache stores intermediate results for different depths
- Blinking stone cache prevents redundant stone transformations
- Total cached entries: 139,830

## 🛠️ Technical Notes
- Uses `i64` for number storage to handle large values
- Implements efficient string parsing for digit manipulation
- Leverages Rust's enum system for clean state management
- Employs smart pointer usage to minimize cloning

## 🎯 Optimization Strategies
1. Double-layer memoization
2. Efficient enum-based return values
3. Strategic use of string operations
4. Copy-on-write for cache values

## 🏆 Results
The solution successfully processes 75 blinks of the input stones, producing a final sum exceeding 205 trillion.
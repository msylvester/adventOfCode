# 🧮 Day 3: ASCII Memory Repair - Part 2 🛠️

## The Problem 🔍

After fixing part of the memory, you discover conditional statements that control which multiplications should be processed. Two new instructions have been found:

### New Instructions ⚡
- `do()` - *Enables* future multiplication instructions
- `don't()` - *Disables* future multiplication instructions

### Rules 📋
- Only the *most recent* `do()` or `don't()` instruction applies
- At program start, multiplications are *enabled*
- Disabled multiplications should be completely ignored

### Example 🌟
```
xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)do()?mul(8,5))
```

#### Step by Step 🚶
1. `mul(2,4)` is enabled at start = 8
2. `don't()` disables multiplications
3. `mul(5,5)` is disabled (ignored)
4. `mul(11,8)` is disabled (ignored)
5. `do()` re-enables multiplications
6. `mul(8,5)` is enabled = 40

Total sum = 48 (8 + 40)

## Task 📝
Calculate the sum of only the enabled multiplication results.

### Solution ✨
```
Total: 106780429
```

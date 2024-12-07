# Bridge Repair Calibration - Part 2 🌉✨

## Problem Description 📝
Building on Part 1, the engineers discovered a third operator being held by well-hidden elephants 🐘. This new operator changes how equations can be solved and significantly affects the total calibration result.

### Project Structure 📂
```
prob_2/
├── README.md
├── input.txt
└── src/
    └── main.rs
```

### New Operator 🔧
- Concatenation operator (`||`)
- Combines digits from left and right inputs into a single number
- Example: `12 || 345` becomes `12345`
- All operators (including concatenation) are still evaluated left-to-right

### Rules ⚖️
- Three available operators: add (`+`), multiply (`*`), and concatenate (`||`)
- Operators are evaluated left-to-right (no operator precedence)
- Numbers in equations cannot be rearranged
- Each position between numbers must have exactly one operator

## Example Solutions 💡
New valid equations using concatenation:
- `156: 15 6` → `15 || 6 = 156`
- `7290: 6 8 6 15` → `6 * 8 || 6 * 15`
- `192: 17 8 14` → `17 || 8 + 14`

## Implementation Details 🛠️
The solution extends Part 1 with:
1. New Operator enum including Concat
2. Modified evaluation function to handle concatenation
3. Updated combination generator to handle three operators
4. Enhanced testing for concatenation cases

## Correct Answer ⭐⭐
The solution for Part 2 is: 95297119227552

## Performance Metrics ⚡
Measured on debugging build:
- File reading: ~190µs
- Calculation: ~7.86s
- Total execution time: ~7.86s

## Running the Code 🚀
1. Place your input in `input.txt` in the project root
2. Run with `cargo run`
3. The program will output:
   - File path being used
   - Solution value
   - Detailed timing breakdown

## Testing 🧪
Additional test cases verify:
- Concatenation operator functionality
- Mixed operation sequences
- Complex expression evaluation

## Notes 📓
- The solution includes all valid equations from Part 1 plus new equations made possible by concatenation
- Base-3 counting is used to generate all possible operator combinations
- Performance metrics may vary based on hardware and build type (debug vs release)
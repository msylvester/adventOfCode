# Bridge Repair Calibration - Part 1 🌉

## Problem Description 📝
The engineers need help repairing a rope bridge by determining which test values could be produced by placing combinations of operators into their calibration equations. Each line of input represents a single equation, with the test value appearing before the colon.

### Rules ⚖️
- Only two operators are available: add (`+`) and multiply (`*`)
- Operators are evaluated left-to-right (no operator precedence)
- Numbers in equations cannot be rearranged
- Each position between numbers must have exactly one operator

## Example Input 📊
```
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
```

## Implementation Details 🛠️
The solution involves:
1. Parsing each input line into test values and numbers
2. Generating all possible combinations of operators
3. Evaluating each combination left-to-right
4. Checking if any combination produces the target test value
5. Summing up all valid test values

## Correct Answer ⭐
The solution for Part 1 is: 8401132154762

## Running the Code 🚀
1. Place your input in `input.txt` in the project root
2. Run with `cargo run`
3. The program will output the total calibration result

## Testing 🧪
The code includes unit tests to verify:
- Expression evaluation
- Operator combination generation
- Test case validation
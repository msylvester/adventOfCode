# 🔢 Number Transform Algorithm

This program implements a recursive number transformation algorithm that processes a list of integers through multiple iterations based on specific rules. The final output for the provided test case yields a solution length of 55,905 numbers.

## ⚙️ Algorithm Rules

For each number in the input list:
1. If the number is 0, it becomes 1
2. If the number has an even count of digits, it splits into two numbers (first half and second half)
3. If the number has an odd count of digits, it gets multiplied by 2024

All numerical operations are performed modulo 1000000007 to prevent overflow.

## 📋 Requirements

- Python 3.6+ 🐍
- Input file (`input.txt`) containing space-separated integers 📄

## 🚀 Usage

1. Create an `input.txt` file with space-separated integers
2. Run the program:
```bash
python number_transform.py
```

## 🔧 Technical Details

### 🎯 Functions

- `has_even_digit_count(number)`: Determines if a number has an even count of digits
- `split_number_in_half(number)`: Splits a number into two halves based on its digits
- `transform_number(number)`: Applies the transformation rules to a single number
- `process_numbers(numbers, iterations)`: Processes the entire list for a specified number of iterations

### 🔒 Constants

- `MODULUS = 1000000007`: Prime modulus to keep numbers manageable
- `MULTIPLIER = 2024`: Multiplication factor for numbers with odd digit counts
- `iterations = 75`: Number of transformation iterations

## ⚠️ Error Handling

The program includes error handling for:
- Missing input file
- Invalid number format in input file

## 📊 Output

The program outputs the length of the final transformed list. For the provided test case:
```
Solution length: 55905
```

## ⚡ Time and Space Complexity

- Time Complexity: O(n * 2^i) where n is the initial input size and i is the number of iterations
- Space Complexity: O(2^i) for the final output list

## 📝 Notes

- The algorithm is recursive and may require significant memory for large inputs or many iterations
- All numbers are kept within manageable bounds using modulo arithmetic
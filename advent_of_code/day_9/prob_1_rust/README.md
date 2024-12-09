# Advent of Code 2024: Day 9 part 1

A Rust program that processes blocks and spaces according to specific rules and calculates a checksum.

## Description

This program reads a sequence of numbers from an input file, where each number alternates between representing blocks and spaces. It then:
1. Creates an initial arrangement of blocks (represented by IDs) and free spaces
2. Moves all blocks to the first available free spaces
3. Calculates a checksum based on the final arrangement

## Expected Output

When run with the provided input, the program should produce the following checksum:
```
6291146824486
```

## Implementation Details

The program is implemented in Rust and uses a `Result` type for error handling, particularly for file operations. The main components are:

- `read_input`: Reads the input file
- `parse_input`: Converts the input string to a vector of integers
- `create_space_and_blocks`: Creates the initial arrangement
- `move_blocks`: Performs the block movement algorithm
- `calculate_checksum`: Computes the final checksum

### Error Handling

The program uses Rust's `Result` type for robust error handling, particularly for file operations. While it might seem simpler to use void return types, using `Result` allows us to:
- Properly propagate errors up the call chain
- Use the `?` operator for concise error handling
- Follow Rust's philosophy of explicit error handling
- Provide meaningful error messages to the calling environment

## Usage

1. Create an input file named `input_two.txt` with your sequence
2. Run the program:
```bash
cargo run
```

## Building

```bash
cargo build --release
```

## Requirements

- Rust (latest stable version recommended)
- Input file (`input_two.txt`) in the same directory as the executable
"""
A module for processing numbers through a series of transformations based on specific rules.
The main function applies these transformations recursively for a given number of iterations.
"""

from typing import List, Tuple
import sys

# Constants
MODULUS = 1000000007  # Prime modulus to keep numbers manageable
MULTIPLIER = 2024     # Multiplication factor for odd-digit numbers

def has_even_digit_count(number: int) -> bool:
    """
    Check if a number has an even count of digits.
    
    Args:
        number (int): The number to check
        
    Returns:
        bool: True if the number has an even count of digits, False otherwise
    """
    return len(str(abs(number))) % 2 == 0

def split_number_in_half(number: int) -> Tuple[int, int]:
    """
    Split a number into two halves based on its digits.
    
    Args:
        number (int): The number to split
        
    Returns:
        Tuple[int, int]: The first and second half of the number as integers
    """
    num_str = str(number)
    mid_point = len(num_str) // 2
    return int(num_str[:mid_point]), int(num_str[mid_point:])

def transform_number(number: int) -> List[int]:
    """
    Transform a number based on specific rules:
    - If number is 0, return [1]
    - If number has even digits, split it in half
    - Otherwise, multiply by MULTIPLIER
    
    Args:
        number (int): The number to transform
        
    Returns:
        List[int]: List containing the transformed number(s)
    """
    number = number % MODULUS
    
    if number == 0:
        return [1]
    elif has_even_digit_count(number):
        first, second = split_number_in_half(number)
        return [first, second]
    else:
        return [(number * MULTIPLIER) % MODULUS]

def process_numbers(numbers: List[int], iterations: int) -> List[int]:
    """
    Process a list of numbers through multiple iterations of transformations.
    
    Args:
        numbers (List[int]): Initial list of numbers
        iterations (int): Number of iterations to process
        
    Returns:
        List[int]: Transformed list of numbers after all iterations
    """
    if iterations == 0:
        return numbers
    
    new_numbers = []
    for num in numbers:
        new_numbers.extend(transform_number(num))
    
    return process_numbers(new_numbers, iterations - 1)

def main():
    """Main function to read input and process numbers."""
    try:
        with open('input.txt', 'r') as file:
            numbers = [int(x) for x in file.read().split()]
        
        result = process_numbers(numbers, 25)
        print(f'Solution length: {len(result)}')
        
    except FileNotFoundError:
        print("Error: input.txt file not found")
        sys.exit(1)
    except ValueError:
        print("Error: Invalid number format in input file")
        sys.exit(1)

if __name__ == "__main__":
    main()
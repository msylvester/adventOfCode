// XMAS is found when:

// Four consecutive positions spell "XMAS" in any of 8 directions
// Can go horizontally, vertically, or diagonally
// Can read forwards or backwards ("SAMX" counts)
// Must stay within grid bounds
// Can overlap with other instances of XMAS
// X(5,1)M(6,1)A(7,1)S(8,1)
// X(6,1)M(7,1)A(8,1)S(9,1)
// S(2,2)A(3,2)M(4,2)X(5,2)
// S(6,2)A(7,2)M(8,2)X(9,2)
// X(1,5)M(2,5)A(3,5)S(4,5)
// X(5,5)M(6,5)A(7,5)S(8,5)
// X(7,5)M(8,5)A(9,5)S(10,5)
// X(1,6)M(2,6)A(3,6)S(4,6)
// X(7,6)M(8,6)A(9,6)S(10,6)
// X(5,1)M(4,2)A(3,3)S(2,4)
// X(6,1)M(5,2)A(4,3)S(3,4)
// X(7,1)M(6,2)A(5,3)S(4,4)
// X(8,1)M(7,2)A(6,3)S(5,4)
// X(9,1)M(8,2)A(7,3)S(6,4)
// X(2,10)M(3,9)A(4,8)S(5,7)
// X(4,10)M(5,9)A(6,8)S(7,7)
// X(6,10)M(7,9)A(8,8)S(9,7)
// X(8,10)M(9,9)A(10,8)S(1,7)
// For example, in this snippet:

fn main() {
    println!("Hello, world!");
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_grid() {
        let grid: Vec<String> = vec![];
        assert_eq!(count_xmas(&grid), 0);
    }

    #[test]
    fn test_single_horizontal() {
        let grid = vec![
            "XMAS".to_string(),
        ];
        assert_eq!(count_xmas(&grid), 1);
    }

    #[test]
    fn test_single_vertical() {
        let grid = vec![
            "X".to_string(),
            "M".to_string(),
            "A".to_string(),
            "S".to_string(),
        ];
        assert_eq!(count_xmas(&grid), 1);
    }

    #[test]
    fn test_diagonal() {
        let grid = vec![
            "X...".to_string(),
            ".M..".to_string(),
            "..A.".to_string(),
            "...S".to_string(),
        ];
        assert_eq!(count_xmas(&grid), 1);
    }

    #[test]
    fn test_reverse_horizontal() {
        let grid = vec![
            "SAMX".to_string(),
        ];
        assert_eq!(count_xmas(&grid), 1);
    }

    #[test]
    fn test_reverse_vertical() {
        let grid = vec![
            "S".to_string(),
            "A".to_string(),
            "M".to_string(),
            "X".to_string(),
        ];
        assert_eq!(count_xmas(&grid), 1);
    }

    #[test]
    fn test_reverse_diagonal() {
        let grid = vec![
            "...S".to_string(),
            "..A.".to_string(),
            ".M..".to_string(),
            "X...".to_string(),
        ];
        assert_eq!(count_xmas(&grid), 1);
    }

    #[test]
    fn test_multiple_overlapping() {
        let grid = vec![
            "MMMSXXMASM".to_string(),
            "MSAMXMSMSA".to_string(),
            "AMXSXMAAMM".to_string(),
            "MSAMASMSMX".to_string(),
            "XMASAMXAMM".to_string(),
            "XXAMMXXAMA".to_string(),
            "SMSMSASXSS".to_string(),
            "SAXAMASAAA".to_string(),
            "MAMMMXMMMM".to_string(),
            "MXMXAXMASX".to_string(),
        ];
        assert_eq!(count_xmas(&grid), 18);
    }

    #[test]
    fn test_no_xmas() {
        let grid = vec![
            "ABCD".to_string(),
            "EFGH".to_string(),
            "IJKL".to_string(),
        ];
        assert_eq!(count_xmas(&grid), 0);
    }
}
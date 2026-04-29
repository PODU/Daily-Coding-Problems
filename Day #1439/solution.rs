// Day 1439: Find a word in a char grid reading left-to-right or top-to-bottom.
// Approach: build each row and column string, check if target is a substring.
// Time: O(R*C*L) substring scan, Space: O(R+C).

fn find_word(grid: &[Vec<char>], target: &str) -> bool {
    let rows = grid.len();
    if rows == 0 {
        return false;
    }
    let cols = grid[0].len();
    for row in grid {
        let s: String = row.iter().collect();
        if s.contains(target) {
            return true;
        }
    }
    for c in 0..cols {
        let col: String = (0..rows).map(|r| grid[r][c]).collect();
        if col.contains(target) {
            return true;
        }
    }
    false
}

fn main() {
    let grid: Vec<Vec<char>> = vec![
        "FACI".chars().collect(),
        "OBQP".chars().collect(),
        "ANOB".chars().collect(),
        "MASS".chars().collect(),
    ];
    println!("{}", find_word(&grid, "FOAM")); // true
    println!("{}", find_word(&grid, "MASS")); // true
}

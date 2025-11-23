// Day 645: Find a word in a grid going left-to-right or top-to-bottom.
// Approach: scan every row and every column for the target as a substring start.
// Time: O(R*C*L), Space: O(1).
fn find_word(grid: &[Vec<char>], word: &str) -> bool {
    let w: Vec<char> = word.chars().collect();
    let (r_n, c_n, l) = (grid.len(), grid[0].len(), w.len());
    // horizontal
    for r in 0..r_n {
        for c in 0..=c_n.saturating_sub(l) {
            if (0..l).all(|k| grid[r][c + k] == w[k]) {
                return true;
            }
        }
    }
    // vertical
    for c in 0..c_n {
        for r in 0..=r_n.saturating_sub(l) {
            if (0..l).all(|k| grid[r + k][c] == w[k]) {
                return true;
            }
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

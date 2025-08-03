// Search word in matrix rows (L->R) and columns (top->bottom) via substring check.
// Time O(N*M*L), Space O(max(N,M)).
fn find_word(grid: &[Vec<char>], word: &str) -> bool {
    let n = grid.len();
    let m = grid[0].len();
    for r in 0..n {
        let row: String = grid[r].iter().collect();
        if row.contains(word) {
            return true;
        }
    }
    for c in 0..m {
        let col: String = (0..n).map(|r| grid[r][c]).collect();
        if col.contains(word) {
            return true;
        }
    }
    false
}

fn main() {
    let grid: Vec<Vec<char>> = vec![
        vec!['F', 'A', 'C', 'I'],
        vec!['O', 'B', 'Q', 'P'],
        vec!['A', 'N', 'O', 'B'],
        vec!['M', 'A', 'S', 'S'],
    ];
    println!("'FOAM' -> {}", find_word(&grid, "FOAM"));
    println!("'MASS' -> {}", find_word(&grid, "MASS"));
}

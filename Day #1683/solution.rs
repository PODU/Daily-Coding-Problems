// Day 1683: Find word reading left-to-right (a row) or top-to-bottom (a column).
// Build row/column strings, substring search. Time O(N*M), Space O(N+M).
fn find_word(grid: &[&str], word: &str) -> bool {
    let rows: Vec<&[u8]> = grid.iter().map(|s| s.as_bytes()).collect();
    let (nr, nc) = (rows.len(), rows[0].len());
    for r in 0..nr {
        if grid[r].contains(word) {
            return true;
        }
    }
    for c in 0..nc {
        let col: String = (0..nr).map(|r| rows[r][c] as char).collect();
        if col.contains(word) {
            return true;
        }
    }
    false
}

fn main() {
    let grid = ["FACI", "OBQP", "ANOB", "MASS"];
    println!("'FOAM' -> {}", find_word(&grid, "FOAM")); // true
    println!("'MASS' -> {}", find_word(&grid, "MASS")); // true
}

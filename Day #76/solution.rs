// Day 76: Minimum columns to remove so every column is sorted top-to-bottom.
// Greedy scan: count columns that are not non-decreasing. Time O(N*M), Space O(1).
fn min_columns_to_remove(grid: &[&str]) -> usize {
    if grid.is_empty() {
        return 0;
    }
    let rows: Vec<&[u8]> = grid.iter().map(|s| s.as_bytes()).collect();
    let cols = rows[0].len();
    let mut remove = 0;
    for c in 0..cols {
        for r in 1..rows.len() {
            if rows[r][c] < rows[r - 1][c] {
                remove += 1;
                break;
            }
        }
    }
    remove
}

fn main() {
    let grid = ["cba", "daf", "ghi"];
    println!("{}", min_columns_to_remove(&grid)); // 1
}

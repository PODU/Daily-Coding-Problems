// Day 1216: Min columns to delete so each column is non-decreasing top->bottom.
// Approach: scan each column once, count unsorted columns. Time O(N*M), Space O(1).
fn min_deletions(grid: &[&str]) -> usize {
    if grid.is_empty() {
        return 0;
    }
    let rows: Vec<&[u8]> = grid.iter().map(|s| s.as_bytes()).collect();
    let cols = rows[0].len();
    let mut count = 0;
    for c in 0..cols {
        for r in 1..rows.len() {
            if rows[r][c] < rows[r - 1][c] {
                count += 1;
                break;
            }
        }
    }
    count
}

fn main() {
    let grid = ["cba", "daf", "ghi"];
    println!("{}", min_deletions(&grid)); // 1
}

// Count columns to delete so each remaining column is non-decreasing top->bottom.
// Scan each column for any adjacent out-of-order pair. Time O(N*M), Space O(1).

fn min_deletions(grid: &[&str]) -> usize {
    if grid.is_empty() {
        return 0;
    }
    let rows: Vec<&[u8]> = grid.iter().map(|s| s.as_bytes()).collect();
    let m = rows[0].len();
    let mut count = 0;
    for c in 0..m {
        for i in 0..rows.len() - 1 {
            if rows[i][c] > rows[i + 1][c] {
                count += 1;
                break;
            }
        }
    }
    count
}

fn main() {
    let grid = ["cba", "daf", "ghi"];
    println!("{}", min_deletions(&grid));
}

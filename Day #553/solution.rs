// Count columns that are NOT non-decreasing top-to-bottom; that's the min to remove.
// O(N*M) time, O(1) extra space.

fn min_deletions(grid: &[&str]) -> i32 {
    if grid.is_empty() { return 0; }
    let rows: Vec<&[u8]> = grid.iter().map(|s| s.as_bytes()).collect();
    let cols = rows[0].len();
    let mut count = 0;
    for c in 0..cols {
        for r in 1..rows.len() {
            if rows[r][c] < rows[r - 1][c] { count += 1; break; }
        }
    }
    count
}

fn main() {
    println!("{}", min_deletions(&["cba", "daf", "ghi"])); // 1
    println!("{}", min_deletions(&["abcdef"]));            // 0
    println!("{}", min_deletions(&["zyx", "wvu", "tsr"])); // 3
}

// Day 1671: Min columns to remove so each column is non-decreasing top->bottom.
// Count columns containing any out-of-order adjacent pair. Time O(N*M), Space O(1).
fn min_deletions(g: &[&str]) -> usize {
    if g.is_empty() {
        return 0;
    }
    let rows: Vec<&[u8]> = g.iter().map(|s| s.as_bytes()).collect();
    let cols = rows[0].len();
    let mut del = 0;
    for j in 0..cols {
        for i in 0..rows.len() - 1 {
            if rows[i][j] > rows[i + 1][j] {
                del += 1;
                break;
            }
        }
    }
    del
}

fn main() {
    println!("{}", min_deletions(&["cba", "daf", "ghi"])); // 1
}

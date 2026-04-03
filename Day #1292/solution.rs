// Day 1292: kth (0-indexed) row of Pascal's triangle.
// Update row in place from right to left. O(k^2) time, O(k) space.
fn pascal_row(k: usize) -> Vec<u64> {
    let mut row = vec![1u64; k + 1];
    for i in 1..=k {
        for j in (1..i).rev() {
            row[j] += row[j - 1];
        }
    }
    row
}

fn main() {
    let r = pascal_row(4); // 1 4 6 4 1
    let parts: Vec<String> = r.iter().map(|v| v.to_string()).collect();
    println!("{}", parts.join(" "));
}

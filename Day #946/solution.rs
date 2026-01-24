// Day 946: kth row of Pascal's triangle (1-indexed) using O(k) space.
// In-place update of a single row, right-to-left. Time O(k^2), Space O(k).

fn pascal_row(k: usize) -> Vec<u64> {
    let mut row = vec![1u64];
    for _ in 1..k {
        row.push(0);
        for j in (1..row.len()).rev() {
            row[j] += row[j - 1];
        }
    }
    row
}

fn main() {
    let k = 5; // README example -> 5th row
    let r = pascal_row(k);
    let s: Vec<String> = r.iter().map(|v| v.to_string()).collect();
    println!("{}", s.join(" "));
}

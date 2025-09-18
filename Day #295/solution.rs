// Kth row of Pascal's triangle (1-indexed) via iterative binomials in one array. O(k) space, O(k) time.
fn pascal_row(k: usize) -> Vec<u64> {
    let n = (k - 1) as u64; // 0-indexed row number
    let mut row = vec![1u64; k];
    for r in 1..k {
        let rr = r as u64;
        row[r] = row[r - 1] * (n - rr + 1) / rr;
    }
    row
}

fn main() {
    let row = pascal_row(5);
    let parts: Vec<String> = row.iter().map(|x| x.to_string()).collect();
    println!("[{}]", parts.join(", "));
}

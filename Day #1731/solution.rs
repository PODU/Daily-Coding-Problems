// Day 1731: kth row of Pascal's triangle (1-indexed) using O(k) space.
// Binomial coefficients built in place. Time O(k), Space O(k).
fn pascal_row(k: usize) -> Vec<u64> {
    let mut row = vec![1u64; k];
    for i in 1..k {
        row[i] = row[i - 1] * (k as u64 - i as u64) / i as u64;
    }
    row
}

fn main() {
    let k = 5; // row [1,4,6,4,1]
    let r = pascal_row(k);
    let s: Vec<String> = r.iter().map(|v| v.to_string()).collect();
    println!("{}", s.join(" "));
}

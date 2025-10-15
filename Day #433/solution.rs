// Day 433: Next larger integer with the same number of set bits (Gosper's hack).
// c = n & -n; r = n + c; next = (((r ^ n) >> 2) / c) | r. O(1) time, O(1) space.
fn next_same_bits(n: i64) -> i64 {
    if n <= 0 {
        return n;
    }
    let c = n & (-n);
    let r = n + c;
    (((r ^ n) >> 2) / c) | r
}

fn main() {
    let n: i64 = 6;
    let m = next_same_bits(n);
    println!("Input: {} ({:b} in binary)", n, n);
    println!("Next: {} ({:b} in binary)", m, m);
}

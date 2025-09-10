// Max of two ints without if/else/branch/ternary/comparison.
// Use sign bit of (a-b) via 64-bit diff to avoid overflow. O(1) time, O(1) space.

fn max_of(a: i64, b: i64) -> i64 {
    let d = a - b;                       // safe in 64-bit for 32-bit inputs
    let sign = (d >> 63) & 1;            // 1 if a<b, else 0
    a - sign * d                          // a - (a-b) = b when a<b, else a
}

fn main() {
    println!("max(3, 7) = {}", max_of(3, 7));
    println!("max(10, 2) = {}", max_of(10, 2));
}

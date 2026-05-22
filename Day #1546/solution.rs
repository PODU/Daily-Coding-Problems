// Day 1546: Stable partition of a linked list around pivot k.
// Collect into two ordered vectors (< k) and (>= k), concatenate. Time O(n), Space O(n).
// (Rust ownership makes index-free in-place splicing awkward; the order-preserving
//  two-bucket approach gives the identical result.)

fn partition(values: &[i32], k: i32) -> Vec<i32> {
    let mut less = Vec::new();
    let mut ge = Vec::new();
    for &v in values {
        if v < k {
            less.push(v);
        } else {
            ge.push(v);
        }
    }
    less.extend(ge);
    less
}

fn main() {
    let result = partition(&[5, 1, 8, 0, 3], 3);
    let parts: Vec<String> = result.iter().map(|v| v.to_string()).collect();
    println!("{}", parts.join(" -> "));
}

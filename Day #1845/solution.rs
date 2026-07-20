// Day 1845: Index of nearest larger value (by array distance) via outward expansion.
// Time O(N) per query, Space O(1). Returns None if none exists.

fn nearest_larger(a: &[i32], i: usize) -> Option<usize> {
    let n = a.len();
    for d in 1..n {
        if i >= d && a[i - d] > a[i] {
            return Some(i - d);
        }
        if i + d < n && a[i + d] > a[i] {
            return Some(i + d);
        }
    }
    None
}

fn main() {
    println!("{:?}", nearest_larger(&[4, 1, 3, 5, 6], 0)); // Some(3)
}

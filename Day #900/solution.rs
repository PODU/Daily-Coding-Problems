// Kadane's max subarray sum; empty subarray (0) allowed so answer is never negative. O(N) time, O(1) space.
fn max_subarray_sum(a: &[i64]) -> i64 {
    let mut best = 0;
    let mut cur = 0;
    for &x in a {
        cur = (cur + x).max(0);
        best = best.max(cur);
    }
    best
}

fn main() {
    println!("{}", max_subarray_sum(&[34, -50, 42, 14, -5, 86])); // 137
    println!("{}", max_subarray_sum(&[-5, -1, -8, -9]));          // 0
}

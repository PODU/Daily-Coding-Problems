// Maximum contiguous subarray sum (Kadane), empty subarray allowed (min 0).
// Time O(n), Space O(1).
fn max_subarray(a: &[i64]) -> i64 {
    let mut best = 0i64;
    let mut cur = 0i64;
    for &x in a {
        cur = (cur + x).max(0);
        best = best.max(cur);
    }
    best
}

fn main() {
    println!("{}", max_subarray(&[34, -50, 42, 14, -5, 86]));
}

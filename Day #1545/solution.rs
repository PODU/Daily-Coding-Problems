// Pyramid min cost (only lowering). For each index the max pyramid height is
// min of a left pass (rise +1 from edge) and a right pass. A pyramid of peak h
// retains h*h stones, so cost = sum(a) - max(h)^2. Time O(n), Space O(n).
fn min_cost(a: &[i64]) -> i64 {
    let n = a.len();
    let mut left = vec![0i64; n];
    let mut right = vec![0i64; n];
    left[0] = a[0].min(1);
    for i in 1..n {
        left[i] = a[i].min(left[i - 1] + 1);
    }
    right[n - 1] = a[n - 1].min(1);
    for i in (0..n - 1).rev() {
        right[i] = a[i].min(right[i + 1] + 1);
    }
    let mut best_peak = 0i64;
    let mut sum = 0i64;
    for i in 0..n {
        best_peak = best_peak.max(left[i].min(right[i]));
        sum += a[i];
    }
    sum - best_peak * best_peak
}

fn main() {
    println!("{}", min_cost(&[1, 1, 3, 3, 2, 1]));
}

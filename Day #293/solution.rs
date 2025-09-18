// Pyramid reshape (lowering only): L[i]/R[i] cap ramp slopes, peak v=max min(L,R), cost=sum-v*v.
// Time O(n), Space O(n).
fn min_cost(h: &[i64]) -> i64 {
    let n = h.len();
    let mut l = vec![0i64; n];
    let mut r = vec![0i64; n];
    l[0] = h[0].min(1);
    for i in 1..n {
        l[i] = h[i].min(l[i - 1] + 1);
    }
    r[n - 1] = h[n - 1].min(1);
    for i in (0..n - 1).rev() {
        r[i] = h[i].min(r[i + 1] + 1);
    }
    let mut v = 0i64;
    let mut sum = 0i64;
    for i in 0..n {
        v = v.max(l[i].min(r[i]));
        sum += h[i];
    }
    sum - v * v
}

fn main() {
    let h = vec![1i64, 1, 3, 3, 2, 1];
    println!("{}", min_cost(&h));
}

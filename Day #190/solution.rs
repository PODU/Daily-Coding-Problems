// Day 190: Maximum circular subarray sum, empty subarray (sum 0) allowed.
// ans = max(0, maxKadane, total - minKadane). Time O(n), Space O(1).
fn max_circular_sum(a: &[i64]) -> i64 {
    let mut total = 0i64;
    let mut max_k = i64::MIN;
    let mut cur_max = 0i64;
    let mut min_k = i64::MAX;
    let mut cur_min = 0i64;
    for &x in a {
        total += x;
        cur_max = x.max(cur_max + x);
        max_k = max_k.max(cur_max);
        cur_min = x.min(cur_min + x);
        min_k = min_k.min(cur_min);
    }
    0.max(max_k.max(total - min_k))
}

fn main() {
    println!("{}", max_circular_sum(&[8, -1, 3, 4]));
    println!("{}", max_circular_sum(&[-4, 5, 1, 0]));
}

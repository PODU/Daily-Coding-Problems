// Day 852: maximum circular subarray sum (empty allowed -> 0).
// answer = max(maxKadane clamped at 0, total - minKadane). O(n) time, O(1) space.
fn max_circular(a: &[i64]) -> i64 {
    let mut total = 0i64;
    let mut max_k = i64::MIN;
    let mut min_k = i64::MAX;
    let mut cur_max = 0i64;
    let mut cur_min = 0i64;
    for &x in a {
        total += x;
        cur_max = x.max(cur_max + x);
        max_k = max_k.max(cur_max);
        cur_min = x.min(cur_min + x);
        min_k = min_k.min(cur_min);
    }
    let non_wrap = max_k.max(0);
    let wrap = total - min_k;
    non_wrap.max(wrap)
}

fn main() {
    println!("{}", max_circular(&[8, -1, 3, 4])); // 15
    println!("{}", max_circular(&[-4, 5, 1, 0])); // 6
}

// Day 494: Maximum circular subarray sum (empty allowed => 0).
// max(maxKadane, total - minKadane); if all negative answer is 0. Time O(n), Space O(1).
fn max_circular_subarray(a: &[i64]) -> i64 {
    let mut total: i64 = 0;
    let mut max_k = i64::MIN;
    let mut cur_max: i64 = 0;
    let mut min_k = i64::MAX;
    let mut cur_min: i64 = 0;
    for &x in a {
        total += x;
        cur_max = x.max(cur_max + x);
        max_k = max_k.max(cur_max);
        cur_min = x.min(cur_min + x);
        min_k = min_k.min(cur_min);
    }
    if max_k < 0 {
        return 0; // all negative -> empty subarray
    }
    max_k.max(total - min_k)
}

fn main() {
    println!("{}", max_circular_subarray(&[8, -1, 3, 4])); // 15
    println!("{}", max_circular_subarray(&[-4, 5, 1, 0])); // 6
}

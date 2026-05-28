// Day 1579: Maximum circular subarray sum (empty allowed -> 0).
// ans = max(Kadane-with-empty, total - minSubarray). Time: O(n); Space: O(1).

fn max_circular(a: &[i64]) -> i64 {
    let (mut total, mut max_end, mut max_sum) = (0i64, 0i64, 0i64);
    let mut min_end = 0i64;
    let mut min_sum = i64::MAX;
    for &x in a {
        total += x;
        max_end = x.max(max_end + x);
        max_sum = max_sum.max(max_end);
        min_end = x.min(min_end + x);
        min_sum = min_sum.min(min_end);
    }
    max_sum.max(total - min_sum)
}

fn main() {
    println!("{}", max_circular(&[8, -1, 3, 4])); // 15
    println!("{}", max_circular(&[-4, 5, 1, 0])); // 6
}

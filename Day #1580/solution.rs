// Day 1580: Largest divisible subset (every pair mutually divisible).
// Sort, then LIS-style DP: dp[i] = longest chain ending at i where a[i] % a[j] == 0.
// Time: O(n^2); Space: O(n).

fn largest_divisible(a: &[i64]) -> Vec<i64> {
    let mut a = a.to_vec();
    a.sort();
    let n = a.len();
    if n == 0 {
        return vec![];
    }
    let mut dp = vec![1usize; n];
    let mut prev = vec![-1i64; n];
    let mut best = 0usize;
    for i in 0..n {
        for j in 0..i {
            if a[i] % a[j] == 0 && dp[j] + 1 > dp[i] {
                dp[i] = dp[j] + 1;
                prev[i] = j as i64;
            }
        }
        if dp[i] > dp[best] {
            best = i;
        }
    }
    let mut res = Vec::new();
    let mut i = best as i64;
    while i != -1 {
        res.push(a[i as usize]);
        i = prev[i as usize];
    }
    res.reverse();
    res
}

fn main() {
    println!("{:?}", largest_divisible(&[3, 5, 10, 20, 21])); // [5, 10, 20]
}

// Day 198: Largest divisible subset.
// Sort, then DP: dp[i] = longest chain ending at i (a[j] | a[i]); track parent for reconstruction.
// Time: O(n^2), Space: O(n).
fn largest_divisible_subset(arr: &[i64]) -> Vec<i64> {
    let mut a = arr.to_vec();
    a.sort();
    let n = a.len();
    if n == 0 {
        return vec![];
    }
    let mut dp = vec![1usize; n];
    let mut parent = vec![-1i64; n];
    let mut best = 0;
    for i in 0..n {
        for j in 0..i {
            if a[i] % a[j] == 0 && dp[j] + 1 > dp[i] {
                dp[i] = dp[j] + 1;
                parent[i] = j as i64;
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
        i = parent[i as usize];
    }
    res.reverse();
    res
}

fn main() {
    println!("{:?}", largest_divisible_subset(&[3, 5, 10, 20, 21])); // [5, 10, 20]
    println!("{:?}", largest_divisible_subset(&[1, 3, 6, 24]));      // [1, 3, 6, 24]
}

// Day 464: Largest divisible subset.
// Approach: sort, then LIS-style DP where dp[i]=longest chain ending at i;
// j divides i means a[i]%a[j]==0. Reconstruct via parent pointers.
// Time: O(n^2), Space: O(n).
fn largest_divisible_subset(mut nums: Vec<i64>) -> Vec<i64> {
    if nums.is_empty() {
        return vec![];
    }
    nums.sort();
    let n = nums.len();
    let mut dp = vec![1usize; n];
    let mut parent = vec![-1i64; n];
    let mut best = 0usize;
    for i in 0..n {
        for j in 0..i {
            if nums[i] % nums[j] == 0 && dp[j] + 1 > dp[i] {
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
    while i >= 0 {
        res.push(nums[i as usize]);
        i = parent[i as usize];
    }
    res.reverse();
    res
}

fn format(v: &[i64]) -> String {
    let parts: Vec<String> = v.iter().map(|x| x.to_string()).collect();
    format!("[{}]", parts.join(", "))
}

fn main() {
    println!("{}", format(&largest_divisible_subset(vec![3, 5, 10, 20, 21])));
    println!("{}", format(&largest_divisible_subset(vec![1, 3, 6, 24])));
}

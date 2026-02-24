// Day 1123 - Largest divisible subset
// Sort, LIS-style DP where j extends i if a[i] % a[j] == 0; reconstruct via
// parent pointers. Time: O(n^2), Space: O(n).

fn largest_divisible_subset(nums: &[i32]) -> Vec<i32> {
    if nums.is_empty() {
        return vec![];
    }
    let mut a = nums.to_vec();
    a.sort();
    let n = a.len();
    let mut dp = vec![1usize; n];
    let mut parent = vec![-1i32; n];
    let mut best = 0;
    for i in 0..n {
        for j in 0..i {
            if a[i] % a[j] == 0 && dp[j] + 1 > dp[i] {
                dp[i] = dp[j] + 1;
                parent[i] = j as i32;
            }
        }
        if dp[i] > dp[best] {
            best = i;
        }
    }
    let mut res = Vec::new();
    let mut k = best as i32;
    while k != -1 {
        res.push(a[k as usize]);
        k = parent[k as usize];
    }
    res.reverse();
    res
}

fn main() {
    println!("{:?}", largest_divisible_subset(&[3, 5, 10, 20, 21])); // [5, 10, 20]
    println!("{:?}", largest_divisible_subset(&[1, 3, 6, 24]));      // [1, 3, 6, 24]
}

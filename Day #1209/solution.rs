// Day 1209: Largest divisible subset.
// Sort, dp[i]=longest chain ending at i with parent pointers. Time O(n^2), Space O(n).
fn largest_divisible_subset(mut a: Vec<i64>) -> Vec<i64> {
    a.sort();
    let n = a.len();
    if n == 0 {
        return vec![];
    }
    let mut dp = vec![1usize; n];
    let mut par = vec![usize::MAX; n];
    let mut best = 0;
    for i in 0..n {
        for j in 0..i {
            if a[i] % a[j] == 0 && dp[j] + 1 > dp[i] {
                dp[i] = dp[j] + 1;
                par[i] = j;
            }
        }
        if dp[i] > dp[best] {
            best = i;
        }
    }
    let mut res = Vec::new();
    let mut i = best;
    loop {
        res.push(a[i]);
        if par[i] == usize::MAX {
            break;
        }
        i = par[i];
    }
    res.reverse();
    res
}

fn main() {
    println!("{:?}", largest_divisible_subset(vec![3, 5, 10, 20, 21])); // [5, 10, 20]
}

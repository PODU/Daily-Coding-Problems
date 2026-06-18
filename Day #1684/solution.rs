// Day 1684: Levenshtein edit distance via 1D rolling DP.
// Time O(n*m), Space O(min(n,m)).
fn edit_distance(a: &str, b: &str) -> usize {
    let a: Vec<char> = a.chars().collect();
    let b: Vec<char> = b.chars().collect();
    let (n, m) = (a.len(), b.len());
    let mut dp: Vec<usize> = (0..=m).collect();
    for i in 1..=n {
        let mut prev = dp[0];
        dp[0] = i;
        for j in 1..=m {
            let tmp = dp[j];
            dp[j] = if a[i - 1] == b[j - 1] {
                prev
            } else {
                1 + prev.min(dp[j]).min(dp[j - 1])
            };
            prev = tmp;
        }
    }
    dp[m]
}

fn main() {
    println!("{}", edit_distance("kitten", "sitting")); // 3
}

// Day 391: Longest common contiguous subsequence (substring) of two histories.
// DP on suffix-run lengths. Time O(n*m), Space O(n*m).
fn longest_common(a: &[&str], b: &[&str]) -> Vec<String> {
    let (n, m) = (a.len(), b.len());
    let mut dp = vec![vec![0usize; m + 1]; n + 1];
    let (mut best, mut end_i) = (0usize, 0usize);
    for i in 1..=n {
        for j in 1..=m {
            if a[i - 1] == b[j - 1] {
                dp[i][j] = dp[i - 1][j - 1] + 1;
                if dp[i][j] > best {
                    best = dp[i][j];
                    end_i = i;
                }
            }
        }
    }
    a[end_i - best..end_i].iter().map(|s| s.to_string()).collect()
}

fn main() {
    let user1 = ["/home", "/register", "/login", "/user", "/one", "/two"];
    let user2 = ["/home", "/red", "/login", "/user", "/one", "/pink"];
    println!("{:?}", longest_common(&user1, &user2));
}

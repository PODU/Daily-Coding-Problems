// Day 1848: Longest common subsequence of three strings via 3D dynamic programming.
// Time O(L1*L2*L3), Space O(L1*L2*L3).

fn lcs3(a: &str, b: &str, c: &str) -> usize {
    let a: Vec<u8> = a.bytes().collect();
    let b: Vec<u8> = b.bytes().collect();
    let c: Vec<u8> = c.bytes().collect();
    let (la, lb, lc) = (a.len(), b.len(), c.len());
    let mut dp = vec![vec![vec![0usize; lc + 1]; lb + 1]; la + 1];
    for i in 1..=la {
        for j in 1..=lb {
            for k in 1..=lc {
                if a[i - 1] == b[j - 1] && b[j - 1] == c[k - 1] {
                    dp[i][j][k] = dp[i - 1][j - 1][k - 1] + 1;
                } else {
                    dp[i][j][k] = dp[i - 1][j][k].max(dp[i][j - 1][k]).max(dp[i][j][k - 1]);
                }
            }
        }
    }
    dp[la][lb][lc]
}

fn main() {
    println!(
        "{}",
        lcs3("epidemiologist", "refrigeration", "supercalifragilisticexpialodocious")
    ); // 5
}

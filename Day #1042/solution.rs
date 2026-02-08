// LCS of three strings via 3D DP dp[i][j][k]. Time O(n1*n2*n3), Space O(n1*n2*n3).
fn lcs3(a: &str, b: &str, c: &str) -> usize {
    let a: Vec<u8> = a.bytes().collect();
    let b: Vec<u8> = b.bytes().collect();
    let c: Vec<u8> = c.bytes().collect();
    let (n1, n2, n3) = (a.len(), b.len(), c.len());
    let mut dp = vec![vec![vec![0usize; n3 + 1]; n2 + 1]; n1 + 1];
    for i in 1..=n1 {
        for j in 1..=n2 {
            for k in 1..=n3 {
                if a[i - 1] == b[j - 1] && b[j - 1] == c[k - 1] {
                    dp[i][j][k] = dp[i - 1][j - 1][k - 1] + 1;
                } else {
                    dp[i][j][k] = dp[i - 1][j][k]
                        .max(dp[i][j - 1][k])
                        .max(dp[i][j][k - 1]);
                }
            }
        }
    }
    dp[n1][n2][n3]
}

fn main() {
    println!(
        "{}",
        lcs3(
            "epidemiologist",
            "refrigeration",
            "supercalifragilisticexpialodocious"
        )
    );
}

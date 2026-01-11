// Min palindrome partition via DP with palindrome table + reconstruction.
// Time O(n^2), Space O(n^2).

fn min_palindrome_partition(s: &str) -> Vec<String> {
    let chars: Vec<char> = s.chars().collect();
    let n = chars.len();
    if n == 0 {
        return vec![];
    }
    let mut pal = vec![vec![false; n]; n];
    for i in (0..n).rev() {
        for j in i..n {
            if chars[i] == chars[j] && (j - i < 2 || pal[i + 1][j - 1]) {
                pal[i][j] = true;
            }
        }
    }

    const INF: i32 = 1 << 30;
    let mut dp = vec![INF; n + 1];
    let mut cut = vec![-1i32; n + 1];
    dp[0] = 0;
    for i in 1..=n {
        for j in 0..i {
            if pal[j][i - 1] && dp[j] + 1 < dp[i] {
                dp[i] = dp[j] + 1;
                cut[i] = j as i32;
            }
        }
    }

    let mut res: Vec<String> = Vec::new();
    let mut i = n as i32;
    while i > 0 {
        let c = cut[i as usize];
        res.push(chars[c as usize..i as usize].iter().collect());
        i = c;
    }
    res.reverse();
    res
}

fn main() {
    println!("{:?}", min_palindrome_partition("racecarannakayak"));
    println!("{:?}", min_palindrome_partition("abc"));
}

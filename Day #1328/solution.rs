// Day 1328: Split a string into the fewest palindromic substrings.
// DP: isPal[i][j] in O(n^2); dp[i]=min pieces for prefix i, with parent pointers to rebuild. O(n^2) time/space.

fn min_palindrome_partition(s: &str) -> Vec<String> {
    let b = s.as_bytes();
    let n = b.len();
    if n == 0 {
        return vec![];
    }
    let mut pal = vec![vec![false; n]; n];
    for i in (0..n).rev() {
        for j in i..n {
            pal[i][j] = b[i] == b[j] && (j - i < 2 || pal[i + 1][j - 1]);
        }
    }

    const INF: i32 = 1 << 30;
    let mut dp = vec![INF; n + 1];
    let mut prev = vec![0usize; n + 1];
    dp[0] = 0;
    for i in 1..=n {
        for j in 0..i {
            if pal[j][i - 1] && dp[j] + 1 < dp[i] {
                dp[i] = dp[j] + 1;
                prev[i] = j;
            }
        }
    }

    let mut res = Vec::new();
    let mut i = n;
    while i > 0 {
        res.push(s[prev[i]..i].to_string());
        i = prev[i];
    }
    res.reverse();
    res
}

fn main() {
    println!("{:?}", min_palindrome_partition("racecarannakayak")); // ["racecar", "anna", "kayak"]
    println!("{:?}", min_palindrome_partition("abc"));               // ["a", "b", "c"]
}

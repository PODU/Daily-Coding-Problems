// Day 529: Split a string into the fewest palindromic substrings.
// DP: is_pal[i][j] in O(n^2); dp[i] = min cuts for prefix i with parent pointers
// to reconstruct one optimal partition. Time O(n^2), space O(n^2).

fn min_palindrome_partition(s: &str) -> Vec<String> {
    let b = s.as_bytes();
    let n = b.len();
    let mut pal = vec![vec![false; n]; n];
    for i in (0..n).rev() {
        for j in i..n {
            pal[i][j] = b[i] == b[j] && (j - i < 2 || pal[i + 1][j - 1]);
        }
    }

    const INF: i32 = 1 << 30;
    let mut dp = vec![INF; n + 1];
    let mut prev = vec![-1i32; n + 1];
    dp[0] = 0;
    for j in 1..=n {
        for i in 0..j {
            if pal[i][j - 1] && dp[i] + 1 < dp[j] {
                dp[j] = dp[i] + 1;
                prev[j] = i as i32;
            }
        }
    }

    let mut parts = Vec::new();
    let mut j = n as i32;
    while j > 0 {
        let i = prev[j as usize];
        parts.push(s[i as usize..j as usize].to_string());
        j = i;
    }
    parts.reverse();
    parts
}

fn main() {
    let s = "racecarannakayak";
    let parts = min_palindrome_partition(s);
    let quoted: Vec<String> = parts.iter().map(|p| format!("\"{}\"", p)).collect();
    println!("[{}]", quoted.join(", ")); // expected: ["racecar", "anna", "kayak"]
}

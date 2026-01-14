// Palindrome by deleting at most k chars: min deletions = n - LPS(s).
// LPS via interval DP (space-optimized to O(n)). Time O(n^2), Space O(n).
fn can_make_palindrome(s: &str, k: usize) -> bool {
    let b = s.as_bytes();
    let n = b.len();
    if n == 0 {
        return 0 <= k;
    }
    let mut prev = vec![0usize; n];
    let mut cur = vec![0usize; n];
    for i in (0..n).rev() {
        cur = vec![0usize; n];
        cur[i] = 1;
        for j in (i + 1)..n {
            if b[i] == b[j] {
                cur[j] = prev[j - 1] + 2;
            } else {
                cur[j] = prev[j].max(cur[j - 1]);
            }
        }
        prev = cur.clone();
    }
    let lps = cur[n - 1];
    (n - lps) <= k
}

fn main() {
    println!("{}", if can_make_palindrome("waterrfetawx", 2) { "True" } else { "False" });
}

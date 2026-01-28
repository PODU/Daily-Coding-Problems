// Day 973: Count ways to decode a digit string (a=1..z=26).
// Approach: DP, dp[i]=dp[i-1] if 1-digit valid + dp[i-2] if 2-digit valid. Time O(n), Space O(1).

fn num_decodings(s: &str) -> i64 {
    let b = s.as_bytes();
    if b.is_empty() || b[0] == b'0' {
        return 0;
    }
    let (mut prev2, mut prev1) = (1i64, 1i64);
    for i in 1..b.len() {
        let mut cur = 0i64;
        if b[i] != b'0' {
            cur += prev1;
        }
        let two = (b[i - 1] - b'0') as i32 * 10 + (b[i] - b'0') as i32;
        if two >= 10 && two <= 26 {
            cur += prev2;
        }
        prev2 = prev1;
        prev1 = cur;
    }
    prev1
}

fn main() {
    println!("{}", num_decodings("111")); // 3
}

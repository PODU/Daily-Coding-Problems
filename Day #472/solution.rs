// Count decodings (a=1..z=26) with bottom-up DP keeping only two running states.
// Time: O(n), Space: O(1).

fn num_decodings(s: &str) -> i64 {
    let b = s.as_bytes();
    if b.is_empty() || b[0] == b'0' {
        return 0;
    }
    let mut prev2: i64 = 1; // ways up to i-2
    let mut prev1: i64 = 1; // ways up to i-1
    for i in 1..b.len() {
        let mut cur = 0;
        if b[i] != b'0' {
            cur += prev1;
        }
        let two = (b[i - 1] - b'0') as i64 * 10 + (b[i] - b'0') as i64;
        if two >= 10 && two <= 26 {
            cur += prev2;
        }
        prev2 = prev1;
        prev1 = cur;
    }
    prev1
}

fn main() {
    let msg = "111";
    println!("{}", num_decodings(msg));
}

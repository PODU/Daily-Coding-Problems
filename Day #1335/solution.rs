// Day 1335: Count decodings of a digit string with a=1..z=26.
// DP: ways[i] += ways[i-1] if digit valid, += ways[i-2] if two-digit 10..26 valid. O(n) time, O(1) space.

fn num_decodings(s: &str) -> u64 {
    let b = s.as_bytes();
    let n = b.len();
    if n == 0 {
        return 0;
    }
    let mut prev2: u64 = 1;
    let mut prev1: u64 = if b[0] != b'0' { 1 } else { 0 };
    for i in 1..n {
        let mut cur = 0;
        if b[i] != b'0' {
            cur += prev1;
        }
        let two = (b[i - 1] - b'0') as i32 * 10 + (b[i] - b'0') as i32;
        if (10..=26).contains(&two) {
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

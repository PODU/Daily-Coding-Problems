// Decode ways: DP rolling two states. Add prev1 if current digit valid, prev2 if last two in 10..26.
// Time: O(n), Space: O(1).

fn num_decodings(s: &str) -> i32 {
    let b = s.as_bytes();
    if b.is_empty() || b[0] == b'0' {
        return 0;
    }
    let (mut prev2, mut prev1) = (1, 1);
    for i in 1..b.len() {
        let mut cur = 0;
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
    println!("{}", num_decodings("111"));
}

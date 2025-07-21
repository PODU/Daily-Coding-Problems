// Decode ways: DP where ways[i] depends on 1-digit and valid 2-digit suffixes.
// Time: O(n), Space: O(1).
fn num_decodings(s: &str) -> u64 {
    let b = s.as_bytes();
    if b.is_empty() || b[0] == b'0' {
        return 0;
    }
    let (mut prev2, mut prev1) = (1u64, 1u64);
    for i in 1..b.len() {
        let mut cur = 0;
        if b[i] != b'0' {
            cur += prev1;
        }
        let two = (b[i - 1] - b'0') as u32 * 10 + (b[i] - b'0') as u32;
        if (10..=26).contains(&two) {
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

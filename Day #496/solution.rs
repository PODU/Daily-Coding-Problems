// Day 496: Total set bits across 1..N.
// For each bit position, count how many numbers in [0,N] have it set using the
// periodic pattern. O(log N) time, O(1) space.
fn count_set_bits(n: i64) -> i64 {
    let mut total = 0i64;
    let mut bit = 1i64;
    while bit <= n {
        let full = n + 1;     // count of integers in [0, n]
        let cycle = bit << 1; // period for this bit
        total += (full / cycle) * bit;
        let rem = full % cycle;
        total += std::cmp::max(0, rem - bit);
        bit <<= 1;
    }
    total
}

fn main() {
    println!("{}", count_set_bits(5));  // 7  (1+1+2+1+2)
    println!("{}", count_set_bits(16)); // 33
}

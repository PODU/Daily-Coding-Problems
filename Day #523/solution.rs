// a+b = (a^b) + 2*(a&b). So c=(M-N)/2 must be a valid carry disjoint from N.
// Ordered pairs = 2^popcount(N), minus the two zero-cases when c==0. O(log M).
fn count_pairs(m: u64, n: u64) -> i64 {
    if m < n || (m - n) & 1 == 1 {
        return 0;
    }
    let c = (m - n) / 2;
    if c & n != 0 {
        return 0;
    }
    let mut ways = 1i64 << n.count_ones();
    if c == 0 {
        ways -= 2;
    }
    ways.max(0)
}

fn main() {
    println!("{}", count_pairs(10, 4)); // 2
}

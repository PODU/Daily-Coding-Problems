// Day 1144: count positive pairs (a,b) with a+b=M, a^b=N.
// a+b = (a^b) + 2*(a&b) => and=(M-N)/2; valid iff M>=N, even, and&N==0.
// Ordered pairs = 2^popcount(N), minus 2 if and==0 (excludes a=0/b=0). O(1).
fn count_pairs(m: i64, n: i64) -> i64 {
    if m < n || (m - n) & 1 == 1 {
        return 0;
    }
    let a_and = (m - n) / 2;
    if a_and & n != 0 {
        return 0;
    }
    let mut cnt = 1i64 << (n.count_ones());
    if a_and == 0 {
        cnt -= 2;
    }
    cnt.max(0)
}

fn main() {
    println!("{}", count_pairs(10, 4)); // 2 -> (7,3) and (3,7)
}

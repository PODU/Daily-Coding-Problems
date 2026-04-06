// Bitwise AND of all integers in [M, N] = common binary prefix of M and N.
// Shift both right until equal, then shift back. Time O(log N), Space O(1).

fn range_and(mut m: i64, mut n: i64) -> i64 {
    let mut shift = 0;
    while m < n {
        m >>= 1;
        n >>= 1;
        shift += 1;
    }
    m << shift
}

fn main() {
    println!("{}", range_and(5, 7)); // 4  (5 & 6 & 7)
}

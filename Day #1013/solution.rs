// Bitwise AND of all ints in [M,N] = common binary prefix of M and N.
// Shift both right until equal, then shift back. Time: O(log N), Space: O(1).

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
    println!("AND(5, 7) = {}", range_and(5, 7)); // 4
    println!("AND(12, 15) = {}", range_and(12, 15)); // 12
}

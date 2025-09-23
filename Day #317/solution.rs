// Bitwise AND of all integers in [M, N] = common binary prefix.
// Shift both right until equal, then shift back. Time O(log N), Space O(1).

fn range_and(mut m: u64, mut n: u64) -> u64 {
    let mut shift = 0;
    while m != n {
        m >>= 1;
        n >>= 1;
        shift += 1;
    }
    m << shift
}

fn main() {
    let (m, n) = (5u64, 7u64);
    println!("Bitwise AND of [{}, {}] = {}", m, n, range_and(m, n));
}

// Bitwise AND of range [M,N] = common binary prefix; shift both right until equal, then back. O(log N) time, O(1) space.
fn range_bitwise_and(mut m: u32, mut n: u32) -> u32 {
    let mut shift = 0;
    while m < n {
        m >>= 1;
        n >>= 1;
        shift += 1;
    }
    m << shift
}

fn main() {
    println!("{}", range_bitwise_and(5, 7));
}

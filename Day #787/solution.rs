// Next bigger integer with same popcount via bit-hack (Gosper's hack). O(1) time, O(1) space.
fn next_same_popcount(n: u32) -> u32 {
    if n == 0 {
        return 0;
    }
    let c = n & n.wrapping_neg(); // lowest set bit
    let r = n + c;                // ripple carry
    let ones = ((n ^ r) >> 2) / c; // shifted-in ones
    r | ones
}

fn main() {
    println!("{}", next_same_popcount(6)); // 0110 -> 1001 = 9
}

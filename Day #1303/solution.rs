// Day 1303: Next larger integer with the same number of set bits (snoob).
// Bit-hack: add lowest set bit, then re-insert the moved bits at the bottom. O(1) time.
fn next_same_bits(n: u64) -> u64 {
    let c = n & n.wrapping_neg();   // lowest set bit
    let r = n + c;                  // ripple carry
    let ones = ((n ^ r) >> 2) / c;  // moved bits, shifted down
    r | ones
}

fn main() {
    println!("{}", next_same_bits(6)); // 9
}

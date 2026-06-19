// Next bigger integer with the same number of set bits (Gosper's hack). O(1).

fn next_same_popcount(n: u32) -> u32 {
    let c = n & n.wrapping_neg();        // lowest set bit
    let r = n.wrapping_add(c);           // ripple the carry
    r | (((n ^ r) >> 2) / c)             // restore trailing ones
}

fn main() {
    println!("{}", next_same_popcount(6)); // 6 (0110) -> 9 (1001)
}

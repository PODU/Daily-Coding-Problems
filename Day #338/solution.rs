// Next higher integer with same number of set bits (snoob bit-twiddle).
// O(1) time, O(1) space.

fn next_same_bits(n: u32) -> u32 {
    let smallest = n & n.wrapping_neg(); // n & -n
    let ripple = n + smallest;
    let mut ones = n ^ ripple;
    ones = (ones >> 2) / smallest;
    ripple | ones
}

fn main() {
    println!("{}", next_same_bits(6)); // 9
}

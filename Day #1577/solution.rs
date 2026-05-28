// Day 1577: Smallest sparse number (no adjacent 1 bits) >= N.
// Repeatedly fix the lowest pair of adjacent ones by clearing low bits and carrying.
// Time: O((log N)^2); Space: O(1).

fn smallest_sparse(mut n: u64) -> u64 {
    while n & (n >> 1) != 0 {
        let mut i = 0u32;
        while !(((n >> i) & 1) != 0 && ((n >> (i + 1)) & 1) != 0) {
            i += 1;
        }
        let mask = (1u64 << (i + 2)) - 1;
        n = (n & !mask) + (1u64 << (i + 2));
    }
    n
}

fn main() {
    println!("{}", smallest_sparse(21)); // 21
}

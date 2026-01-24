// Day 947: smallest sparse number (no two adjacent set bits) >= N.
// Repeatedly fix the lowest adjacent-1 pair by rounding up. Time O(log N), Space O(1).

fn smallest_sparse(mut n: u64) -> u64 {
    while n & (n >> 1) != 0 {
        let m = n & (n >> 1);
        let p = m.trailing_zeros();
        let step = 1u64 << (p + 1);
        n = (n + step) & !(step - 1);
    }
    n
}

fn main() {
    println!("{}", smallest_sparse(21)); // 21
    println!("{}", smallest_sparse(22)); // 32
}

// Day 1336: Count distinct N-Queens arrangements.
// Backtracking with column/diagonal bitmasks. Time: O(N!) worst, Space: O(N).
fn count(n: u32, row: u32, cols: u32, d1: u32, d2: u32) -> u64 {
    if row == n {
        return 1;
    }
    let mut total = 0u64;
    let mut avail = ((1u32 << n) - 1) & !(cols | d1 | d2);
    while avail != 0 {
        let bit = avail & avail.wrapping_neg();
        avail -= bit;
        total += count(n, row + 1, cols | bit, (d1 | bit) << 1, (d2 | bit) >> 1);
    }
    total
}

fn total_n_queens(n: u32) -> u64 {
    count(n, 0, 0, 0, 0)
}

fn main() {
    println!("N=1 -> {}", total_n_queens(1));
    println!("N=4 -> {}", total_n_queens(4));
    println!("N=8 -> {}", total_n_queens(8));
}

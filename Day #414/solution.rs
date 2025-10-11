// Day 414: Count N-Queens arrangements via bitmask backtracking.
// Track used columns/diagonals as bitmasks. Time O(N!)-ish, Space O(N).
fn solve(n: u32, row: u32, cols: u32, diag1: u32, diag2: u32) -> u64 {
    if row == n {
        return 1;
    }
    let mut count = 0;
    let mut avail = ((1u32 << n) - 1) & !(cols | diag1 | diag2);
    while avail != 0 {
        let bit = avail & avail.wrapping_neg();
        avail -= bit;
        count += solve(n, row + 1, cols | bit, (diag1 | bit) << 1, (diag2 | bit) >> 1);
    }
    count
}

fn count_n_queens(n: u32) -> u64 {
    solve(n, 0, 0, 0, 0)
}

fn main() {
    for n in 1..=8 {
        println!("N={}: {}", n, count_n_queens(n));
    }
    println!("{}", count_n_queens(8)); // 92
}

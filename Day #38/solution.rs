// N-Queens count via bitmask backtracking (columns + two diagonals). O(N!) worst, O(N) space.
fn solve(cols: i32, diag1: i32, diag2: i32, full: i32) -> i32 {
    if cols == full {
        return 1;
    }
    let mut count = 0;
    let mut avail = !(cols | diag1 | diag2) & full;
    while avail != 0 {
        let bit = avail & (-avail);
        avail -= bit;
        count += solve(cols | bit, (diag1 | bit) << 1, (diag2 | bit) >> 1, full);
    }
    count
}

fn count_n_queens(n: u32) -> i32 {
    solve(0, 0, 0, (1 << n) - 1)
}

fn main() {
    println!("{}", count_n_queens(8));
}

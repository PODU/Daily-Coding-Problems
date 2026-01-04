// Day 855: count N-Queens solutions via backtracking with bitmasks for column/diagonals.
// O(N!) worst case, O(N) space. Bitmask makes conflict checks O(1).
fn count_n_queens(n: u32) -> u64 {
    let full: u32 = (1u32 << n) - 1;
    fn solve(cols: u32, diag1: u32, diag2: u32, full: u32, count: &mut u64) {
        if cols == full {
            *count += 1;
            return;
        }
        let mut avail = full & !(cols | diag1 | diag2);
        while avail != 0 {
            let p = avail & avail.wrapping_neg();
            avail -= p;
            solve(cols | p, ((diag1 | p) << 1) & full, (diag2 | p) >> 1, full, count);
        }
    }
    let mut count = 0u64;
    solve(0, 0, 0, full, &mut count);
    count
}

fn main() {
    for n in 1..=8 {
        println!("N={}: {}", n, count_n_queens(n)); // N=8: 92
    }
}

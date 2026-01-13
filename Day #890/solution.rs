// Lattice paths in N x M grid = C(N+M-2, N-1), computed iteratively.
// Time: O(min(N,M)), Space: O(1).
fn paths(n: u64, m: u64) -> u64 {
    let total = n + m - 2;
    let k = std::cmp::min(n - 1, m - 1);
    let mut res: u64 = 1;
    for i in 1..=k {
        res = res * (total - k + i) / i;
    }
    res
}

fn main() {
    println!("{}", paths(2, 2));
    println!("{}", paths(5, 5));
}

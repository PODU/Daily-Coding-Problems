// Day 312: Tilings of a 2xN board with dominoes & L-trominoes.
// DP recurrence f(n) = 2*f(n-1) + f(n-3). O(N) time.
fn tilings(n: usize) -> i64 {
    if n == 0 || n == 1 { return 1; }
    if n == 2 { return 2; }
    let mut f = vec![0i64; n + 1];
    f[0] = 1; f[1] = 1; f[2] = 2;
    for i in 3..=n { f[i] = 2 * f[i - 1] + f[i - 3]; }
    f[n]
}

fn main() {
    println!("{}", tilings(4)); // 11
}

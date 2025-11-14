// Day 598: Expected number of rounds flipping n coins (remove tails) until one remains.
// DP: f(n) = (1 + 2^-n * sum_{k<n} C(n,k) f(k)) / (1 - 2^-n). Time O(n^2), Space O(n^2).

fn expected_rounds(n: usize) -> f64 {
    if n <= 1 {
        return 0.0;
    }
    let mut c = vec![vec![0.0f64; n + 1]; n + 1];
    for i in 0..=n {
        c[i][0] = 1.0;
        for j in 1..=i {
            c[i][j] = c[i - 1][j - 1] + c[i - 1][j];
        }
    }
    let mut f = vec![0.0f64; n + 1];
    for m in 2..=n {
        let half = 0.5f64.powi(m as i32);
        let mut s = 0.0;
        for k in 0..m {
            s += c[m][k] * f[k];
        }
        f[m] = (1.0 + half * s) / (1.0 - half);
    }
    f[n]
}

fn main() {
    let n = 4;
    println!("{:.4}", expected_rounds(n)); // ~2.0571
}

// Expected rounds until one coin remains. Each round a coin survives w.p. 1/2.
// DP recurrence: E(n)*(2^n-1) = 2^n + sum_{k=2..n-1} C(n,k) E(k); E(0)=E(1)=0. O(n^2).
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
    let mut e = vec![0.0f64; n + 1];
    for m in 2..=n {
        let pm = 2.0f64.powi(m as i32);
        let mut sum = pm;
        for k in 2..=m - 1 {
            sum += c[m][k] * e[k];
        }
        e[m] = sum / (pm - 1.0);
    }
    e[n]
}

fn main() {
    let n = 4;
    println!("{}", expected_rounds(n)); // ~2.05714 rounds
}

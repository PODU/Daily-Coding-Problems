// Day 124: Expected flipping rounds until one coin remains.
// DP: E[n](1-2^-n) = 1 + sum_{k<n} P(k survive)*E[k]. O(n^2) time, O(n) space.
fn expected_rounds(n: usize) -> f64 {
    let mut e = vec![0.0f64; n + 1];
    for m in 2..=n {
        let mut p = 0.5f64.powi(m as i32); // p_0
        let mut s = 0.0;
        for k in 0..m {
            s += p * e[k];
            p = p * (m - k) as f64 / (k + 1) as f64;
        }
        let pn = 0.5f64.powi(m as i32);
        e[m] = (1.0 + s) / (1.0 - pn);
    }
    if n < 1 {
        0.0
    } else {
        e[n]
    }
}

fn main() {
    for &n in &[1usize, 2, 3, 4, 5, 10] {
        println!("n={:<2} expected rounds = {:.6}", n, expected_rounds(n));
    }
}

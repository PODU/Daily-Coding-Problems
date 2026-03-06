// E[T] = sum_t (1 - (1-p)^n - n*p*(1-p)^(n-1)), p=2^-t (P(>1 coin alive after t rounds)). Sum until negligible. O(iterations) time, O(1) space.
fn expected_rounds(n: i32) -> f64 {
    let mut total = 0.0;
    let nf = n as f64;
    for t in 0..1000 {
        let p = 2.0_f64.powi(-t);
        let q = 1.0 - p;
        let term = 1.0 - q.powf(nf) - nf * p * q.powf(nf - 1.0);
        total += term;
        if t > 0 && term < 1e-15 {
            break;
        }
    }
    total
}

fn main() {
    let n = 4;
    println!("Expected rounds for n={}: {:.4}", n, expected_rounds(n));
}

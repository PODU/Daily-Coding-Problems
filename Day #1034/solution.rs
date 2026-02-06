// Day 1034: Expected rounds until one of n fair coins remains.
// Model: expected max of n iid Geometric(1/2) lifetimes; E = sum_{m>=0}(1-(1-2^-m)^n). O(iter).
fn expected_rounds(n: u32) -> f64 {
    let mut e = 0.0f64;
    let mut p = 1.0f64; // p = 2^-m
    for m in 0..100000 {
        let term = 1.0 - (1.0 - p).powi(n as i32);
        if term < 1e-12 && m > 0 {
            break;
        }
        e += term;
        p *= 0.5;
    }
    e
}

fn main() {
    let n = 4;
    println!("n={} -> expected rounds: {:.4}", n, expected_rounds(n));
}

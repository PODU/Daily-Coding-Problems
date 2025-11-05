// Estimate Pi via Monte Carlo: sample random points in unit square, fraction inside
// quarter circle ~ pi/4. O(S) for S samples. Deterministic LCG seed for reproducibility.
fn estimate_pi(samples: u64) -> f64 {
    let mut seed: u64 = 12345;
    let mut rand = || {
        // xorshift64 for a fast deterministic stream
        seed ^= seed << 13;
        seed ^= seed >> 7;
        seed ^= seed << 17;
        (seed >> 11) as f64 / (1u64 << 53) as f64
    };
    let mut inside: u64 = 0;
    for _ in 0..samples {
        let x = rand();
        let y = rand();
        if x * x + y * y <= 1.0 {
            inside += 1;
        }
    }
    4.0 * inside as f64 / samples as f64
}

fn main() {
    println!("{:.3}", estimate_pi(20_000_000)); // ~3.142
}

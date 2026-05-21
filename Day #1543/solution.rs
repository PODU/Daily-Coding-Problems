// Estimate pi via Monte Carlo: fraction of random points in [0,1]^2 inside the
// unit quarter-circle approximates pi/4. Time O(samples), Space O(1).
struct Lcg {
    state: u64,
}
impl Lcg {
    fn new(seed: u64) -> Self {
        Lcg { state: seed }
    }
    fn next_f64(&mut self) -> f64 {
        self.state = self
            .state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        ((self.state >> 11) as f64) / ((1u64 << 53) as f64)
    }
}

fn estimate_pi(samples: u64, seed: u64) -> f64 {
    let mut rng = Lcg::new(seed);
    let mut inside = 0u64;
    for _ in 0..samples {
        let x = rng.next_f64();
        let y = rng.next_f64();
        if x * x + y * y <= 1.0 {
            inside += 1;
        }
    }
    4.0 * inside as f64 / samples as f64
}

fn main() {
    let pi = estimate_pi(10_000_000, 42);
    println!("{:.3}", pi);
}

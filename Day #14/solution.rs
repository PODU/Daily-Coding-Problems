// Estimate pi via Monte Carlo: fraction of random points in unit circle * 4.
// Time: O(samples), Space: O(1). Seeded xorshift for reproducible 3-decimal output.
struct Rng {
    state: u64,
}

impl Rng {
    fn new(seed: u64) -> Self {
        Rng { state: seed }
    }
    fn next_f64(&mut self) -> f64 {
        // xorshift64
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.state = x;
        (x >> 11) as f64 / (1u64 << 53) as f64
    }
}

fn estimate_pi(samples: u64) -> f64 {
    let mut rng = Rng::new(88172645463325252);
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
    println!("{:.3}", estimate_pi(10_000_000));
}

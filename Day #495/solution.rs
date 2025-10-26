// Day 495: Reservoir sampling (size 1) from a stream of unknown length.
// For the i-th element (1-indexed) keep it with probability 1/i. Uses O(1) memory.
// Time: O(n) per pass, Space: O(1).
use std::collections::HashMap;

// Processes any iterator without storing it; rng yields uniform doubles in [0,1).
fn reservoir_sample<I: Iterator<Item = i32>>(stream: I, rng: &mut Rng) -> i32 {
    let mut chosen = 0;
    let mut count = 0u64;
    for x in stream {
        count += 1;
        if rng.next_f64() < 1.0 / count as f64 {
            chosen = x;
        }
    }
    chosen
}

// Simple deterministic PRNG (xorshift64) for a reproducible demo.
struct Rng(u64);
impl Rng {
    fn next_f64(&mut self) -> f64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        (x >> 11) as f64 / (1u64 << 53) as f64
    }
}

fn main() {
    let mut rng = Rng(42);
    const TRIALS: usize = 100000;
    let mut counts: HashMap<i32, usize> = HashMap::new();
    for _ in 0..TRIALS {
        let v = reservoir_sample(1..=10, &mut rng);
        *counts.entry(v).or_insert(0) += 1;
    }

    println!("Empirical selection frequency per element (~0.100 each):");
    for v in 1..=10 {
        println!("{}: {:.3}", v, *counts.get(&v).unwrap_or(&0) as f64 / TRIALS as f64);
    }
    println!("One sampled value: {}", reservoir_sample(1..=10, &mut rng));
}

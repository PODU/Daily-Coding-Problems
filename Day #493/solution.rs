// Day 493: Sample from a discrete distribution.
// Precompute cumulative probabilities; binary-search a uniform r in [0,1).
// Time: O(n) preprocessing, O(log n) per sample. Space: O(n).
use std::collections::HashMap;

struct DiscreteSampler {
    numbers: Vec<i32>,
    cdf: Vec<f64>,
}

impl DiscreteSampler {
    fn new(numbers: Vec<i32>, probs: &[f64]) -> Self {
        let mut cdf = Vec::with_capacity(probs.len());
        let mut acc = 0.0;
        for &p in probs {
            acc += p;
            cdf.push(acc);
        }
        DiscreteSampler { numbers, cdf }
    }

    fn sample(&self, r: f64) -> i32 {
        // first index whose cdf > r
        let (mut lo, mut hi) = (0usize, self.cdf.len() - 1);
        while lo < hi {
            let mid = (lo + hi) / 2;
            if self.cdf[mid] > r {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }
        self.numbers[lo]
    }
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
    let numbers = vec![1, 2, 3, 4];
    let probs = [0.1, 0.5, 0.2, 0.2];
    let sampler = DiscreteSampler::new(numbers.clone(), &probs);

    let mut rng = Rng(42);
    const N: usize = 100000;
    let mut counts: HashMap<i32, usize> = HashMap::new();
    for _ in 0..N {
        *counts.entry(sampler.sample(rng.next_f64())).or_insert(0) += 1;
    }
    for n in &numbers {
        println!("{}: {:.3}", n, *counts.get(n).unwrap_or(&0) as f64 / N as f64);
    }
}

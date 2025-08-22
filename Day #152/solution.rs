// Day 152: Weighted random sampling. Build cumulative distribution, draw u in
// [0,1) and binary-search the bucket. Preprocess O(n), per-sample O(log n).
use std::collections::HashMap;

// Small deterministic xorshift RNG producing f64 in [0,1).
struct Rng {
    state: u64,
}
impl Rng {
    fn new(seed: u64) -> Self {
        Rng { state: seed.max(1) }
    }
    fn next_f64(&mut self) -> f64 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.state = x;
        (x >> 11) as f64 / (1u64 << 53) as f64
    }
}

struct WeightedSampler {
    nums: Vec<i32>,
    cdf: Vec<f64>,
    rng: Rng,
}
impl WeightedSampler {
    fn new(nums: Vec<i32>, probs: Vec<f64>, seed: u64) -> Self {
        let mut cdf = Vec::with_capacity(probs.len());
        let mut acc = 0.0;
        for p in &probs {
            acc += *p;
            cdf.push(acc);
        }
        WeightedSampler { nums, cdf, rng: Rng::new(seed) }
    }
    fn sample(&mut self) -> i32 {
        let u = self.rng.next_f64();
        let idx = match self.cdf.binary_search_by(|x| x.partial_cmp(&u).unwrap()) {
            Ok(i) => i,
            Err(i) => i,
        };
        let idx = idx.min(self.nums.len() - 1);
        self.nums[idx]
    }
}

fn main() {
    let nums = vec![1, 2, 3, 4];
    let probs = vec![0.1, 0.5, 0.2, 0.2];
    let mut s = WeightedSampler::new(nums.clone(), probs, 42);
    let n = 1_000_000;
    let mut counts: HashMap<i32, i32> = HashMap::new();
    for _ in 0..n {
        *counts.entry(s.sample()).or_insert(0) += 1;
    }
    for num in &nums {
        let c = *counts.get(num).unwrap_or(&0);
        println!("{}: {:.2}%", num, 100.0 * c as f64 / n as f64);
    }
}

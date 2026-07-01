// Day 1746: Weighted random sampler.
// Approach: prefix-sum (CDF) of probabilities + binary search on a uniform U[0,1).
// Build O(n), sample O(log n) time, O(n) space.
// Uses a small self-contained xorshift PRNG (no external crates).

struct Rng {
    state: u64,
}
impl Rng {
    fn new(seed: u64) -> Self {
        Rng { state: seed | 1 }
    }
    fn next_f64(&mut self) -> f64 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.state = x;
        // top 53 bits -> [0,1)
        ((x >> 11) as f64) / ((1u64 << 53) as f64)
    }
}

struct WeightedSampler {
    nums: Vec<i32>,
    cdf: Vec<f64>,
    rng: Rng,
}
impl WeightedSampler {
    fn new(nums: Vec<i32>, probs: &[f64], seed: u64) -> Self {
        let mut cdf = Vec::with_capacity(probs.len());
        let mut acc = 0.0;
        for &p in probs {
            acc += p;
            cdf.push(acc);
        }
        WeightedSampler { nums, cdf, rng: Rng::new(seed) }
    }
    fn sample(&mut self) -> i32 {
        let r = self.rng.next_f64();
        let mut lo = 0usize;
        let mut hi = self.cdf.len() - 1;
        while lo < hi {
            let mid = (lo + hi) / 2;
            if self.cdf[mid] < r {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        self.nums[lo]
    }
}

fn main() {
    let numbers = vec![1, 2, 3, 4];
    let probs = [0.1, 0.5, 0.2, 0.2];
    let mut s = WeightedSampler::new(numbers.clone(), &probs, 42);

    let n = 1_000_000;
    let mut cnt = std::collections::HashMap::new();
    for _ in 0..n {
        *cnt.entry(s.sample()).or_insert(0) += 1;
    }
    println!("Observed frequencies over {} samples:", n);
    for x in &numbers {
        let c = *cnt.get(x).unwrap_or(&0);
        println!("{}: {:.1}%", x, 100.0 * c as f64 / n as f64);
    }
    println!("Expected: 1 10% of the time, 2 50% of the time, and 3 and 4 20% of the time");
}

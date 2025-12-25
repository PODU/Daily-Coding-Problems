// Day 802: Sample a number per given probability distribution.
// Build CDF prefix sums once O(n); each sample draws u~U(0,1) + binary search O(log n).
// Self-contained xorshift PRNG for a deterministic demo (no external crates).

struct Rng(u64);
impl Rng {
    fn next_f64(&mut self) -> f64 {
        // xorshift64
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        (x >> 11) as f64 / (1u64 << 53) as f64
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
        WeightedSampler { nums, cdf, rng: Rng(seed) }
    }

    fn sample(&mut self) -> i32 {
        let u = self.rng.next_f64();
        let i = self.cdf.partition_point(|&c| c < u);
        self.nums[i.min(self.nums.len() - 1)]
    }
}

fn main() {
    let numbers = vec![1, 2, 3, 4];
    let probs = [0.1, 0.5, 0.2, 0.2];
    let mut s = WeightedSampler::new(numbers.clone(), &probs, 42);
    let trials = 100_000;
    let mut count = vec![0u32; numbers.len()];
    for _ in 0..trials {
        let v = s.sample();
        count[(v - 1) as usize] += 1;
    }
    for (i, &n) in numbers.iter().enumerate() {
        println!("{}: {:.2}", n, count[i] as f64 / trials as f64);
    }
    // ~ 1:0.10  2:0.50  3:0.20  4:0.20
}

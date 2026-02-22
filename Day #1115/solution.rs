// Day 1115 - Uniform random in [0, n) excluding list l
// Approach: remap |E| excluded slots below m=n-|E| to available high slots,
// then sample once in [0, m). Time: O(|E|) prep, O(1)/sample.
// Uses a small self-contained xorshift RNG (no external deps).
use std::collections::{BTreeSet, HashMap};

struct Rng(u64);
impl Rng {
    fn next_u64(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }
    fn below(&mut self, m: usize) -> usize {
        (self.next_u64() % m as u64) as usize
    }
}

struct Sampler {
    m: usize,
    mapping: HashMap<usize, usize>,
    rng: Rng,
}

impl Sampler {
    fn new(n: usize, l: &[usize]) -> Self {
        let excluded: BTreeSet<usize> = l.iter().cloned().filter(|&x| x < n).collect();
        let m = n - excluded.len();
        let available: Vec<usize> = (m..n).filter(|v| !excluded.contains(v)).collect();
        let mut mapping = HashMap::new();
        let mut ai = 0;
        for &e in &excluded {
            if e < m {
                mapping.insert(e, available[ai]);
                ai += 1;
            }
        }
        Sampler { m, mapping, rng: Rng(0x9E3779B97F4A7C15) }
    }
    fn sample(&mut self) -> usize {
        let r = self.rng.below(self.m);
        *self.mapping.get(&r).unwrap_or(&r)
    }
}

fn main() {
    let n = 10usize;
    let l = vec![2usize, 5, 7];
    let mut s = Sampler::new(n, &l);
    let mut seen = BTreeSet::new();
    for _ in 0..2000 {
        seen.insert(s.sample());
    }
    println!("n=10, excluded=[2, 5, 7]");
    let parts: Vec<String> = seen.iter().map(|v| v.to_string()).collect();
    println!("Sampled valid numbers: [{}]", parts.join(", "));
}

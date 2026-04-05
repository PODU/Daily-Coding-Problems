// Day 1304: Uniformly sample an integer in [0, n) not in list l.
// Precompute sorted excluded; pick r in [0, n-|excl|) and map to the r-th allowed value.
// Preprocess O(m log m); each draw O(m). Uniform over all allowed values.
use std::collections::BTreeSet;

// Tiny deterministic xorshift RNG (avoids external crates).
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
    fn below(&mut self, bound: u64) -> u64 {
        self.next_u64() % bound
    }
}

struct RandExcluder {
    n: i64,
    excl: Vec<i64>,
    rng: Rng,
}

impl RandExcluder {
    fn new(n: i64, l: &[i64], seed: u64) -> Self {
        let set: BTreeSet<i64> = l.iter().cloned().filter(|&x| x >= 0 && x < n).collect();
        RandExcluder { n, excl: set.into_iter().collect(), rng: Rng(seed) }
    }
    fn next(&mut self) -> i64 {
        let count = self.n - self.excl.len() as i64;
        let mut res = self.rng.below(count as u64) as i64;
        for &e in &self.excl {
            if e <= res {
                res += 1;
            } else {
                break;
            }
        }
        res
    }
}

fn main() {
    let mut r = RandExcluder::new(5, &[1, 3], 42);
    let mut seen: BTreeSet<i64> = BTreeSet::new();
    for _ in 0..1000 {
        seen.insert(r.next());
    }
    let out: Vec<i64> = seen.into_iter().collect();
    println!("{:?}", out); // [0, 2, 4]
}

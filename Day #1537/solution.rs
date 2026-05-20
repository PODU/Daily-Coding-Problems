// Simulate a Markov chain for num_steps and count visits per state.
// Time O(num_steps * outdegree), Space O(states + transitions).
// Uses a small deterministic LCG PRNG for reproducibility.
use std::collections::BTreeMap;

struct Lcg {
    state: u64,
}
impl Lcg {
    fn new(seed: u64) -> Self {
        Lcg { state: seed }
    }
    fn next_f64(&mut self) -> f64 {
        // Numerical Recipes LCG
        self.state = self.state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        ((self.state >> 11) as f64) / ((1u64 << 53) as f64)
    }
}

fn run_chain(start: &str, num_steps: usize, transitions: &[(&str, &str, f64)], seed: u64) -> BTreeMap<String, i64> {
    let mut trans: BTreeMap<String, Vec<(String, f64)>> = BTreeMap::new();
    for (src, dst, p) in transitions {
        trans.entry(src.to_string()).or_default().push((dst.to_string(), *p));
    }
    let mut rng = Lcg::new(seed);
    let mut counts: BTreeMap<String, i64> = BTreeMap::new();
    let mut cur = start.to_string();
    for _ in 0..num_steps {
        *counts.entry(cur.clone()).or_insert(0) += 1;
        let r = rng.next_f64();
        let mut acc = 0.0;
        for (dst, p) in &trans[&cur] {
            acc += p;
            if r <= acc {
                cur = dst.clone();
                break;
            }
        }
    }
    counts
}

fn main() {
    let transitions = [
        ("a", "a", 0.9), ("a", "b", 0.075), ("a", "c", 0.025),
        ("b", "a", 0.15), ("b", "b", 0.8), ("b", "c", 0.05),
        ("c", "a", 0.25), ("c", "b", 0.25), ("c", "c", 0.5),
    ];
    let counts = run_chain("a", 5000, &transitions, 42);
    let parts: Vec<String> = counts.iter().map(|(k, v)| format!("'{}': {}", k, v)).collect();
    println!("{{ {} }}", parts.join(", "));
}

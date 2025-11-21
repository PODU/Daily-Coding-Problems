// Day 640: Simulate a Markov chain and tally state visits.
// Approach: build outgoing-edge table, draw next state by cumulative prob.
// Time: O(num_steps * avg_out_degree), Space: O(states + edges).
// Note: output is stochastic; counts approximate the README sample (sum 5000).
use std::collections::BTreeMap;
use std::collections::HashMap;

// Simple xorshift RNG for self-contained reproducibility.
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

fn run_markov(start: &str, trans: &[(&str, &str, f64)], steps: usize, seed: u64) -> BTreeMap<String, i32> {
    let mut adj: HashMap<&str, Vec<(&str, f64)>> = HashMap::new();
    for &(src, dst, p) in trans {
        adj.entry(src).or_default().push((dst, p));
    }
    let mut rng = Rng(seed);
    let mut counts: BTreeMap<String, i32> = BTreeMap::new();
    let mut cur = start;
    for _ in 0..steps {
        *counts.entry(cur.to_string()).or_insert(0) += 1;
        let r = rng.next_f64();
        let mut acc = 0.0;
        for &(dst, p) in &adj[cur] {
            acc += p;
            if r <= acc {
                cur = dst;
                break;
            }
        }
    }
    counts
}

fn main() {
    let trans = vec![
        ("a", "a", 0.9), ("a", "b", 0.075), ("a", "c", 0.025),
        ("b", "a", 0.15), ("b", "b", 0.8), ("b", "c", 0.05),
        ("c", "a", 0.25), ("c", "b", 0.25), ("c", "c", 0.5),
    ];
    let counts = run_markov("a", &trans, 5000, 42);
    let parts: Vec<String> = counts.iter().map(|(k, v)| format!("'{}': {}", k, v)).collect();
    println!("{{ {} }}", parts.join(", ")); // e.g. { 'a': 3012, 'b': 1656, 'c': 332 }
}

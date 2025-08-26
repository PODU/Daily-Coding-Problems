// Markov chain simulation: sample next state via cumulative probabilities, fixed-seed RNG.
// Time O(num_steps * avg_outdegree), Space O(states). (Exact counts depend on RNG.)
use std::collections::HashMap;

// Simple deterministic LCG PRNG producing f64 in [0,1).
struct Rng { state: u64 }
impl Rng {
    fn new(seed: u64) -> Self { Rng { state: seed } }
    fn next_f64(&mut self) -> f64 {
        // Numerical Recipes LCG constants.
        self.state = self.state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        ((self.state >> 11) as f64) / ((1u64 << 53) as f64)
    }
}

fn simulate(start: char, num_steps: usize, transitions: &[(char, char, f64)], seed: u64) -> HashMap<char, i32> {
    let mut trans: HashMap<char, Vec<(char, f64)>> = HashMap::new();
    for &(src, dst, prob) in transitions {
        trans.entry(src).or_default().push((dst, prob));
    }
    let mut rng = Rng::new(seed);
    let mut counts: HashMap<char, i32> = HashMap::new();
    let mut state = start;
    for _ in 0..num_steps {
        *counts.entry(state).or_insert(0) += 1;
        let r = rng.next_f64();
        let mut cum = 0.0;
        for &(dst, prob) in &trans[&state] {
            cum += prob;
            if r < cum { state = dst; break; }
        }
    }
    counts
}

fn main() {
    let transitions = [
        ('a','a',0.9),('a','b',0.075),('a','c',0.025),
        ('b','a',0.15),('b','b',0.8),('b','c',0.05),
        ('c','a',0.25),('c','b',0.25),('c','c',0.5),
    ];
    let c = simulate('a', 5000, &transitions, 42);
    println!("{{'a': {}, 'b': {}, 'c': {}}}",
        c.get(&'a').unwrap_or(&0), c.get(&'b').unwrap_or(&0), c.get(&'c').unwrap_or(&0));
}

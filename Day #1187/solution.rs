// Markov chain simulation: cumulative transition table, draw uniform RNG per step.
// Result is stochastic/approximate (fixed seed for reproducibility). Time O(steps), Space O(states^2).
use std::collections::BTreeMap;

// Simple seeded LCG / xorshift-style RNG producing f64 in [0,1).
struct Rng {
    state: u64,
}
impl Rng {
    fn new(seed: u64) -> Self {
        Rng { state: seed }
    }
    fn next_f64(&mut self) -> f64 {
        // xorshift64
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.state = x;
        (x >> 11) as f64 / (1u64 << 53) as f64
    }
}

fn main() {
    let transitions: Vec<(char, char, f64)> = vec![
        ('a', 'a', 0.9), ('a', 'b', 0.075), ('a', 'c', 0.025),
        ('b', 'a', 0.15), ('b', 'b', 0.8), ('b', 'c', 0.05),
        ('c', 'a', 0.25), ('c', 'b', 0.25), ('c', 'c', 0.5),
    ];

    let mut raw: BTreeMap<char, Vec<(f64, char)>> = BTreeMap::new();
    for (frm, to, prob) in &transitions {
        raw.entry(*frm).or_default().push((*prob, *to));
    }
    let mut table: BTreeMap<char, Vec<(f64, char)>> = BTreeMap::new();
    for (st, lst) in &raw {
        let mut cum = 0.0;
        let mut v = Vec::new();
        for (prob, to) in lst {
            cum += prob;
            v.push((cum, *to));
        }
        table.insert(*st, v);
    }

    let start = 'a';
    let num_steps = 5000;
    let mut counts: BTreeMap<char, i64> = BTreeMap::new();
    counts.insert('a', 0);
    counts.insert('b', 0);
    counts.insert('c', 0);

    let mut rng = Rng::new(12345);
    let mut state = start;
    for _ in 0..num_steps {
        let r = rng.next_f64();
        for (cum, to) in &table[&state] {
            if r < *cum {
                state = *to;
                break;
            }
        }
        *counts.get_mut(&state).unwrap() += 1;
    }

    println!("{{'a': {}, 'b': {}, 'c': {}}}", counts[&'a'], counts[&'b'], counts[&'c']);
}

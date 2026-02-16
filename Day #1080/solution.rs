// Markov chain simulation: seeded xorshift64 RNG (no external deps); O(steps*states) time O(states^2) space
// Counts state arrived at after each step (not initial state); total counts = num_steps = 5000
// Exact counts vary by seed; approx distribution: ~60% a, ~33% b, ~7% c

struct Rng(u64);

impl Rng {
    fn new(seed: u64) -> Self { Rng(seed) }
    fn next_f64(&mut self) -> f64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        // Use upper 53 bits for double precision
        (self.0 >> 11) as f64 / (1u64 << 53) as f64
    }
}

fn main() {
    let transitions: &[(&str, &str, f64)] = &[
        ("a", "a", 0.9),  ("a", "b", 0.075), ("a", "c", 0.025),
        ("b", "a", 0.15), ("b", "b", 0.8),   ("b", "c", 0.05),
        ("c", "a", 0.25), ("c", "b", 0.25),  ("c", "c", 0.5),
    ];

    let mut trans: std::collections::HashMap<&str, Vec<(&str, f64)>> =
        std::collections::HashMap::new();
    for &(f, t, p) in transitions {
        trans.entry(f).or_default().push((t, p));
    }

    let mut rng = Rng::new(42);
    let mut state = "a";
    let mut counts: std::collections::HashMap<&str, u32> = std::collections::HashMap::new();
    counts.insert("a", 0);
    counts.insert("b", 0);
    counts.insert("c", 0);
    let num_steps = 5000_usize;

    for _ in 0..num_steps {
        let r = rng.next_f64();
        let mut cumulative = 0.0_f64;
        for &(to, prob) in &trans[state] {
            cumulative += prob;
            if r < cumulative {
                state = to;
                break;
            }
        }
        *counts.get_mut(state).unwrap() += 1;
    }

    println!("{{ 'a': {}, 'b': {}, 'c': {} }}", counts["a"], counts["b"], counts["c"]);
}

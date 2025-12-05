// Day 695: Uniform random in [0, n-1] excluding values in list l.
// Approach: precompute the allowed values once, then pick a uniform index using a
// small self-contained xorshift PRNG. Preprocess O(n), each draw O(1).
use std::collections::HashSet;

struct RandExcluder {
    allowed: Vec<i32>,
    state: u64,
}

impl RandExcluder {
    fn new(n: i32, l: &[i32], seed: u64) -> Self {
        let bad: HashSet<i32> = l.iter().cloned().collect();
        let allowed = (0..n).filter(|x| !bad.contains(x)).collect();
        RandExcluder { allowed, state: seed }
    }
    fn next(&mut self) -> i32 {
        // xorshift64
        self.state ^= self.state << 13;
        self.state ^= self.state >> 7;
        self.state ^= self.state << 17;
        let idx = (self.state % self.allowed.len() as u64) as usize;
        self.allowed[idx]
    }
}

fn main() {
    let mut r = RandExcluder::new(10, &[2, 4, 6, 8], 42);
    print!("sample: ");
    for _ in 0..8 {
        print!("{} ", r.next());
    }
    println!("\n(all values are in [0,9] and never 2,4,6,8)");
}

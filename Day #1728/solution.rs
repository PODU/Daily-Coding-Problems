// Day 1728: Simulate a fair coin from a biased one (Von Neumann trick).
// Toss biased coin twice; (0,1)->0, (1,0)->1, else retry. Expected P(heads) ~= 0.5.
// Time: O(1) expected tosses per fair() call. Space: O(1).

// Simple seeded RNG (xorshift64) for reproducible output, no external deps.
struct Rng {
    state: u64,
}

impl Rng {
    fn new(seed: u64) -> Self {
        Rng { state: seed }
    }
    fn next_u64(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.state = x;
        x
    }
    // Returns a float in [0, 1).
    fn next_f64(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 / (1u64 << 53) as f64
    }
}

// Biased coin: returns 1 with probability p (= 0.3), else 0.
fn toss_biased(rng: &mut Rng) -> u8 {
    if rng.next_f64() < 0.3 {
        1
    } else {
        0
    }
}

// Von Neumann: extract a fair bit from the biased coin.
fn fair(rng: &mut Rng) -> u8 {
    loop {
        let a = toss_biased(rng);
        let b = toss_biased(rng);
        if a == 0 && b == 1 {
            return 0;
        }
        if a == 1 && b == 0 {
            return 1;
        }
    }
}

fn main() {
    let mut rng = Rng::new(12345);
    let n = 100000;
    let mut heads: u64 = 0;
    for _ in 0..n {
        heads += fair(&mut rng) as u64;
    }
    let ratio = heads as f64 / n as f64;
    println!("Fair coin over {} tosses, P(heads) ~= {:.2}", n, ratio);
}

// Von Neumann extractor: toss biased coin twice; (0,1)->0, (1,0)->1, else retry => 50/50. Time O(1) expected.

// Deterministic xorshift PRNG so the demo is stable without external crates.
struct Rng {
    state: u64,
}

impl Rng {
    fn next_f64(&mut self) -> f64 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.state = x;
        (x >> 11) as f64 / (1u64 << 53) as f64
    }
}

fn toss_biased(rng: &mut Rng) -> i32 {
    if rng.next_f64() < 0.3 {
        1
    } else {
        0
    }
}

fn toss_fair(rng: &mut Rng) -> i32 {
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
    let mut rng = Rng { state: 12345 };
    let trials = 100000;
    let mut ones = 0i64;
    for _ in 0..trials {
        ones += toss_fair(&mut rng) as i64;
    }
    let frac = ones as f64 / trials as f64;
    assert!(frac > 0.48 && frac < 0.52);
    println!("Fair coin ~0.5");
}

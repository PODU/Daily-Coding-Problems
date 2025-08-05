// rand5 from rand7 via rejection sampling: call rand7() until <=5. Uniform 1..5. Time O(1) expected, Space O(1).
use std::time::{SystemTime, UNIX_EPOCH};

// Simple xorshift PRNG (no external crates).
struct Rng {
    state: u64,
}

impl Rng {
    fn new(seed: u64) -> Self {
        Rng { state: seed | 1 }
    }
    fn next_u64(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.state = x;
        x
    }
    fn range(&mut self, lo: u64, hi: u64) -> u64 {
        lo + self.next_u64() % (hi - lo + 1)
    }
}

fn rand7(rng: &mut Rng) -> i32 {
    rng.range(1, 7) as i32
}

fn rand5(rng: &mut Rng) -> i32 {
    loop {
        let x = rand7(rng);
        if x <= 5 {
            return x;
        }
    }
}

fn main() {
    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64;
    let mut rng = Rng::new(seed);

    let trials = 100_000;
    let mut counts = [0i64; 6];
    for _ in 0..trials {
        let v = rand5(&mut rng);
        assert!(v >= 1 && v <= 5);
        counts[v as usize] += 1;
    }
    let expected = trials as f64 / 5.0;
    for v in 1..=5 {
        assert!((counts[v] as f64 - expected).abs() < expected * 0.1);
    }
    println!("rand5 OK");
}
